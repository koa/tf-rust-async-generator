use std::{
    borrow::BorrowMut,
    fmt::Debug,
    ops::{Deref, DerefMut},
    sync::{
        Arc,
        atomic::{AtomicBool, Ordering},
    },
    time::Duration,
};


use log::{debug, error, info, warn};
use tokio::{
    io::{self, AsyncReadExt, AsyncWriteExt, WriteHalf},
    net::{TcpStream, ToSocketAddrs},
    sync::{
        broadcast::{self, Receiver},
        Mutex,
    },
    task::AbortHandle,
};
use tokio_stream::{
    empty,
    Stream,
    StreamExt, wrappers::{BroadcastStream, errors::BroadcastStreamRecvError},
};
use tokio_util::either::Either;

use crate::{
    base58::{Base58Error, Uid},
    byte_converter::{FromByteSlice, ToBytes},
    error::TinkerforgeError,
    ip_connection::{EnumerateResponse, PacketHeader},
};

#[derive(Debug, Clone)]
pub struct AsyncIpConnection {
    inner: Arc<Mutex<InnerAsyncIpConnection>>,
}

impl AsyncIpConnection {
    pub async fn enumerate(&mut self) -> Result<impl Stream<Item=EnumerateResponse> , TinkerforgeError> {
        self.inner.borrow_mut().lock().await.enumerate().await
    }
    pub async fn disconnect_probe(&mut self) -> Result<(), TinkerforgeError> {
        self.inner.borrow_mut().lock().await.disconnect_probe().await
    }
    pub async fn get_authentication_nonce(&mut self) -> Result<[u8; 4], TinkerforgeError> {
        self.inner.borrow_mut().lock().await.get_authentication_nonce().await
    }
    pub async fn set(
        &mut self,
        uid: Uid,
        function_id: u8,
        payload: &[u8],
        timeout: Option<Duration>,
    ) -> Result<Option<PacketData>, TinkerforgeError> {
        self.inner.borrow_mut().lock().await.set(uid, function_id, payload, timeout).await
    }
    pub async fn get(
        &mut self,
        uid: Uid,
        function_id: u8,
        payload: &[u8],
        timeout: Duration,
    ) -> Result<PacketData, TinkerforgeError> {
        self.inner.borrow_mut().lock().await.get(uid, function_id, payload, timeout).await
    }
    pub async fn callback_stream(&mut self, uid: Uid, function_id: u8) -> impl Stream<Item=PacketData> {
        self.inner.borrow_mut().lock().await.callback_stream(uid, function_id).await
    }
    pub async fn new<T: ToSocketAddrs + Debug + Clone + Send + 'static>(addr: T) -> Result<Self, TinkerforgeError> {
        Ok(Self { inner: Arc::new(Mutex::new(InnerAsyncIpConnection::new(addr).await?)) })
    }
}

#[derive(Debug)]
struct InnerAsyncIpConnection {
    write_stream: WriteHalf<TcpStream>,
    receiver: Receiver<Option<PacketData>>,
    seq_num: u8,
    running: Arc<AtomicBool>,
    abort_handle: AbortHandle,
}

impl InnerAsyncIpConnection {
    pub async fn new<T: ToSocketAddrs + Clone + Debug + Send + 'static>(addr: T) -> Result<Self, TinkerforgeError> {
        let socket = TcpStream::connect(addr.clone()).await?;
        Self::enable_keepalive(&socket)?;

        let (mut rd, write_stream) = io::split(socket);
        let (enum_tx, receiver) = broadcast::channel(512);
        let running = Arc::new(AtomicBool::new(true));
        let running_clone = running.clone();
        let abort_handle = tokio::spawn(async move {
            loop {
                let mut header_buffer = Box::new([0; PacketHeader::SIZE]);
                match rd.read_exact(header_buffer.deref_mut()).await {
                    Ok(8) => {
                        let header = PacketHeader::from_le_byte_slice(header_buffer.deref());
                        let body_size = header.length as usize - PacketHeader::SIZE;
                        let mut body = vec![0; body_size].into_boxed_slice();
                        match rd.read_exact(body.deref_mut()).await {
                            Ok(l) if l == body_size => {}
                            Ok(l) => {
                                panic!("Unexpected body size: {}", l)
                            }
                            Err(e) => panic!("Error from socket: {}", e),
                        }
                        let packet_data = PacketData { header, body };
                        debug!("Received: {packet_data:?}");
                        if let Err(error) = enum_tx.send(Some(packet_data)) {
                            warn!("Cannot process packet from {addr:?}: {error}");
                            break;
                        }
                    }
                    Ok(n) => {
                        error!("Unexpected read count from {addr:?}: {}", n);
                        if let Err(error) = enum_tx.send(None) {
                            warn!("Cannot close connection on read error: {error}");
                        }
                        break;
                    }
                    Err(e) => {
                        error!("Error from socket {addr:?}: {e}");
                        if let Err(error) = enum_tx.send(None) {
                            warn!("Cannot close connection on communication error: {error}");
                        }
                        break;
                    }
                };
            }
            running_clone.store(false, Ordering::Relaxed);
            info!("Terminated receiver thread");
        })
            .abort_handle();
        Ok(Self { write_stream, abort_handle, seq_num: 1, receiver, running })
    }

    fn enable_keepalive(socket: &TcpStream) -> Result<(), TinkerforgeError> {
        let mut ka = socket2::TcpKeepalive::new();
        ka = ka.with_time(Duration::from_secs(20));
        ka = ka.with_interval(Duration::from_secs(20));
        socket2::SockRef::from(&socket).set_tcp_keepalive(&ka)?;
        Ok(())
    }
    pub async fn enumerate(&mut self) -> Result<impl Stream<Item=EnumerateResponse>, TinkerforgeError> {
        if !self.running.as_ref().load(Ordering::Relaxed) {
            return Ok(Either::Left(empty()));
        }
        let request = Request::Set { uid: Uid::zero(), function_id: 254, payload: &[] };
        let stream = BroadcastStream::new(self.receiver.resubscribe()).map_while(Self::while_some).filter_map(EnumerateResponse::extract_enumeration_packet);
        let seq = self.next_seq();
        self.send_packet(&request, seq, true).await?;
        Ok(Either::Right(stream))
    }

    pub async fn disconnect_probe(&mut self) -> Result<(), TinkerforgeError> {
        let request = Request::Set { uid: Uid::zero(), function_id: 128, payload: &[] };
        let seq = self.next_seq();
        self.send_packet(&request, seq, true).await?;
        Ok(())
    }
    async fn get_authentication_nonce(&mut self) -> Result<[u8; 4], TinkerforgeError> {
        let request = Request::Set { uid: Uid::zero(), function_id: 1, payload: &[] };
        let seq = self.next_seq();
        let stream = BroadcastStream::new(self.receiver.resubscribe()).map_while(Self::while_some).timeout(Duration::from_secs(5));
        self.send_packet(&request, seq, true).await?;
        tokio::pin!(stream);
        let option = stream.next().await;
        info!("Paket: {option:?}");
        if let Some(Ok(Ok(next_paket))) = option {
            let body = next_paket.body;
            if body.len() == 4 {
                let mut ret = [0; 4];
                ret.copy_from_slice(&body);
                Ok(ret)
            } else {
                todo!()
            }
        } else {
            todo!()
        }
    }
    pub async fn set(
        &mut self,
        uid: Uid,
        function_id: u8,
        payload: &[u8],
        timeout: Option<Duration>,
    ) -> Result<Option<PacketData>, TinkerforgeError> {
        let request = Request::Set { uid, function_id, payload };
        let seq = self.next_seq();
        if let Some(timeout) = timeout {
            let stream = BroadcastStream::new(self.receiver.resubscribe())
                .map_while(Self::while_some)
                .filter(Self::filter_response(uid, function_id, seq))
                .timeout(timeout);
            self.send_packet(&request, seq, true).await?;
            tokio::pin!(stream);
            if let Some(done) = stream.next().await {
                Ok(Some(done.map_err(|_| TinkerforgeError::NoResponseReceived)??))
            } else {
                Err(TinkerforgeError::NoResponseReceived)
            }
        } else {
            self.send_packet(&request, seq, false).await?;
            Ok(None)
        }
    }

    fn filter_response(uid: Uid, function_id: u8, seq: u8) -> impl Fn(&Result<PacketData, BroadcastStreamRecvError>) -> bool {
        move |result| {
            if let Ok(PacketData { header, .. }) = result {
                header.uid == uid && header.function_id == function_id && header.sequence_number == seq
            } else {
                false
            }
        }
    }
    pub async fn get(&mut self, uid: Uid, function_id: u8, payload: &[u8], timeout: Duration) -> Result<PacketData, TinkerforgeError> {
        let request = Request::Get { uid, function_id, payload };
        let seq = self.next_seq();
        let stream = BroadcastStream::new(self.receiver.resubscribe())
            .map_while(Self::while_some)
            .filter(Self::filter_response(uid, function_id, seq))
            .timeout(timeout);
        tokio::pin!(stream);
        self.send_packet(&request, seq, true).await?;
        Ok(stream.next().await.ok_or(TinkerforgeError::NoResponseReceived)?.map_err(|_| TinkerforgeError::NoResponseReceived)??)
    }

    fn while_some(v: Result<Option<PacketData>, BroadcastStreamRecvError>) -> Option<Result<PacketData, BroadcastStreamRecvError>> {
        match v {
            Ok(None) => None,
            Ok(Some(p)) => Some(Ok(p)),
            Err(e) => Some(Err(e)),
        }
    }
    pub async fn callback_stream(&mut self, uid: Uid, function_id: u8) -> impl Stream<Item=PacketData> {
        BroadcastStream::new(self.receiver.resubscribe())
            .map_while(move |result| match result {
                Ok(Some(p)) => {
                    let header = &p.header;

                    if header.uid == uid && header.function_id == function_id {
                        Some(Some(p))
                    } else if header.function_id == 253 {
                        if let Ok(enum_paket) = Result::<EnumerateResponse, Base58Error>::from_le_byte_slice(p.body()) {
                            if enum_paket.uid == uid {
                                // device is disconnected -> end stream
                                None
                            } else {
                                Some(None)
                            }
                        } else {
                            Some(None)
                        }
                    } else {
                        Some(None)
                    }
                }
                Ok(None) => None,
                Err(BroadcastStreamRecvError::Lagged(count)) => {
                    warn!("Slow receiver, skipped {count} Packets");
                    Some(None)
                }
            })
            .filter_map(|f| f)
    }
    async fn send_packet(&mut self, request: &Request<'_>, seq: u8, response_expected: bool) -> Result<(), TinkerforgeError> {
        let header = request.get_header(response_expected, seq);
        assert!(header.length <= 72);
        let mut result = vec![0; header.length as usize];
        header.uid.write_to_slice(&mut result[0..4]);
        result[4] = header.length;
        result[5] = header.function_id;
        result[6] = header.sequence_number << 4 | (header.response_expected as u8) << 3;
        result[7] = header.error_code << 6;
        let payload = request.get_payload();
        if !payload.is_empty() {
            result[8..].copy_from_slice(payload);
        }
        self.write_stream.write_all(&result[..]).await?;
        debug!("Sent: {request:?}");
        Ok(())
    }
    fn next_seq(&mut self) -> u8 {
        self.seq_num += 1;
        if self.seq_num > 15 {
            self.seq_num = 1;
        }
        self.seq_num
    }
}

impl Drop for InnerAsyncIpConnection {
    fn drop(&mut self) {
        self.abort_handle.abort();
    }
}

#[derive(Clone, Debug)]
pub struct PacketData {
    header: PacketHeader,
    body: Box<[u8]>,
}

impl PacketData {
    #[allow(dead_code)]
    pub fn header(&self) -> PacketHeader {
        self.header
    }
    pub fn body(&self) -> &[u8] {
        &self.body
    }
}

#[derive(Debug, Clone)]
pub enum Request<'a> {
    Set { uid: Uid, function_id: u8, payload: &'a [u8] },
    Get { uid: Uid, function_id: u8, payload: &'a [u8] },
}

impl Request<'_> {
    fn get_header(&self, response_expected: bool, sequence_number: u8) -> PacketHeader {
        match self {
            Request::Set { uid, function_id, payload } => {
                PacketHeader::with_payload(*uid, *function_id, sequence_number, response_expected, payload.len() as u8)
            }
            Request::Get { uid, function_id, payload, .. } => {
                PacketHeader::with_payload(*uid, *function_id, sequence_number, true, payload.len() as u8)
            }
        }
    }
    fn get_payload(&self) -> &[u8] {
        match self {
            Request::Set { payload, .. } => payload,
            Request::Get { payload, .. } => payload,
        }
    }
}

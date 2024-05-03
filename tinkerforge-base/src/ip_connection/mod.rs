//! The IP Connection manages the communication between the API bindings and the Brick Daemon or a WIFI/Ethernet Extension.
use std::{
    fmt::{Debug, Display, Formatter},
    str::{self, FromStr},
};

use tokio_stream::wrappers::errors::BroadcastStreamRecvError;

use crate::ip_connection::async_io::PacketData;
use crate::{
    base58::{Base58Error, Uid},
    byte_converter::{FromByteSlice, ToBytes},
};

pub mod async_io;

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub struct PacketHeader {
    uid: Uid,
    length: u8,
    function_id: u8,
    sequence_number: u8,
    response_expected: bool,
    error_code: u8,
}

impl PacketHeader {
    pub fn with_payload(
        uid: Uid,
        function_id: u8,
        sequence_number: u8,
        response_expected: bool,
        payload_len: u8,
    ) -> PacketHeader {
        PacketHeader {
            uid,
            length: PacketHeader::SIZE as u8 + payload_len,
            function_id,
            sequence_number,
            response_expected,
            error_code: 0,
        }
    }

    pub const SIZE: usize = 8;
}

impl FromByteSlice for PacketHeader {
    fn from_le_byte_slice(bytes: &[u8]) -> PacketHeader {
        PacketHeader {
            uid: Uid::from_le_byte_slice(bytes),
            length: bytes[4],
            function_id: bytes[5],
            sequence_number: (bytes[6] & 0xf0) >> 4,
            response_expected: (bytes[6] & 0x08) != 0,
            error_code: (bytes[7] & 0xc0) >> 6,
        }
    }

    fn bytes_expected() -> usize {
        8
    }
}

impl ToBytes for PacketHeader {
    fn write_to_slice(&self, target: &mut [u8]) -> usize {
        self.uid.write_to_slice(&mut target[0..4]);
        target[4] = self.length;
        target[5] = self.function_id;
        target[6] = self.sequence_number << 4 | (self.response_expected as u8) << 3;
        target[7] = self.error_code << 6;
        8
    }
}

//const MAX_PACKET_SIZE: usize = PacketHeader::SIZE + 64 + 8; //header + payload + optional data

/// Type of enumeration of a device.
#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
pub enum EnumerationType {
    /// Device is available (enumeration triggered by user: [`Enumerate`](crate::ip_connection::async_io::AsyncIpConnection::enumerate())).
    /// This enumeration type can occur multiple times for the same device.
    Available,
    /// Device is newly connected (automatically send by Brick after establishing a communication connection).
    /// This indicates that the device has potentially lost its previous configuration and needs to be reconfigured.
    Connected,
    /// Device is disconnected (only possible for USB connection). In this case only uid and enumerationType are valid.
    Disconnected,
    /// Device returned an unknown enumeration type.
    Unknown,
}

impl From<u8> for EnumerationType {
    fn from(byte: u8) -> EnumerationType {
        match byte {
            0 => EnumerationType::Available,
            1 => EnumerationType::Connected,
            2 => EnumerationType::Disconnected,
            _ => EnumerationType::Unknown,
        }
    }
}

#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq)]
pub struct Version {
    major: u8,
    minor: u8,
    patch: u8,
}

impl Display for Version {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}.{}.{}", self.major, self.minor, self.patch)
    }
}

impl Debug for Version {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Version: {}.{}.{}", self.major, self.minor, self.patch)
    }
}

impl FromByteSlice for Version {
    fn from_le_byte_slice(bytes: &[u8]) -> Self {
        Version {
            major: bytes[0],
            minor: bytes[1],
            patch: bytes[2],
        }
    }

    fn bytes_expected() -> usize {
        3
    }
}

impl ToBytes for Version {
    fn write_to_slice(&self, target: &mut [u8]) -> usize {
        target[0] = self.major;
        target[1] = self.minor;
        target[2] = self.patch;
        3
    }
}

/// Devices send `EnumerateResponse`s when they are connected, disconnected or when an enumeration was
/// triggered by the user using the [`Enumerate`](crate::ip_connection::async_io::AsyncIpConnection::enumerate()) method.
#[derive(Copy, Clone, Debug)]
pub struct EnumerateResponse {
    /// The UID of the device.
    pub uid: Uid,
    /// UID where the device is connected to.
    /// For a Bricklet this is the UID of the Brick or Bricklet it is connected to.
    /// For a Brick it is the UID of the bottommost Brick in the stack.
    /// For the bottommost Brick in a stack it is "0".
    /// With this information it is possible to reconstruct the complete network topology.
    pub connected_uid: Uid,
    /// For Bricks: '0' - '8' (position in stack). For Bricklets: 'a' - 'd' (position on Brick).
    pub position: char,
    /// Major, minor and release number for hardware version.
    pub hardware_version: Version,
    /// Major, minor and release number for firmware version.
    pub firmware_version: Version,
    /// An enum or a number that represents the device.
    /// The device identifier numbers can be found [here](https://www.tinkerforge.com/en/doc/Software/Device_Identifier.html).
    pub device_identifier: u16,
    /// Type of enumeration. See [`EnumerationType`]
    pub enumeration_type: EnumerationType,
}

impl EnumerateResponse {
    pub fn extract_enumeration_packet(
        p: Result<PacketData, BroadcastStreamRecvError>,
    ) -> Option<EnumerateResponse> {
        match p {
            Ok(p) if p.header().function_id == 253 => {
                Result::<EnumerateResponse, Base58Error>::from_le_byte_slice(p.body()).ok()
            }
            _ => None,
        }
    }
}

impl FromByteSlice for Result<EnumerateResponse, Base58Error> {
    fn from_le_byte_slice(bytes: &[u8]) -> Result<EnumerateResponse, Base58Error> {
        let uid = Uid::from_str(
            &str::from_utf8(&bytes[0..8])
                .expect("Could not convert to string. This is a bug in the rust bindings.")
                .replace('\u{0}', ""),
        )?;
        let string = str::from_utf8(&bytes[8..16])
            .expect("Could not convert to string. This is a bug in the rust bindings.")
            .replace('\u{0}', "");
        let connected_uid = Uid::from_str(&string)?;
        Ok(EnumerateResponse {
            uid,
            connected_uid,
            position: bytes[16] as char,
            hardware_version: Version::from_le_byte_slice(&bytes[17..20]),
            firmware_version: Version::from_le_byte_slice(&bytes[20..23]),
            device_identifier: u16::from_le_byte_slice(&bytes[23..25]),
            enumeration_type: EnumerationType::from(bytes[25]),
        })
    }

    fn bytes_expected() -> usize {
        26
    }
}

struct ServerNonce([u8; 4]);

impl FromByteSlice for ServerNonce {
    fn from_le_byte_slice(bytes: &[u8]) -> ServerNonce {
        ServerNonce([bytes[0], bytes[1], bytes[2], bytes[3]])
    }

    fn bytes_expected() -> usize {
        4
    }
}

/// This error is returned if the remote's server nonce could not be queried.
#[derive(Debug, Copy, Clone)]
pub enum AuthenticateError {
    SecretInvalid,
    CouldNotGetServerNonce,
}

impl std::fmt::Display for AuthenticateError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                AuthenticateError::SecretInvalid => {
                    "Authentication secret contained non-ASCII characters"
                }
                AuthenticateError::CouldNotGetServerNonce => "Could not get server nonce",
            }
        )
    }
}

impl std::error::Error for AuthenticateError {}

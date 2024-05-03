use anyhow::Result;

use tinkerforge_base::{byte_converter::ToBytes, ip_connection::async_io::AsyncIpConnection};

use crate::bindings::lcd_128_x_64::{
    Lcd128X64Bricklet,
    SetDisplayConfigurationRequest,
    TouchLedConfig,
    WritePixelsRequest,
};

mod bindings;

mod experiments {
    #[derive(Copy, Clone, PartialEq, Debug)]
    struct WritePixelsRequest<'d> {
        pub x_start: u8,
        pub y_start: u8,
        pub x_end: u8,
        pub y_end: u8,
        pub offset: u16,
        pub stream_data: &'d [bool],
    }

    impl<'d> WritePixelsRequest<'d> {
        pub fn write_to_slices(&'d self) -> WritePixelRequestIterator<'d> {
            WritePixelRequestIterator {
                request: self,
                offset: 0,
            }
        }
    }

    pub struct WritePixelRequestIterator<'r> {
        request: &'r WritePixelsRequest<'r>,
        offset: u16,
    }

    impl<'r> Iterator for WritePixelRequestIterator<'r> {
        type Item = WritePixelRequestSlice<'r>;

        fn next(&mut self) -> Option<Self::Item> {
            if self.offset as usize >= self.request.stream_data.len() {
                None
            } else {
                let slice_offset = self.offset;
                let length = self.request.stream_data.len() as u16;
                let packet_length = u16::min(448, length - slice_offset);
                self.offset += packet_length;
                let data = &self.request.stream_data
                    [slice_offset as usize..slice_offset as usize + packet_length as usize];
                Some(WritePixelRequestSlice {
                    request: self.request,
                    offset: slice_offset + self.request.offset,
                    length: length + self.request.offset,
                    data,
                })
            }
        }
    }

    pub struct WritePixelRequestSlice<'r> {
        request: &'r WritePixelsRequest<'r>,
        offset: u16,
        length: u16,
        data: &'r [bool],
    }

    impl<'r> tinkerforge_base::byte_converter::ToBytes for WritePixelRequestSlice<'r> {
        fn write_to_slice(&self, target: &mut [u8]) -> usize {
            let mut p = 0;
            p += self
                .request
                .x_start
                .write_to_slice(&mut target[p..p + 1usize]);
            p += self
                .request
                .y_start
                .write_to_slice(&mut target[p..p + 1usize]);
            self.request
                .x_end
                .write_to_slice(&mut target[2usize..3usize]);
            self.request
                .y_end
                .write_to_slice(&mut target[3usize..4usize]);
            self.length.write_to_slice(&mut target[4usize..6usize]);
            self.offset.write_to_slice(&mut target[6usize..8usize]);

            8 + self.data.write_to_slice(&mut target[8usize..])
        }
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    let connection = AsyncIpConnection::new("127.0.0.1:4223").await?;
    let mut device =
        tinkerforge_base::device::Device::new("R4c".into(), connection.clone(), "LCD 128x64");
    let mut bricklet = Lcd128X64Bricklet::new("R4c", connection);
    bricklet.clear_display().await?;
    bricklet.set_touch_led_config(TouchLedConfig::Off).await?;
    bricklet
        .set_display_configuration(SetDisplayConfigurationRequest {
            contrast: 14,
            backlight: 100,
            invert: false,
            automatic_draw: true,
        })
        .await?;

    let mut pattern = [false; 8192];
    for (idx, value) in pattern.iter_mut().enumerate() {
        *value = idx % 3 != 0;
    }
    let request = WritePixelsRequest {
        x_start: 0,
        y_start: 0,
        x_end: 127,
        y_end: 63,
        data: &pattern,
    };
    bricklet.write_pixels(request).await?;

    /*let mut buffer = [0; 64];
    for slice in request.write_to_slices() {
        let length = slice.write_to_slice(&mut buffer);
        //println!("Length: {length}");
        let payload = &buffer[0..length];
        for byte in payload {
            print!("{byte:02x}")
        }
        println!();
        device
            .set(1u8, payload, Some(std::time::Duration::from_secs(20)))
            .await?;
    }*/
    Ok(())
}

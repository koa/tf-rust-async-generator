use crate::bindings::lcd_128_x_64::{Lcd128X64Bricklet, SetDisplayConfigurationRequest};
use tinkerforge_base::ip_connection::async_io::AsyncIpConnection;
use anyhow::Result;

mod bindings;

#[tokio::main]
async fn main()->Result<()> {
    let connection = AsyncIpConnection::new("127.0.0.1:4223").await?;
    let mut bricklet = Lcd128X64Bricklet::new("R4c", connection);
    bricklet.set_display_configuration(SetDisplayConfigurationRequest{
        contrast: 20,
        backlight: 50,
        invert: false,
        automatic_draw: false, 
    }).await?;
    println!("Hello, world!");
    Ok(())
}

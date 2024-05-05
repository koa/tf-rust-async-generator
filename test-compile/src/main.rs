use anyhow::Result;

use tinkerforge_base::ip_connection::async_io::AsyncIpConnection;

use crate::bindings::lcd_128_x_64::{
    Lcd128X64Bricklet,
    SetDisplayConfigurationRequest,
    TouchLedConfig,
    WritePixelsRequest,
};
use crate::bindings::master::MasterBrick;

mod bindings;


#[tokio::main]
async fn main() -> Result<()> {
    let connection = AsyncIpConnection::new("127.0.0.1:4223").await?;
    let mut master = MasterBrick::new("6DyH5n", connection.clone());
    master.disable_status_led().await?;
    let mut bricklet = Lcd128X64Bricklet::new("R4c", connection.clone());
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

    Ok(())
}

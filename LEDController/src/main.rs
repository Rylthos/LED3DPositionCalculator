use ddp_rs::connection;
use ddp_rs::protocol;

use std::sync::Arc;
use std::sync::Mutex;
use std::thread;

pub mod colour;
pub mod led_controller;
pub mod vec3;

use crate::colour::Colour;
use crate::led_controller::PixelController;
use crate::vec3::Vec3;

const NUM_PIXELS: usize = 10;
const FPS: u64 = 17; // 60 FPS

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut conn = connection::DDPConnection::try_new(
        "4.3.2.1:4048",                   // The IP address of the device followed by :4048
        protocol::PixelConfig::default(), // Default is RGB, 8 bits ber channel
        protocol::ID::Default,
        std::net::UdpSocket::bind("0.0.0.0:4048").unwrap(), // can be any unused port on 0.0.0.0, but protocol recommends 4048
    )?;

    let controller = Arc::new(Mutex::new(PixelController::new(NUM_PIXELS)));

    if let Ok(mut con) = controller.lock() {
        con.read_pixels_from_file("Output.pixels");
    }

    let running = Arc::new(Mutex::new(true));

    let thread_controller = controller.clone();
    let thread_running = running.clone();

    let handle = thread::spawn(move || loop {
        if let Ok(running) = thread_running.lock() {
            if !(*running) {
                break;
            }
        }

        if let Ok(mut controller) = thread_controller.lock() {
            controller.transmit(&mut conn);
        }

        std::thread::sleep(std::time::Duration::from_millis(FPS));
    });

    loop {
        loop {
            if let Ok(mut con) = controller.lock() {
                con.plane(Vec3::new(0., 60., 0.), Vec3::new(0., 1., 0.), 10.);
                break;
            }
        }

        std::thread::sleep(std::time::Duration::from_millis(FPS));
    }
}

use ddp_rs::connection;
use ddp_rs::protocol;

use std::sync::Arc;
use std::sync::Mutex;
use std::thread;

const NUM_PIXELS: usize = 1;
const FPS: u64 = 17; // 60 FPS

#[derive(Copy, Clone, Debug)]
struct Pixel {
    r: u8,
    g: u8,
    b: u8,
}

#[derive(Debug)]
struct PixelController {
    pixels: Vec<Pixel>,
}

impl PixelController {
    pub fn new(num_pixels: usize) -> PixelController {
        let mut controller = PixelController { pixels: Vec::new() };

        controller
            .pixels
            .resize(num_pixels, Pixel { r: 0, g: 0, b: 0 });
        controller
    }

    pub fn transmit(&mut self, conn: &mut connection::DDPConnection) {
        let values = self.pixels_to_arr();

        println!("Transmitting: {:?}", values);
        let temp = conn.write(values.as_slice());

        if temp.is_err() {
            panic!("Something went Wrong");
        }
    }

    fn pixels_to_arr(&mut self) -> Vec<u8> {
        let mut pixel_values: Vec<u8> = Vec::new();
        for p in self.pixels.clone() {
            pixel_values.push(p.r);
            pixel_values.push(p.g);
            pixel_values.push(p.b);
        }
        pixel_values
    }

    pub fn solid_colour(&mut self, colour: Pixel) {
        for pixel in self.pixels.iter_mut() {
            *pixel = colour;
        }
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut conn = connection::DDPConnection::try_new(
        "192.168.1.40:4048", // The IP address of the device followed by :4048
        protocol::PixelConfig::default(), // Default is RGB, 8 bits ber channel
        protocol::ID::Default,
        std::net::UdpSocket::bind("0.0.0.0:4048").unwrap(), // can be any unused port on 0.0.0.0, but protocol recommends 4048
    )?;

    let controller = Arc::new(Mutex::new(PixelController::new(NUM_PIXELS)));

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

    // loop sets some colors for the first 6 pixels to see if it works
    for i in 0u8..100u8 {
        let high = 10u8.overflowing_mul(i).0;

        let pixel = Pixel {
            r: high,
            g: high,
            b: high,
        };

        loop {
            if let Ok(mut con) = controller.lock() {
                con.solid_colour(pixel);
                break;
            }
        }

        std::thread::sleep(std::time::Duration::from_millis(20));
    }

    loop {
        if let Ok(mut is_running) = running.lock() {
            *is_running = false;
            break;
        } else {
            continue;
        }
    }

    let _ = handle.join();

    Ok(())
}

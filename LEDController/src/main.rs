use std::io;
use std::sync::Arc;
use std::sync::Mutex;
use std::thread;

pub mod app;
pub mod colour;
pub mod led_controller;
pub mod vec3;

use crate::colour::Colour;
use crate::led_controller::PixelController;
use crate::vec3::Vec3;

use crate::app::App;

const NUM_PIXELS: usize = 10;
const FPS: u64 = 17; // 60 FPS

fn main() -> io::Result<()> {
    let mut terminal = ratatui::init();
    let app_result = App::new("4.3.2.1:4048", NUM_PIXELS, FPS).run(&mut terminal);
    ratatui::restore();
    app_result
}

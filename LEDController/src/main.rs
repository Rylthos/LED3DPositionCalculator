use std::io;

pub mod app;
pub mod colour;
pub mod effect;
pub mod led_controller;
pub mod vec3;

use crate::app::App;

const NUM_PIXELS: usize = 300;
const FPS_MS: u64 = 50; // 60 FPS

fn main() -> io::Result<()> {
    let mut terminal = ratatui::init();
    let app_result = App::new("192.168.0.163:4048", NUM_PIXELS, FPS_MS).run(&mut terminal);
    ratatui::restore();
    app_result
}

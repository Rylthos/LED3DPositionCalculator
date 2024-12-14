use crossterm::event::KeyEvent;
use ratatui::{layout::Rect, Frame};

use crate::pixel::Pixel;

pub trait EffectTrait {
    fn as_trait_mut(&mut self) -> &mut dyn EffectTrait;

    fn update(&mut self, delta: f32, pixels: &Vec<Pixel>);
    fn render(&self, pixels: &mut Vec<Pixel>);

    fn handle_input(&mut self, event: KeyEvent);
    fn draw(&self, frame: &mut Frame, layout: Rect);
}

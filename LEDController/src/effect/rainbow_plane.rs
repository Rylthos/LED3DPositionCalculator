use crossterm::event::{KeyCode, KeyEvent, KeyEventKind};
use ratatui::{
    buffer::Buffer,
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Style, Stylize},
    symbols::border,
    text::{Line, Span, Text},
    widgets::{Block, Borders, Clear, List, ListItem, Paragraph, Wrap},
    DefaultTerminal, Frame,
};

use crate::colour::*;
use crate::effect::effect_list;
use crate::effect::effect_trait::EffectTrait;
use crate::pixel::Pixel;
use crate::vec3::Vec3;

#[derive(Copy, Clone)]
pub struct RainbowPlaneEffect {
    pos: Vec3,
    multiplier: f32,
    movement_speed: f32,
}

impl RainbowPlaneEffect {
    pub fn default() -> RainbowPlaneEffect {
        RainbowPlaneEffect {
            pos: Vec3::new(0., 0., 0.),
            multiplier: 2.,
            movement_speed: 100.,
        }
    }
}

impl EffectTrait for RainbowPlaneEffect {
    fn as_trait_mut(&mut self) -> &mut dyn EffectTrait {
        self
    }

    fn update(&mut self, delta: f32, _pixels: &Vec<Pixel>) {
        let normal = Vec3::new(0., 1., 0.);

        let movement = self.movement_speed * delta;
        let mut new_pos = Vec3::new(
            self.pos.x + normal.x * movement,
            self.pos.y + normal.y * movement,
            self.pos.z + normal.z * movement,
        );

        new_pos.y = new_pos.y % 720. + 360.;
        self.pos = new_pos;
    }

    fn render(&self, pixels: &mut Vec<Pixel>) {
        let normal = Vec3::new(0., 1., 0.);

        for pixel in pixels.iter_mut() {
            let new_position = Vec3::sub(pixel.position, self.pos);
            let distance =
                self.multiplier * Vec3::dot(new_position, normal).abs() / Vec3::mag(normal);
            pixel.colour = Colour::new(distance, 1., 1.);
        }
    }

    fn handle_input(&mut self, _event: KeyEvent) {}
    fn draw(&self, _frame: &mut Frame, _layout: Rect) {}
}

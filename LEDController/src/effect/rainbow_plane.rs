use crossterm::event::{KeyCode, KeyEvent};
use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph},
    Frame,
};

use crate::colour::*;
use crate::effect::effect_trait::EffectTrait;
use crate::pixel::Pixel;
use crate::vec3::Vec3;

#[derive(Copy, Clone)]
pub struct RainbowPlaneEffect {
    pos: Vec3,
    multiplier: u32,
    movement_speed: f32,
}

impl RainbowPlaneEffect {
    pub fn default() -> RainbowPlaneEffect {
        RainbowPlaneEffect {
            pos: Vec3::new(0., 0., 0.),
            multiplier: 1,
            movement_speed: 50.,
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
            let distance = f32::abs(
                (self.multiplier as f32)
                    * (Vec3::dot(new_position, normal).abs() / Vec3::mag(normal)),
            );
            pixel.colour = Colour::new(distance + 30., 1., 1.);
        }
    }

    fn handle_input(&mut self, event: KeyEvent) {
        match event.code {
            KeyCode::Char('K') => {
                self.movement_speed = (self.movement_speed + 10.).clamp(0., 1000.)
            }
            KeyCode::Char('k') => self.movement_speed = (self.movement_speed + 5.).clamp(0., 1000.),
            KeyCode::Char('J') => {
                self.movement_speed = (self.movement_speed - 10.).clamp(0., 1000.)
            }
            KeyCode::Char('j') => self.movement_speed = (self.movement_speed - 5.).clamp(0., 1000.),
            KeyCode::Up => self.multiplier = (self.multiplier + 1).clamp(1, 10),
            KeyCode::Down => self.multiplier = (self.multiplier - 1).clamp(1, 10),
            _ => {}
        }
    }

    fn draw(&self, frame: &mut Frame, layout: Rect) {
        let blocks = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([
                Constraint::Percentage(33),
                Constraint::Min(1),
                Constraint::Percentage(33),
            ])
            .split(layout);

        let block = Block::default()
            .borders(Borders::ALL)
            .style(Style::default());

        let block_text = Paragraph::new(vec![
            Line::from(vec![
                Span::styled("J ", Style::default().fg(Color::Red)),
                Span::styled(
                    format!("Movement Speed: {:3.0}", self.movement_speed),
                    Style::default().fg(Color::White),
                ),
                Span::styled(" K", Style::default().fg(Color::Green)),
            ]),
            Line::from(vec![
                Span::styled("<down> ", Style::default().fg(Color::Red)),
                Span::styled(
                    format!("Multiplier: {:2}", self.multiplier),
                    Style::default().fg(Color::White),
                ),
                Span::styled(" <up>", Style::default().fg(Color::Green)),
            ]),
        ])
        .centered()
        .block(block);

        frame.render_widget(block_text, blocks[1]);
    }
}

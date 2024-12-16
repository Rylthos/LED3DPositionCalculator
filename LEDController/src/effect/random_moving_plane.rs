use crossterm::event::{KeyCode, KeyEvent};
use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph},
    Frame,
};

use rand;

use crate::colour::*;
use crate::effect::constants::*;
use crate::effect::effect_trait::EffectTrait;
use crate::pixel::Pixel;
use crate::vec3::Vec3;

#[derive(Copy, Clone)]
pub struct RandomMovingPlaneEffect {
    pos: Vec3,
    normal: Vec3,
    colour: Colour,
    movement_speed: f32,
    decay: f32,
    distance: f32,
}

impl RandomMovingPlaneEffect {
    pub fn default() -> RandomMovingPlaneEffect {
        let mut eff = RandomMovingPlaneEffect {
            pos: Vec3::new(0., 0., 0.),
            normal: Vec3::new(0., 0., 0.),
            movement_speed: 90.,
            distance: 30.,
            decay: 0.9,
            colour: WHITE,
        };

        eff.random_pos();
        eff.recalculate_normal();
        eff.random_colour();

        eff
    }

    fn random_pos(&mut self) {
        let radius = f32::max(WIDTH, DEPTH) + self.distance;

        let y = rand::random::<f32>() * HEIGHT;
        let phi = rand::random::<f32>() * 2. * std::f32::consts::PI;

        let x = radius * f32::cos(phi);
        let z = radius * f32::sin(phi);

        self.pos = Vec3::new(x, y, z);
    }

    fn recalculate_normal(&mut self) {
        let center = Vec3::new(0., HEIGHT / 2., 0.);

        self.normal = Vec3::norm(Vec3::sub(center, self.pos));
    }

    fn random_colour(&mut self) {
        let hue = (self.colour.h + (60. + f32::round(rand::random::<f32>() * 240.))) % 360.;
        self.colour = Colour::new(hue, 1., 1.);
    }

    fn should_be_coloured(&self, pixel: &Pixel) -> Result<f32, ()> {
        let new_position = Vec3::sub(pixel.position, self.pos);
        let distance =
            f32::abs(Vec3::dot(new_position, self.normal).abs() / Vec3::mag(self.normal));
        if distance < self.distance {
            Ok(distance)
        } else {
            Err(())
        }
    }
}

impl EffectTrait for RandomMovingPlaneEffect {
    fn as_trait_mut(&mut self) -> &mut dyn EffectTrait {
        self
    }

    fn update(&mut self, delta: f32, pixels: &Vec<Pixel>) {
        let movement = self.movement_speed * delta;
        let new_pos = Vec3::new(
            self.pos.x + self.normal.x * movement,
            self.pos.y + self.normal.y * movement,
            self.pos.z + self.normal.z * movement,
        );

        self.pos = new_pos;

        let center = Vec3::new(0., HEIGHT / 2., 0.);
        let direction = Vec3::sub(center, self.pos);

        if Vec3::dot(self.normal, direction) < 0. {
            let mut hit = false;
            for pixel in pixels.iter() {
                if let Ok(_) = self.should_be_coloured(pixel) {
                    hit = true;
                    break;
                }
            }

            if !hit {
                self.random_pos();
                self.random_colour();
                self.recalculate_normal();
            }
        }
    }

    fn render(&self, pixels: &mut Vec<Pixel>) {
        for pixel in pixels.iter_mut() {
            if let Ok(_) = self.should_be_coloured(pixel) {
                pixel.colour = Colour {
                    h: self.colour.h,
                    s: 1.,
                    v: 1.,
                };
            } else {
                let mut new_value = pixel.colour.v * self.decay;
                if new_value < 0.1 {
                    new_value = 0.;
                };

                pixel.colour = Colour {
                    h: pixel.colour.h,
                    s: pixel.colour.s,
                    v: new_value,
                };
            }
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

            KeyCode::Char('n') => self.decay = (self.decay - 0.01).clamp(0., 1.),
            KeyCode::Char('m') => self.decay = (self.decay + 0.01).clamp(0., 1.),

            KeyCode::Up => self.distance = (self.distance + 1.).clamp(1., 200.),
            KeyCode::Down => self.distance = (self.distance - 1.).clamp(1., 200.),
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
                    format!("Distance: {:3.0}", self.distance),
                    Style::default().fg(Color::White),
                ),
                Span::styled(" <up>", Style::default().fg(Color::Green)),
            ]),
            Line::from(vec![
                Span::styled("n ", Style::default().fg(Color::Red)),
                Span::styled(
                    format!("Decay: {:1.2}", self.decay),
                    Style::default().fg(Color::White),
                ),
                Span::styled(" m", Style::default().fg(Color::Green)),
            ]),
        ])
        .centered()
        .block(block);

        frame.render_widget(block_text, blocks[1]);
    }
}

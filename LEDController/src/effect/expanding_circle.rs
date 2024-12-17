use crossterm::event::{KeyCode, KeyEvent};
use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph},
    Frame,
};

use ini::Ini;
use rand;

use crate::colour::*;
use crate::effect::constants::*;
use crate::effect::effect_trait::EffectTrait;
use crate::pixel::Pixel;
use crate::vec3::Vec3;

#[derive(Copy, Clone)]
pub struct ExpandingCircleEffect {
    radius: f32,
    expansion_speed: f32,
    colour: Colour,
}

impl ExpandingCircleEffect {
    pub fn default() -> ExpandingCircleEffect {
        let mut eff = ExpandingCircleEffect {
            radius: 0.,
            expansion_speed: 50.,
            colour: BLACK,
        };

        eff.random_colour();
        eff
    }

    pub fn random_colour(&mut self) {
        let hue = (self.colour.h + (60. + f32::round(rand::random::<f32>() * 240.))) % 360.;
        self.colour = Colour::new(hue, 1., 1.);
    }

    fn should_be_coloured(&self, pixel: &Pixel) -> Result<f32, ()> {
        let center = Vec3::new(0., HEIGHT / 2., 0.);
        let new_position = Vec3::sub(pixel.position, center);

        let distance = Vec3::mag(new_position);

        if distance < self.radius {
            Ok(distance)
        } else {
            Err(())
        }
    }
}

impl EffectTrait for ExpandingCircleEffect {
    fn as_trait_mut(&mut self) -> &mut dyn EffectTrait {
        self
    }

    fn save_settings(&self) {
        let mut config: Ini = Ini::new();
        if let Ok(x) = Ini::load_from_file(CONFIG_NAME) {
            config = x;
        }

        config
            .with_section(Some("Effect.ExpandingCircle"))
            .set("expansion_speed", format!("{:3.0}", self.expansion_speed));

        config.write_to_file(CONFIG_NAME).unwrap();
    }

    fn read_settings(&mut self) {
        if let Ok(config) = Ini::load_from_file(CONFIG_NAME) {
            if let Some(section) = config.section(Some("Effect.ExpandingCircle")) {
                if let Some(expansion_speed) = section.get("expansion_speed") {
                    self.expansion_speed = expansion_speed.parse().unwrap();
                }
            }
        }
    }

    fn update(&mut self, delta: f32, pixels: &Vec<Pixel>) {
        self.radius += self.expansion_speed * delta;

        let mut all_coloured = true;

        for pixel in pixels.iter() {
            if let Err(_) = self.should_be_coloured(pixel) {
                all_coloured = false;
                break;
            }
        }

        if all_coloured {
            self.random_colour();
            self.radius = 0.;
        }
    }

    fn render(&self, pixels: &mut Vec<Pixel>) {
        for pixel in pixels.iter_mut() {
            if let Ok(_) = self.should_be_coloured(pixel) {
                pixel.colour = self.colour;
            }
        }
    }

    fn handle_input(&mut self, event: KeyEvent) {
        match event.code {
            KeyCode::Char('K') => {
                self.expansion_speed = (self.expansion_speed + 10.).clamp(0., 1000.)
            }
            KeyCode::Char('k') => {
                self.expansion_speed = (self.expansion_speed + 5.).clamp(0., 1000.)
            }
            KeyCode::Char('J') => {
                self.expansion_speed = (self.expansion_speed - 10.).clamp(0., 1000.)
            }
            KeyCode::Char('j') => {
                self.expansion_speed = (self.expansion_speed - 5.).clamp(0., 1000.)
            }
            // KeyCode::Up => self.multiplier = (self.multiplier + 1).clamp(1, 10),
            // KeyCode::Down => self.multiplier = (self.multiplier - 1).clamp(1, 10),
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
                    format!("Expansion Speed: {:3.0}", self.expansion_speed),
                    Style::default().fg(Color::White),
                ),
                Span::styled(" K", Style::default().fg(Color::Green)),
            ]),
            // Line::from(vec![
            //     Span::styled("<down> ", Style::default().fg(Color::Red)),
            //     Span::styled(
            //         format!("Multiplier: {:2}", self.multiplier),
            //         Style::default().fg(Color::White),
            //     ),
            //     Span::styled(" <up>", Style::default().fg(Color::Green)),
            // ]),
        ])
        .centered()
        .block(block);

        frame.render_widget(block_text, blocks[1]);
    }
}

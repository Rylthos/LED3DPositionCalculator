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

#[derive(Copy, Clone)]
pub struct SolidColourEffect {
    colour: Colour,
}

impl SolidColourEffect {
    pub fn default() -> SolidColourEffect {
        SolidColourEffect { colour: CYAN }
    }
}

impl EffectTrait for SolidColourEffect {
    fn as_trait_mut(&mut self) -> &mut dyn EffectTrait {
        self
    }

    fn update(&mut self, _delta: f32, _pixels: &Vec<Pixel>) {}

    fn render(&self, pixels: &mut Vec<Pixel>) {
        for pixel in pixels.iter_mut() {
            pixel.colour = self.colour;
        }
    }

    fn handle_input(&mut self, event: KeyEvent) {
        match event.code {
            KeyCode::Char('h') => self.colour.h = (self.colour.h + 10.) % 360.,
            KeyCode::Char('H') => {
                self.colour.h = self.colour.h - 10.;
                if self.colour.h < 0. {
                    self.colour.h = 350.
                }
                self.colour.h = self.colour.h % 360.;
            }
            KeyCode::Char('s') => self.colour.s = (self.colour.s + 0.05).clamp(0., 1.),
            KeyCode::Char('S') => self.colour.s = (self.colour.s - 0.05).clamp(0., 1.),
            KeyCode::Char('v') => self.colour.v = (self.colour.v + 0.05).clamp(0., 1.),
            KeyCode::Char('V') => self.colour.v = (self.colour.v - 0.05).clamp(0., 1.),
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

        let colour_block = Block::default()
            .borders(Borders::ALL)
            .style(Style::default());

        let colour = Paragraph::new(vec![
            Line::from(vec![
                Span::styled("H ", Style::default().fg(Color::Red)),
                Span::styled(
                    format!("Hue: {:3.0}", self.colour.h),
                    Style::default().fg(Color::White),
                ),
                Span::styled(" h", Style::default().fg(Color::Green)),
            ]),
            Line::from(vec![
                Span::styled("S ", Style::default().fg(Color::Red)),
                Span::styled(
                    format!("Saturation: {:1.2}", self.colour.s),
                    Style::default().fg(Color::White),
                ),
                Span::styled(" s", Style::default().fg(Color::Green)),
            ]),
            Line::from(vec![
                Span::styled("V ", Style::default().fg(Color::Red)),
                Span::styled(
                    format!("Value: {:1.2}", self.colour.v),
                    Style::default().fg(Color::White),
                ),
                Span::styled(" v", Style::default().fg(Color::Green)),
            ]),
        ])
        .centered()
        .block(colour_block);

        frame.render_widget(colour, blocks[1]);
    }
}

use crossterm::event::KeyEvent;
use ratatui::{layout::Rect, Frame};

use crate::effect::effect_trait::EffectTrait;
use crate::effect::rainbow_plane::RainbowPlaneEffect;
use crate::effect::solid_colour::SolidColourEffect;
use crate::pixel::Pixel;

#[derive(Copy, Clone)]
pub enum Effect {
    SolidColour(SolidColourEffect),
    RainbowPlane(RainbowPlaneEffect),
}

const NUM_EFFECTS: i32 = 2;

impl Effect {
    pub fn to_string(&self) -> &str {
        match self {
            Effect::SolidColour(_) => "Solid Colour",
            Effect::RainbowPlane(_) => "Rainbow Colour",
        }
    }

    pub fn render(&self, pixels: &mut Vec<Pixel>) {
        self.decompose().render(pixels);
    }

    pub fn update(&mut self, delta: f32, pixels: &Vec<Pixel>) {
        self.decompose_mut().update(delta, pixels);
    }

    pub fn handle_input(&mut self, event: KeyEvent) {
        self.decompose_mut().handle_input(event);
    }

    pub fn draw(&self, frame: &mut Frame, layout: Rect) {
        self.decompose().draw(frame, layout);
    }

    pub fn change_effect(&mut self, offset: i32) {
        let current_id = Effect::effect_to_id(*self);
        let new_id_unnormalized = current_id + offset;
        let new_id = if new_id_unnormalized < 0 {
            NUM_EFFECTS - 1
        } else {
            new_id_unnormalized % (NUM_EFFECTS)
        };

        *self = Effect::id_to_effect(new_id);
    }

    pub fn decompose(&self) -> Box<dyn EffectTrait> {
        match self {
            Effect::SolidColour(e) => Box::new(*e) as Box<dyn EffectTrait>,
            Effect::RainbowPlane(e) => Box::new(*e) as Box<dyn EffectTrait>,
        }
    }

    pub fn decompose_mut(&mut self) -> &mut dyn EffectTrait {
        match self {
            Effect::SolidColour(e) => e.as_trait_mut(),
            Effect::RainbowPlane(e) => e.as_trait_mut(),
        }
    }

    pub fn effect_to_id(effect: Effect) -> i32 {
        match effect {
            Effect::SolidColour(_) => 0,
            Effect::RainbowPlane(_) => 1,
        }
    }

    pub fn id_to_effect(effect_id: i32) -> Effect {
        match effect_id {
            0 => Effect::SolidColour(SolidColourEffect::default()),
            1 => Effect::RainbowPlane(RainbowPlaneEffect::default()),
            _ => panic!("Undefined ID"),
        }
    }
}

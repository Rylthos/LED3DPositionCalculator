use crossterm::event::{KeyCode, KeyEvent};
use ratatui::{layout::Rect, Frame};

use crate::effect::effect_trait::EffectTrait;
use crate::effect::expanding_circle::ExpandingCircleEffect;
use crate::effect::rainbow_plane::RainbowPlaneEffect;
use crate::effect::random_moving_plane::RandomMovingPlaneEffect;
use crate::effect::solid_colour::SolidColourEffect;
use crate::pixel::Pixel;

#[derive(Copy, Clone)]
pub enum Effect {
    SolidColour(SolidColourEffect),
    RainbowPlane(RainbowPlaneEffect),
    RandomMovingPlane(RandomMovingPlaneEffect),
    ExpandingCircle(ExpandingCircleEffect),
}

const NUM_EFFECTS: i32 = 4;

impl Effect {
    pub fn to_string(&self) -> &str {
        match self {
            Effect::SolidColour(_) => "Solid Colour",
            Effect::RainbowPlane(_) => "Rainbow Plane",
            Effect::RandomMovingPlane(_) => "Rainbow Moving Plane",
            Effect::ExpandingCircle(_) => "Expanding Circle",
        }
    }

    pub fn save_settings(&self) {
        self.decompose().save_settings();
    }

    pub fn read_settings(&mut self) {
        self.decompose_mut().read_settings();
    }

    pub fn render(&self, pixels: &mut Vec<Pixel>) {
        self.decompose().render(pixels);
    }

    pub fn update(&mut self, delta: f32, pixels: &Vec<Pixel>) {
        self.decompose_mut().update(delta, pixels);
    }

    pub fn handle_input(&mut self, event: KeyEvent) {
        match event.code {
            KeyCode::Char('r') => *self = Effect::default_effect(Effect::effect_to_id(*self)),
            _ => self.decompose_mut().handle_input(event),
        }
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

        self.save_settings();
        *self = Effect::id_to_effect(new_id);
    }

    pub fn decompose(&self) -> Box<dyn EffectTrait> {
        match self {
            Effect::SolidColour(e) => Box::new(*e) as Box<dyn EffectTrait>,
            Effect::RainbowPlane(e) => Box::new(*e) as Box<dyn EffectTrait>,
            Effect::RandomMovingPlane(e) => Box::new(*e) as Box<dyn EffectTrait>,
            Effect::ExpandingCircle(e) => Box::new(*e) as Box<dyn EffectTrait>,
        }
    }

    pub fn decompose_mut(&mut self) -> &mut dyn EffectTrait {
        match self {
            Effect::SolidColour(e) => e.as_trait_mut(),
            Effect::RainbowPlane(e) => e.as_trait_mut(),
            Effect::RandomMovingPlane(e) => e.as_trait_mut(),
            Effect::ExpandingCircle(e) => e.as_trait_mut(),
        }
    }

    pub fn effect_to_id(effect: Effect) -> i32 {
        match effect {
            Effect::SolidColour(_) => 0,
            Effect::RainbowPlane(_) => 1,
            Effect::RandomMovingPlane(_) => 2,
            Effect::ExpandingCircle(_) => 3,
        }
    }

    pub fn default_effect(effect_id: i32) -> Effect {
        match effect_id {
            0 => Effect::SolidColour(SolidColourEffect::default()),
            1 => Effect::RainbowPlane(RainbowPlaneEffect::default()),
            2 => Effect::RandomMovingPlane(RandomMovingPlaneEffect::default()),
            3 => Effect::ExpandingCircle(ExpandingCircleEffect::default()),
            _ => panic!("Undefined ID"),
        }
    }

    pub fn id_to_effect(effect_id: i32) -> Effect {
        let mut effect = Effect::default_effect(effect_id);
        effect.read_settings();
        effect
    }
}

use crate::colour::*;
use crate::vec3::Vec3;

#[derive(Debug, Clone, Copy)]
pub enum Effect {
    SolidColour(Colour),
    MovingPlane(Vec3, Vec3, Colour, Colour, f32),
    RainbowPlane(Vec3),
}

const NUM_EFFECTS: i32 = 3;

impl Effect {
    pub fn default_solid() -> Effect {
        Effect::SolidColour(CYAN)
    }

    pub fn default_moving_plane() -> Effect {
        Effect::MovingPlane(
            Vec3::new(0., 0., 0.),
            Vec3::new(0., 1., 0.),
            WHITE,
            GREEN,
            1.,
        )
    }

    pub fn default_rainbow_plane() -> Effect {
        Effect::RainbowPlane(Vec3::new(360., 0., 0.))
    }

    pub fn change_effect(effect: Effect, offset: i32) -> Effect {
        let current_id = Effect::effect_to_id(effect);
        let new_id_unnormalized = (current_id as i32) + offset;
        let new_id = if new_id_unnormalized < 0 {
            NUM_EFFECTS - 1
        } else {
            new_id_unnormalized % (NUM_EFFECTS)
        };

        Effect::id_to_effect(new_id as usize)
    }

    pub fn effect_to_id(effect: Effect) -> usize {
        match effect {
            Effect::SolidColour(..) => 0,
            Effect::MovingPlane(..) => 1,
            Effect::RainbowPlane(..) => 2,
        }
    }

    pub fn id_to_effect(effect_id: usize) -> Effect {
        match effect_id {
            0 => Effect::default_solid(),
            1 => Effect::default_moving_plane(),
            2 => Effect::default_rainbow_plane(),
            _ => panic!("Undefined ID"),
        }
    }

    pub fn to_string(&self) -> &str {
        match self {
            Effect::SolidColour(..) => "Solid Colour",
            Effect::MovingPlane(..) => "Moving Plane",
            Effect::RainbowPlane(..) => "Rainbow Plane",
        }
    }
}

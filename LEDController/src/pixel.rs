use crate::colour::*;
use crate::vec3::Vec3;

use std::fmt;

#[derive(Copy, Clone, Debug)]
pub struct Pixel {
    pub colour: Colour,
    pub position: Vec3,
}

impl std::fmt::Display for Pixel {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} | {}", self.position, self.colour)
    }
}

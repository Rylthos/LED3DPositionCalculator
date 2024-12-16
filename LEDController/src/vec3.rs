use std::fmt;

#[derive(Copy, Clone, Debug)]
pub struct Vec3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl Vec3 {
    pub fn new(x: f32, y: f32, z: f32) -> Vec3 {
        Vec3 { x, y, z }
    }

    pub fn dot(a: Vec3, b: Vec3) -> f32 {
        a.x * b.x + a.y * b.y + a.z * b.z
    }

    pub fn mag_squared(a: Vec3) -> f32 {
        Vec3::dot(a, a)
    }

    pub fn mag(a: Vec3) -> f32 {
        f32::sqrt(Vec3::mag_squared(a))
    }

    pub fn norm(a: Vec3) -> Vec3 {
        let norm = 1. / Vec3::mag(a);
        Vec3 {
            x: a.x * norm,
            y: a.y * norm,
            z: a.z * norm,
        }
    }

    pub fn add(a: Vec3, b: Vec3) -> Vec3 {
        Vec3 {
            x: a.x + b.x,
            y: a.y + b.y,
            z: a.z + b.z,
        }
    }

    pub fn sub(a: Vec3, b: Vec3) -> Vec3 {
        Vec3 {
            x: a.x - b.x,
            y: a.y - b.y,
            z: a.z - b.z,
        }
    }

    pub fn mul_scalar(a: Vec3, b: f32) -> Vec3 {
        Vec3 {
            x: a.x * b,
            y: a.y * b,
            z: a.z * b,
        }
    }

    pub fn clamp_scalar(a: Vec3, low: f32, high: f32) -> Vec3 {
        Vec3 {
            x: f32::clamp(a.x, low, high),
            y: f32::clamp(a.y, low, high),
            z: f32::clamp(a.z, low, high),
        }
    }
}

impl std::fmt::Display for Vec3 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:.2} {:.2} {:.2}", self.x, self.y, self.z)
    }
}

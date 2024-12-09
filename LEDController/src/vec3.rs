#[derive(Copy, Clone, Debug)]
pub struct Vec3 {
    x: f32,
    y: f32,
    z: f32,
}

impl Vec3 {
    pub fn new(x: f32, y: f32, z: f32) -> Vec3 {
        Vec3 { x, y, z }
    }

    pub fn dot(a: Vec3, b: Vec3) -> f32 {
        a.x * b.x + a.y * b.y + a.z * b.z
    }

    pub fn mag(a: Vec3) -> f32 {
        Vec3::dot(a, a)
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
}

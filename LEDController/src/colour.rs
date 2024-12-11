#[derive(Copy, Clone, Debug)]
pub struct Colour {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

impl Colour {
    pub fn new(r: u8, g: u8, b: u8) -> Colour {
        Colour { r, g, b }
    }

    pub fn lerp(c1: Colour, c2: Colour, t: f32) -> Colour {
        let t1 = 1. - t;
        let t2 = t;
        let r = ((c1.r as f32) * t1 + (c2.r as f32) * t2)
            .clamp(0., 255.)
            .floor() as u8;
        let g = ((c1.r as f32) * t1 + (c2.r as f32) * t2)
            .clamp(0., 255.)
            .floor() as u8;
        let b = ((c1.r as f32) * t1 + (c2.r as f32) * t2)
            .clamp(0., 255.)
            .floor() as u8;
        Colour { r, g, b }
    }
}

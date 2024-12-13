use std::fmt;

#[derive(Copy, Clone, Debug)]
pub struct Colour {
    pub h: f32,
    pub s: f32,
    pub v: f32,
}

pub const WHITE: Colour = Colour {
    h: 0.,
    s: 0.,
    v: 1.,
};

pub const BLACK: Colour = Colour {
    h: 0.,
    s: 0.,
    v: 0.,
};

pub const RED: Colour = Colour {
    h: 0.,
    s: 1.,
    v: 1.,
};

pub const YELLOW: Colour = Colour {
    h: 60.,
    s: 1.,
    v: 1.,
};

pub const GREEN: Colour = Colour {
    h: 120.,
    s: 1.,
    v: 1.,
};

pub const CYAN: Colour = Colour {
    h: 180.,
    s: 1.,
    v: 1.,
};

pub const BLUE: Colour = Colour {
    h: 240.,
    s: 1.,
    v: 1.,
};

pub const PINK: Colour = Colour {
    h: 300.,
    s: 1.,
    v: 1.,
};

impl Colour {
    pub fn new(h: f32, s: f32, v: f32) -> Colour {
        Colour { h, s, v }
    }

    pub fn lerp(c1: Colour, c2: Colour, t: f32) -> Colour {
        let t1 = 1. - t;
        let t2 = t;
        let h = (c1.h * t1 + c2.h * t2).clamp(0., 360.);
        let s = (c1.s * t1 + c2.s * t2).clamp(0., 1.);
        let v = (c1.v * t1 + c2.v * t2).clamp(0., 1.);
        Colour { h, s, v }
    }

    pub fn to_rgb(hsv: &Colour) -> (u8, u8, u8) {
        let c = hsv.v * hsv.s;
        let h = (hsv.h % 360.) / 60.;
        let x = c * (1. - (h % 2. - 1.).abs());

        let (r1, g1, b1) = if h >= 0. && h < 1. {
            (c, x, 0.)
        } else if h >= 1. && h < 2. {
            (x, c, 0.)
        } else if h >= 2. && h < 3. {
            (0., c, x)
        } else if h >= 3. && h < 4. {
            (0., x, c)
        } else if h >= 4. && h < 5. {
            (x, 0., c)
        } else {
            (c, 0., x)
        };

        let m = hsv.v - c;
        let r = 255. * (r1 + m);
        let g = 255. * (g1 + m);
        let b = 255. * (b1 + m);

        (r.round() as u8, g.round() as u8, b.round() as u8)
    }
}

impl std::fmt::Display for Colour {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} {} {}", self.h, self.s, self.v)
    }
}

use crate::colour::*;
use crate::effect::Effect;
use crate::vec3::Vec3;

use ddp_rs::connection;

use regex::Regex;

use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

use std::fmt;

#[derive(Copy, Clone, Debug)]
struct Pixel {
    colour: Colour,
    position: Vec3,
}

impl std::fmt::Display for Pixel {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} | {}", self.position, self.colour)
    }
}

#[derive(Debug)]
pub struct PixelController {
    pixels: Vec<Pixel>,
    effect: Effect,
}

impl PixelController {
    pub fn new(num_pixels: usize) -> PixelController {
        let mut controller = PixelController {
            pixels: Vec::new(),
            effect: Effect::default_solid(),
        };

        controller.pixels.resize(
            num_pixels,
            Pixel {
                colour: BLACK,
                position: Vec3::new(0., 0., 0.),
            },
        );
        controller.read_pixels_from_file("Output.pixels");

        controller
    }

    pub fn get_num_pixels(&self) -> usize {
        self.pixels.len()
    }

    pub fn get_current_effect(&self) -> Effect {
        self.effect
    }

    pub fn next_effect(&mut self) {
        self.effect = Effect::change_effect(self.effect, 1);
    }

    pub fn prev_effect(&mut self) {
        self.effect = Effect::change_effect(self.effect, -1);
    }

    pub fn read_pixels_from_file(&mut self, file_name: &str) {
        let path = Path::new(file_name);
        let display = path.display();

        let mut file = match File::open(&path) {
            Err(err) => panic!("Couldn't open {}: {}", display, err),
            Ok(file) => file,
        };

        let mut s = String::new();

        if !file.read_to_string(&mut s).is_ok() {
            panic!("Couldn't read {}", file_name);
        }

        let parse_position = Regex::new(r"(\d+): (-?[0-9.]+) (-?[0-9.]+) (-?[0-9.]+)").unwrap();

        for line in s.split("\n") {
            if line.is_empty() {
                continue;
            }

            for (_, [id, x, y, z]) in parse_position.captures_iter(line).map(|s| s.extract()) {
                let id_num: usize = id.parse().unwrap();
                let x_num: f32 = x.parse().unwrap();
                let y_num: f32 = y.parse().unwrap();
                let z_num: f32 = z.parse().unwrap();
                self.pixels.get_mut(id_num).unwrap().position = Vec3::new(x_num, y_num, z_num);
            }
        }
    }

    pub fn transmit(&self, conn: &mut connection::DDPConnection) {
        let values = self.pixels_to_arr();

        // println!("Transmitting: {:?}", self.pixels);
        let temp = conn.write(values.as_slice());

        if temp.is_err() {
            panic!("Something went Wrong");
        }
    }

    pub fn update(&mut self, delta: f32) {
        match self.effect {
            Effect::SolidColour(c) => self.solid_colour(c),
            Effect::MovingPlane(p, n, c1, c2, d) => self.moving_plane(p, n, c1, c2, d, delta),
            Effect::RainbowPlane(p) => self.rainbow_plane(p, delta),
        }
    }

    fn pixels_to_arr(&self) -> Vec<u8> {
        let mut pixel_values: Vec<u8> = Vec::new();
        for p in self.pixels.clone() {
            let (r, g, b) = Colour::to_rgb(&p.colour);
            pixel_values.push(r);
            pixel_values.push(g);
            pixel_values.push(b);
        }
        pixel_values
    }

    fn solid_colour(&mut self, colour: Colour) {
        for pixel in self.pixels.iter_mut() {
            pixel.colour = colour;
        }
    }

    fn plane(&mut self, pos: Vec3, normal: Vec3, c1: Colour, c2: Colour, distance_coef: f32) {
        for pixel in self.pixels.iter_mut() {
            let new_position = Vec3::sub(pixel.position, pos);
            let mut distance = Vec3::dot(new_position, normal).abs() / Vec3::mag(normal);

            distance /= distance_coef;
            distance = ((distance + 1.) / 2.).clamp(0., 1.);

            pixel.colour = Colour::lerp(c1, c2, distance);
        }
    }

    fn moving_plane(
        &mut self,
        pos: Vec3,
        normal: Vec3,
        c1: Colour,
        c2: Colour,
        norm_mult: f32,
        delta: f32,
    ) {
        let direction_normal = Vec3::mul_scalar(normal, norm_mult);
        self.plane(pos, direction_normal, c1, c2, 100.);

        let movement_speed = 100.;
        let movement = movement_speed * delta;
        let mut new_pos = Vec3::new(
            pos.x + normal.x * movement,
            pos.y + normal.y * movement,
            pos.z + normal.z * movement,
        );

        let (normal, direction) = if new_pos.y > 410. || new_pos.y < 0. {
            new_pos = Vec3::clamp_scalar(new_pos, 0., 410.);
            (Vec3::mul_scalar(normal, -1.), norm_mult * -1.)
        } else {
            (normal, norm_mult)
        };

        self.effect = Effect::MovingPlane(new_pos, normal, c1, c2, direction);
    }

    fn rainbow_plane(&mut self, pos: Vec3, delta: f32) {
        let normal = Vec3::new(0., 1., 0.);

        let movement_speed = 100.;
        let movement = movement_speed * delta;
        let mut new_pos = Vec3::new(
            pos.x + normal.x * movement,
            pos.y + normal.y * movement,
            pos.z + normal.z * movement,
        );

        new_pos.y = new_pos.y % 720. + 360.;

        // if new_pos.y > 410. {
        //     new_pos.y = 0.;
        // }

        for pixel in self.pixels.iter_mut() {
            let new_position = Vec3::sub(pixel.position, pos);
            let distance = Vec3::dot(new_position, normal).abs() / Vec3::mag(normal);
            pixel.colour = Colour::new(distance, 1., 1.);
        }

        self.effect = Effect::RainbowPlane(new_pos);
    }
}

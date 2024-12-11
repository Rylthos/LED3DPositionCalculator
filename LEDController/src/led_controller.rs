use crate::colour::Colour;
use crate::effect::Effect;
use crate::vec3::Vec3;

use ddp_rs::connection;

use regex::Regex;

use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

#[derive(Copy, Clone, Debug)]
struct Pixel {
    colour: Colour,
    position: Vec3,
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
                colour: Colour { r: 0, g: 0, b: 0 },
                position: Vec3::new(0., 0., 0.),
            },
        );
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

        let parse_position = Regex::new(r"(\d+): ([0-9.]+) ([0-9.]+) ([0-9.]+)").unwrap();

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
            Effect::Plane(p, n, c1, c2, d) => self.plane(p, n, c1, c2, d),
            _ => (),
        }
    }

    fn pixels_to_arr(&self) -> Vec<u8> {
        let mut pixel_values: Vec<u8> = Vec::new();
        for p in self.pixels.clone() {
            pixel_values.push(p.colour.r);
            pixel_values.push(p.colour.g);
            pixel_values.push(p.colour.b);
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

            distance = if Vec3::dot(new_position, normal) < 0. {
                distance * -1.
            } else {
                distance
            };

            distance /= distance_coef;
            distance = ((distance + 1.) / 2.).clamp(0., 1.);

            pixel.colour = Colour::lerp(c1, c2, distance);
        }
    }
}

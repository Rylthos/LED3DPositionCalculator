use crate::colour::*;
use crate::effect::{
    effect_list::Effect, effect_trait::EffectTrait, rainbow_plane::RainbowPlaneEffect,
    solid_colour::SolidColourEffect,
};
use crate::pixel::Pixel;
use crate::vec3::Vec3;

use ddp_rs::connection;

use regex::Regex;

use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

pub struct PixelController {
    pixels: Vec<Pixel>,
    effect: Effect,
    max_brightness: f32,
}

impl PixelController {
    pub fn new(num_pixels: usize) -> PixelController {
        let mut controller = PixelController {
            pixels: Vec::new(),
            effect: Effect::SolidColour(SolidColourEffect::default()),
            max_brightness: 0.2,
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

    pub fn get_current_effect_mut(&mut self) -> &mut Effect {
        &mut self.effect
    }

    pub fn increase_brightness(&mut self) {
        self.max_brightness = (self.max_brightness + 0.05).clamp(0., 1.);
    }

    pub fn decrease_brightness(&mut self) {
        self.max_brightness = (self.max_brightness - 0.05).clamp(0., 1.);
    }

    pub fn get_brightness(&self) -> f32 {
        self.max_brightness
    }

    pub fn next_effect(&mut self) {
        self.effect.change_effect(1);
    }

    pub fn prev_effect(&mut self) {
        self.effect.change_effect(-1);
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

        let temp = conn.write(values.as_slice());

        if temp.is_err() {
            panic!("Something went Wrong");
        }
    }

    pub fn update(&mut self, delta: f32) {
        self.effect.render(&mut self.pixels);
        self.effect.update(delta, &self.pixels);
    }

    fn pixels_to_arr(&self) -> Vec<u8> {
        let mut pixel_values: Vec<u8> = Vec::new();
        for p in self.pixels.clone() {
            let mut colour = p.colour;
            colour.v = colour.v * self.max_brightness;
            let (r, g, b) = Colour::to_rgb(&colour);
            pixel_values.push(r);
            pixel_values.push(g);
            pixel_values.push(b);
        }
        pixel_values
    }
}

use std::{
    error::Error,
    fs::{File, OpenOptions},
    io::Write,
};

use crate::{vec3::*, Config};

pub struct Output {
    file: File,
}

impl Output {
    pub fn new(path: &String) -> Result<Output, Box<dyn Error>> {
        let file = OpenOptions::new().write(true).create(true).open(path)?;

        Ok(Output { file })
    }

    pub fn initial(&mut self, config: &Config) -> Result<(), Box<dyn Error>> {
        self.file.write_fmt(format_args!(
            "P3\n{} {}\n255\n",
            config.image_width, config.image_height,
        ))?;

        Ok(())
    }

    pub fn output_color(&mut self, pixel_color: &Color) -> Result<(), Box<dyn Error>> {
        let (r, g, b) = output_color_as_u8(pixel_color);

        self.file.write_fmt(format_args!("{r} {g} {b}\n"))?;

        Ok(())
    }
}

pub fn output_color_as_u8(pixel_color: &Color) -> (u8, u8, u8) {
    (
        (255.999f64 * pixel_color.0) as u8,
        (255.999f64 * pixel_color.1) as u8,
        (255.999f64 * pixel_color.2) as u8,
    )
}

pub fn output_color_as_u16(pixel_color: &Color) -> (u16, u16, u16) {
    (
        (255.999f64 * pixel_color.0) as u16,
        (255.999f64 * pixel_color.1) as u16,
        (255.999f64 * pixel_color.2) as u16,
    )
}

pub fn output_color_as_u32(pixel_color: &Color) -> (u32, u32, u32) {
    (
        (255.999f64 * pixel_color.0) as u32,
        (255.999f64 * pixel_color.1) as u32,
        (255.999f64 * pixel_color.2) as u32,
    )
}

pub fn output_color_as_u64(pixel_color: &Color) -> (u64, u64, u64) {
    (
        (255.999f64 * pixel_color.0) as u64,
        (255.999f64 * pixel_color.1) as u64,
        (255.999f64 * pixel_color.2) as u64,
    )
}

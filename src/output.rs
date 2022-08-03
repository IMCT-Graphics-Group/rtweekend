use std::{
    error::Error,
    fs::{File, OpenOptions},
    io::Write,
};

use crate::{vec3::*, Config};

pub struct Output<'a> {
    file: File,
    config: &'a Config,
}

impl<'a> Output<'a> {
    pub fn new(config: &'a Config) -> Result<Output<'a>, Box<dyn Error>> {
        let file = OpenOptions::new()
            .write(true)
            .create(true)
            .open(&config.filename)?;

        Ok(Output { file, config })
    }

    pub fn initial(&mut self) -> Result<(), Box<dyn Error>> {
        self.file.write_fmt(format_args!(
            "P3\n{} {}\n255\n",
            self.config.image_width, self.config.image_height,
        ))?;

        Ok(())
    }

    pub fn output_color(&mut self, mut pixel_color: Color) -> Result<(), Box<dyn Error>> {
        pixel_color /= self.config.samples_per_pixel as f64;

        let (r, g, b) = output_color_as_u8(gamma_correct(pixel_color));

        self.file.write_fmt(format_args!("{r} {g} {b}\n"))?;

        Ok(())
    }
}

fn gamma_correct(pixel_color: Color) -> Color {
    Color::new_color(
        f64::sqrt(pixel_color.0),
        f64::sqrt(pixel_color.1),
        f64::sqrt(pixel_color.2),
    )
}

fn output_color_as_u8(pixel_color: Color) -> (u8, u8, u8) {
    (
        (256.0 * pixel_color.0) as u8,
        (256.0 * pixel_color.1) as u8,
        (256.0 * pixel_color.2) as u8,
    )
}

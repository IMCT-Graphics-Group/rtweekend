use core::panic;
use std::{
    error::Error,
    fs::{File, OpenOptions},
    io::Write,
    ops::Div, fmt::write,
};

use crate::{vec3::*, Config};
use stb_image_write_rust::ImageWriter::ImageWriter;

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

    pub fn output_color(&mut self, pixel_color: &Color) -> Result<(), Box<dyn Error>> {
        let (r, g, b) =
            output_color_as_u8(&(pixel_color.div(self.config.samples_per_pixel as f64)));

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

//by default our component size is 3
pub struct ImageStream {
    width : i32,
    height : i32,
    data   : Vec<u8>,
    spp    : i32
}

impl ImageStream {
    pub fn new(config : &Config)->ImageStream{
        ImageStream 
        { 
            width: config.image_width, 
            height: config.image_height, 
            data: Vec::new(),
            spp : config.samples_per_pixel
        }
    }

    pub fn output_color(&mut self,color : &Color){
        if self.data.len()  >= (self.width * self.height * 3) as usize {
            panic!("ImageStream::output_color : image out of bondary");
        }
        let (r,g,b) = output_color_as_u8(&color.div(self.spp as f64));
        self.data.push(r);
        self.data.push(g);
        self.data.push(b);
    }

    pub fn save_png(&self,path:&str){
        if self.data.len() != (self.width * self.height * 3) as usize{
            panic!("ImageStream::save : not enough data have written to image stream yet");
        }
        let mut write_path = String::from(path);
        match path.find(".png") {
            Some(_) => (),
            None => {
                write_path.push_str(".png");
            }
        };
        let mut writer = ImageWriter::new(write_path.as_str());
        writer.write_png(self.width, self.height, 3, self.data.as_ptr());

    }
}



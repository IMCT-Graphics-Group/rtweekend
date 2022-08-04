use crate::*;

use stb_image_write_rust::ImageWriter::ImageWriter;

pub struct ImageStream<'a> {
    config: &'a Config,
    data: Vec<u8>,
}

impl<'a> ImageStream<'a> {
    pub fn new(config: &'a Config) -> ImageStream<'a> {
        ImageStream {
            config,
            data: Vec::with_capacity((config.image_width * config.image_height * 3) as usize),
        }
    }

    pub fn write_color(&mut self, pixel_color: Color) {
        convert_color_to_u8(gamma_correct(pixel_color)).map(|channel| self.data.push(channel));
    }

    pub fn save_png(&self) {
        if self.data.len() > (self.config.image_width * self.config.image_height * 3) as usize {
            panic!("ImageStream::save : not enough data have written to image stream yet");
        }

        let mut out_path = self.config.file_path.clone();
        match out_path.find(".png") {
            Some(_) => (),
            None => {
                out_path.push_str(".png");
            }
        };

        let mut writer = ImageWriter::new(out_path.as_str());
        writer.write_png(
            self.config.image_width,
            self.config.image_height,
            3,
            self.data.as_ptr(),
        );
    }
}

fn gamma_correct(pixel_color: Color) -> Color {
    Color::new_color(
        f64::sqrt(pixel_color.0),
        f64::sqrt(pixel_color.1),
        f64::sqrt(pixel_color.2),
    )
}

fn convert_color_to_u8(pixel_color: Color) -> [u8; 3] {
    [
        (256.0 * pixel_color.0) as u8,
        (256.0 * pixel_color.1) as u8,
        (256.0 * pixel_color.2) as u8,
    ]
}

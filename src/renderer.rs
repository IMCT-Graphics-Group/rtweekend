use std::sync::mpsc::{channel, Receiver, Sender};

use crate::*;

use image::{Rgb, RgbImage};
use threadpool::ThreadPool;

pub struct Renderer {
    config: ConfigType,
    image: RgbImage,
    pool: ThreadPool,
    transmitter: Sender<(u32, u32, Rgb<u8>)>,
    receiver: Receiver<(u32, u32, Rgb<u8>)>,
}

impl<'a> Renderer {
    pub fn new(config: ConfigType) -> Renderer {
        let (transmitter, receiver) = channel();
        Renderer {
            config: config.clone(),
            image: RgbImage::new(config.image_width as u32, config.image_height as u32),
            pool: ThreadPool::new(num_cpus::get()),
            transmitter,
            receiver,
        }
    }

    pub fn threadpool(&self) -> &ThreadPool {
        &self.pool
    }

    pub fn sender(&self) -> Sender<(u32, u32, Rgb<u8>)> {
        self.transmitter.clone()
    }

    pub fn save_png(&mut self) {
        for _ in 0..(self.image.width() * self.image.height()) {
            let (x, y, pixel) = self.receiver.recv().unwrap();
            self.image.put_pixel(x, y, pixel);
        }

        self.image.save(self.config.file_path.clone()).unwrap();
    }
}

pub fn gamma_correct(pixel_color: Color) -> Rgb<u8> {
    let color = Color::new_color(
        f64::sqrt(pixel_color.0),
        f64::sqrt(pixel_color.1),
        f64::sqrt(pixel_color.2),
    );

    Rgb(convert_color_to_u8(color))
}

fn convert_color_to_u8(pixel_color: Color) -> [u8; 3] {
    [
        (256.0 * pixel_color.0) as u8,
        (256.0 * pixel_color.1) as u8,
        (256.0 * pixel_color.2) as u8,
    ]
}

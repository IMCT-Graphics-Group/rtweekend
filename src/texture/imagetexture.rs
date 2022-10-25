use crate::*;
use image::{Rgb, RgbImage};

pub struct ImageTexture{
    width:u32,
    height:u32,
    rgb_image: RgbImage,
}

impl ImageTexture {
    pub fn new_from_file(filename: String) -> ImageTexture{
        let img = image::open(filename).unwrap();
        let rgb_image:RgbImage = img.into_rgb8();
        let width = rgb_image.width();
        let height = rgb_image.height();
        ImageTexture { width, height, rgb_image }
    }
}

impl Texture for ImageTexture {
    fn value(&self, u:f64,v:f64, p:&Point3) -> Color {
        let u = u.clamp(0.0, 1.0);
        let v = 1.0 - v.clamp(0.0, 1.0);

        let mut i = (u * self.width as f64)as u32;
        let mut j = (v * self.height as f64)as u32;

        //println!("i is {},j is {}",i,j);

        if i >= self.width{
            i = self.width - 1;
        }
        if j >= self.height{
            j = self.height - 1;
        }

        let color_scale = 1.0/255.0;
        let pixel = self.rgb_image.get_pixel(i, j);

        Color::new_color(
            color_scale*pixel[0] as f64, 
            color_scale*pixel[1] as f64, 
            color_scale*pixel[2] as f64
        )
    }
}
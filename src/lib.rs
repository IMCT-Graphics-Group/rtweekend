mod color;
mod ray;
mod vec3;

pub use crate::color::*;
pub use crate::ray::*;
pub use crate::vec3::*;

use std::{error::Error, fs::OpenOptions, io::Write};

#[derive(Debug)]
pub struct Config {
    pub filename: String,
    pub image_width: i32,
    pub image_height: i32,
    pub aspect_ratio: f64,

    pub viewport_width: f64,
    pub viewport_height: f64,
    pub focal_length: f64,

    pub origin: Point3,
    pub horizontal: Vec3,
    pub vertical: Vec3,
    pub lower_left_corner: Vec3,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            filename: String::from("image.ppm"),
            image_width: 400,
            image_height: Default::default(),
            aspect_ratio: 16.0 / 9.0,
            viewport_width: Default::default(),
            viewport_height: 2.0,
            focal_length: 1.0,
            origin: Vec3(0.0, 0.0, 0.0),
            horizontal: Default::default(),
            vertical: Default::default(),
            lower_left_corner: Default::default(),
        }
    }
}

impl Config {
    pub fn new() -> Config {
        let mut config = Config::default();
        config.image_height = ((config.image_width as f64) / config.aspect_ratio) as i32;
        config.viewport_width = config.aspect_ratio * config.viewport_height;
        config.horizontal = Vec3(config.viewport_width, 0.0, 0.0);
        config.vertical = Vec3(0.0, config.viewport_height, 0.0);
        config.lower_left_corner = config.origin
            - config.horizontal / 2.0
            - config.vertical / 2.0
            - Vec3(0.0, 0.0, config.focal_length);
        config
    }
}

fn ray_color(ray: &Ray) -> Color {
    let unit_direction = ray.direction().unit_vector();
    let t = 0.5 * (unit_direction.y() + 1.0);
    return Color::new_color(1.0, 1.0, 1.0) * (1.0 - t) + Color::new_color(0.5, 0.7, 1.0) * t;
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let mut file = OpenOptions::new()
        .write(true)
        .create(true)
        .open(config.filename)?;

    file.write_fmt(format_args!(
        "P3\n{} {}\n255\n",
        config.image_width, config.image_height,
    ))?;

    for j in (0..=config.image_height - 1).rev() {
        print!("\x1b[2J");
        print!("\x1b[H");
        print!("\rScanlines remaining: {j}");

        for i in 0..config.image_width {

            let (u, v) = (
                (i as f64) / (config.image_width - 1) as f64,
                (j as f64) / (config.image_height - 1) as f64,
            );

            let r = Ray::new(
                config.origin,
                config.lower_left_corner + config.horizontal * u + config.vertical * v
                    - config.origin,
            );

            let pixel_color = ray_color(&r);
            
            let (r, g, b) = output_color_as_u8(&pixel_color);

            file.write_fmt(format_args!("{r} {g} {b}\n"))?;
        }
    }

    println!("\nDone.");

    Ok(())
}

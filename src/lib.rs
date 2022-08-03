mod geometry;
mod output;
mod ray;
mod utils;
mod vec3;

pub use crate::geometry::*;
pub use crate::output::*;
pub use crate::ray::*;
pub use crate::utils::*;
pub use crate::vec3::*;

use std::error::Error;

pub struct Config {
    pub filename: String,
    pub image_width: i32,
    pub image_height: i32,
    pub aspect_ratio: f64,
    pub samples_per_pixel: i32,
    pub ray_depth: i32,

    pub viewport_width: f64,
    pub viewport_height: f64,
    pub focal_length: f64,

    pub origin: Point3,
    pub horizontal: Vec3,
    pub vertical: Vec3,
    pub lower_left_corner: Vec3,

    pub scene: Scene,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            filename: String::from("image.ppm"),
            image_width: 400,
            image_height: Default::default(),
            aspect_ratio: 16.0 / 9.0,
            samples_per_pixel: 100,
            ray_depth: 50,
            viewport_width: Default::default(),
            viewport_height: 2.0,
            focal_length: 1.0,
            origin: Vec3(0.0, 0.0, 0.0),
            horizontal: Default::default(),
            vertical: Default::default(),
            lower_left_corner: Default::default(),
            scene: Default::default(),
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
        config.scene = initial_scene();
        config
    }

    fn ray_dierction_from_uv(&self, u: f64, v: f64) -> Vec3 {
        self.lower_left_corner + self.horizontal * u + self.vertical * v - self.origin
    }
}

fn initial_scene() -> Scene {
    let mut scene = Scene::new();

    scene.add_object(Sphere::new(Point3::new_point3(0.0, 0.0, -1.0), 0.5));
    scene.add_object(Sphere::new(Point3::new_point3(0.0, -100.5, -1.0), 100.0));

    scene
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let mut out_stream = Output::new(&config)?;

    out_stream.initial()?;

    for j in (0..=config.image_height - 1).rev() {
        print!("\x1b[2J");
        print!("\x1b[H");
        print!("\rScanlines remaining: {j}");

        for i in 0..config.image_width {
            let mut pixel_color = Color::new_color(0.0, 0.0, 0.0);
            for _ in 0..config.samples_per_pixel {
                let (u, v) = (
                    (i as f64 + random_01()) / (config.image_width - 1) as f64,
                    (j as f64 + random_01()) / (config.image_height - 1) as f64,
                );

                let ray = Ray::new(
                    config.origin,
                    config.ray_dierction_from_uv(u, v),
                    config.ray_depth,
                );

                pixel_color += ray_color(&ray, &config);
            }
            out_stream.output_color(pixel_color)?;
        }
    }

    println!("\nDone.");

    Ok(())
}

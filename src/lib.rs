mod camera;
mod config;
mod geometry;
mod hittable;
mod material;
mod ray;
mod renderer;
mod scene;
mod utils;
mod vec3;

pub use crate::camera::*;
pub use crate::config::*;
pub use crate::geometry::aabb::*;
pub use crate::geometry::bvh::*;
pub use crate::geometry::sphere::*;
pub use crate::hittable::*;
pub use crate::material::dielectric::*;
pub use crate::material::lambertian::*;
pub use crate::material::metal::*;
pub use crate::material::*;
pub use crate::ray::*;
pub use crate::renderer::*;
pub use crate::scene::*;
pub use crate::utils::*;
pub use crate::vec3::*;

use std::error::Error;
use std::sync::Arc;

pub type MaterialType = Arc<Box<dyn Material + Send + Sync>>;

pub type ObjectType = Arc<Box<dyn Bounded + Send + Sync>>;

pub type ConfigType = Arc<Box<Config>>;

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let config = Arc::new(Box::new(config));
    let mut renderer = Renderer::new(config.clone());

    for j in (0..=config.image_height - 1).rev() {
        print!("\r{j} scanlines remaining.");

        for i in 0..config.image_width {
            let mut pixel_color = Color::new_color(0.0, 0.0, 0.0);
            let sender = renderer.sender();
            let config = config.clone();

            renderer.threadpool().execute(move || {
                for _ in 0..config.samples_per_pixel {
                    let (u, v) = (
                        (i as f64 + random_01()) / (config.image_width - 1) as f64,
                        (j as f64 + random_01()) / (config.image_height - 1) as f64,
                    );

                    let ray = config.camera.get_ray_upper_left(u, v);

                    pixel_color += ray_color(ray, &config);
                }

                let pixel = gamma_correct(pixel_color / config.samples_per_pixel as f64);
                sender.send((i, j, pixel)).expect("Could not send pixel");
            });
        }
    }

    renderer.save_png();
    println!("\nDone.");

    Ok(())
}

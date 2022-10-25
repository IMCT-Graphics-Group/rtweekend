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
mod transform;
mod texture;
mod onb;
mod pdf;

pub use crate::camera::*;
pub use crate::config::*;
pub use crate::geometry::aabb::*;
pub use crate::geometry::bvh::*;
pub use crate::geometry::sphere::*;
pub use crate::geometry::aarect::*;
pub use crate::geometry::mybox::*;
pub use crate::geometry::constantmedium::*;
pub use crate::geometry::triangle::*;
pub use crate::hittable::*;
pub use crate::material::dielectric::*;
pub use crate::material::lambertian::*;
pub use crate::material::metal::*;
pub use crate::material::diffuselight::*;
pub use crate::material::isotropic::*;
pub use crate::material::*;
pub use crate::ray::*;
pub use crate::renderer::*;
pub use crate::scene::*;
pub use crate::utils::*;
pub use crate::vec3::*;
pub use crate::transform::*;
pub use crate::texture::*;
pub use crate::texture::solidcolor::*;
pub use crate::texture::checker::*;
pub use crate::texture::imagetexture::*;
pub use crate::onb::*;
pub use crate::pdf::*;

use std::error::Error;
use std::sync::Arc;

pub type MaterialType = Arc<Box<dyn Material + Send + Sync>>;

pub type ObjectType = Arc<Box<dyn Bounded + Send + Sync>>;

pub type TextureType = Arc<Box<dyn Texture + Send + Sync>>;

pub type HittableType = Arc<Box<dyn Hittable + Send + Sync>>;

pub type ConfigType = Arc<Box<Config>>;

pub type Float = f64;

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {

    let config = Arc::new(Box::new(config));
    let mut renderer = Renderer::new(config.clone());

    let light:MaterialType = 
        Arc::new(Box::new(DiffuseLight::new_color(Color::new_color(6.0, 6.0, 6.0))));

    let lights: HittableType = Arc::new(Box::new(XZrect::new(
        -1.0,1.0,-2.0,0.0,3.0,light.clone()
    )));

    println!("Running...");
    for j in (0..=config.image_height - 1).rev() {
        for i in 0..config.image_width {
            let mut pixel_color = Color::new_color(0.0, 0.0, 0.0);
            let sender = renderer.sender();
            let config = config.clone();
            let lights = lights.clone();

            renderer.threadpool().execute(move || {
                for _ in 0..config.samples_per_pixel {
                    let (u, v) = (
                        (i as f64 + random_01()) / (config.image_width - 1) as f64,
                        (j as f64 + random_01()) / (config.image_height - 1) as f64,
                    );

                    let ray = config.camera.get_ray_upper_left(u, v);

                    pixel_color += ray_color(ray, &config, lights.clone());
                }

                let pixel = gamma_correct(pixel_color / config.samples_per_pixel as f64);
                sender.send((i, j, pixel)).expect("Could not send pixel");
            });
        }
    }

    renderer.save_png();
    println!("Done.");

    Ok(())
}

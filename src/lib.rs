mod config;
mod geometry;
mod hittable;
mod imagestream;
mod material;
mod ray;
mod scene;
mod utils;
mod vec3;

use material::dielectric::Dielectric;

pub use crate::config::*;
pub use crate::geometry::sphere::*;
pub use crate::hittable::*;
pub use crate::imagestream::*;
pub use crate::material::lambertian::*;
pub use crate::material::metal::*;
pub use crate::material::*;
pub use crate::ray::*;
pub use crate::scene::*;
pub use crate::utils::*;
pub use crate::vec3::*;

use std::error::Error;
use std::rc::Rc;

fn initial_scene() -> Scene {
    let mut scene = Scene::new();

    let material_ground: Rc<Box<dyn Material>> =
        Rc::new(Box::new(Lambertian::new(Color::new_color(0.8, 0.8, 0.0))));
    let material_center: Rc<Box<dyn Material>> =
        Rc::new(Box::new(Lambertian::new(Color::new_color(0.1, 0.2, 0.5))));
    let material_left: Rc<Box<dyn Material>> = Rc::new(Box::new(Dielectric::new(1.5)));
    let material_right: Rc<Box<dyn Material>> =
        Rc::new(Box::new(Metal::new(Color::new_color(0.8, 0.6, 0.2), 0.0)));

    //center_sphere
    scene.add_object(Sphere::new(
        Point3::new_point3(0.0, 0.0, -1.0),
        0.5,
        material_center.clone(),
    ));

    //ground_sphere
    scene.add_object(Sphere::new(
        Point3::new_point3(0.0, -100.5, -1.0),
        100.0,
        material_ground.clone(),
    ));

    //left_sphere
    scene.add_object(Sphere::new(
        Point3::new_point3(-1.0, 0.0, -1.0),
        0.5,
        material_left.clone(),
    ));

    //left_inner_sphere
    scene.add_object(Sphere::new(
        Point3::new_point3(-1.0, 0.0, -1.0),
        -0.4,
        material_left.clone(),
    ));

    //right_sphere
    scene.add_object(Sphere::new(
        Point3::new_point3(1.0, 0.0, -1.0),
        0.5,
        material_right.clone(),
    ));

    scene
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let mut image_stream = ImageStream::new(&config);

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

                pixel_color += ray_color(ray, &config);
            }
            image_stream.write_color(pixel_color / config.samples_per_pixel as f64);
        }
    }

    image_stream.save_png();
    println!("\nDone.");

    Ok(())
}

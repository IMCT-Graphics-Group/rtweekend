pub mod lambertian;
pub mod metal;
pub mod dielectric;

use crate::*;

pub trait Material {
    fn scatter(&self, ray_in: Ray, hit_record: &HitRecord) -> Option<(Ray, Color)>;
}

#[derive(Default)]
pub struct Materials {
    pub materials: Vec<Box<dyn Material>>,
}

impl Materials {
    pub fn new() -> Materials {
        Materials::default()
    }

    pub fn add_material<T: Material + 'static>(&mut self, material: T) {
        self.materials.push(Box::new(material));
    }
}

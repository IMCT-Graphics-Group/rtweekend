pub mod dielectric;
pub mod lambertian;
pub mod metal;

use crate::*;

pub trait Material {
    fn scatter(&self, ray_in: Ray, hit_record: &HitRecord) -> Option<(Ray, Color)>;
}

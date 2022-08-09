use std::fmt::Display;

use crate::*;

pub trait Hittable {
    fn hit(&self, ray: &Ray, t_range: (f64, f64)) -> Option<HitRecord>;
}

pub struct HitRecord {
    pub hit_point: Point3,
    pub hit_normal: Vec3,
    pub hit_material: MaterialType,
    pub t: f64,
    pub front_face: bool,
}

impl HitRecord {
    pub fn new(
        hit_point: Vec3,
        hit_normal: Vec3,
        hit_material: MaterialType,
        t: f64,
        front_face: bool,
    ) -> HitRecord {
        HitRecord {
            hit_point,
            hit_normal,
            hit_material,
            t,
            front_face,
        }
    }
}

impl Display for HitRecord {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Hit_point:{}\nHit_normal:{}\nt:{}\n",
            self.hit_point, self.hit_normal, self.t
        )
    }
}

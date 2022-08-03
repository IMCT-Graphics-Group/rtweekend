use crate::*;

use std::rc::Rc;

pub trait Hittable {
    fn hit(&self, ray: &Ray, t_range: (f64, f64)) -> Option<HitRecord>;
}

pub struct HitRecord {
    pub hit_point: Point3,
    pub hit_normal: Vec3,
    pub hit_material: Rc<Box<dyn Material>>,
    pub t: f64,
}

impl HitRecord {
    pub fn new(
        hit_point: Vec3,
        hit_normal: Vec3,
        hit_material: Rc<Box<dyn Material>>,
        t: f64,
    ) -> HitRecord {
        HitRecord {
            hit_point,
            hit_normal,
            hit_material,
            t,
        }
    }
}

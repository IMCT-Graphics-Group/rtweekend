use std::ops::Add;

use crate::*;

pub trait Bounded: Hittable {
    fn bounding_box(&self) -> AABB;
}

#[derive(Default, Clone, Copy)]
pub struct AABB {
    min: Point3,
    max: Point3,
}

impl AABB {
    pub fn new(min: Point3, max: Point3) -> AABB {
        AABB { min, max }
    }

    pub fn min(&self) -> Point3 {
        self.min
    }

    pub fn max(&self) -> Point3 {
        self.max
    }

    pub fn hit(&self, ray: &Ray, t_range: (f64, f64)) -> Option<(f64, f64)> {
        let (mut t0, mut t1) = t_range;
        for i in 0..3 {
            let inv_dir = 1.0 / ray.dir.get(i);
            let mut _t0 = (self.min.get(i) - ray.orig.get(i)) * inv_dir;
            let mut _t1 = (self.max.get(i) - ray.orig.get(i)) * inv_dir;
            if inv_dir < 0.0 {
                (_t0, _t1) = (_t1, _t0);
            }
            if t_range.1.min(_t1) <= t_range.0.max(_t0) {
                return Option::None;
            }
            (t0, t1) = (t0.max(_t0), t1.min(_t1));
        }
        return Option::Some((t0, t1));
    }
}

impl Add for AABB {
    type Output = AABB;

    fn add(self, rhs: Self) -> Self::Output {
        AABB {
            min: Vec3::from_vec(
                self.min
                    .array()
                    .iter()
                    .zip(rhs.min.array().iter())
                    .map(|(left, right)| left.min(*right))
                    .collect(),
            ),
            max: Vec3::from_vec(
                self.max
                    .array()
                    .iter()
                    .zip(rhs.max.array().iter())
                    .map(|(left, right)| left.max(*right))
                    .collect(),
            ),
        }
    }
}

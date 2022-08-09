use crate::*;

pub struct Sphere {
    center: Point3,
    radius: f64,
    material: MaterialType,
}

impl Sphere {
    pub fn new(center: Point3, radius: f64, material: MaterialType) -> Sphere {
        Sphere {
            center,
            radius,
            material,
        }
    }
}

impl Hittable for Sphere {
    fn hit(&self, ray: &Ray, t_range: (f64, f64)) -> Option<HitRecord> {
        let oc = ray.orig - self.center;
        let a = ray.dir.length_squared();
        let half_b = Vec3::dot(oc, ray.dir);
        let c = oc.length_squared() - self.radius * self.radius;

        let discriminant = half_b * half_b - a * c;
        if discriminant < 0.0 {
            return Option::None;
        }

        let sqrtd = f64::sqrt(discriminant);
        let mut root = (-half_b - sqrtd) / a;
        if root < t_range.0 || t_range.1 < root {
            root = (-half_b + sqrtd) / a;
            if root < t_range.0 || t_range.1 < root {
                return Option::None;
            }
        }

        let hit_point = ray.at(root);
        let outward_normal = (hit_point - self.center) / self.radius;
        let (front_face, hit_normal) = Vec3::set_face_normal(ray.dir, outward_normal);

        Option::Some(HitRecord::new(
            ray.at(root),
            hit_normal,
            self.material.clone(),
            root,
            front_face,
        ))
    }
}

impl Bounded for Sphere {
    fn bounding_box(&self) -> AABB {
        AABB::new(
            self.center - Vec3(self.radius, self.radius, self.radius),
            self.center + Vec3(self.radius, self.radius, self.radius),
        )
    }
}

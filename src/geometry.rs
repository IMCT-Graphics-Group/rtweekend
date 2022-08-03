use crate::ray::*;
use crate::vec3::*;

#[derive(Debug, Default)]
pub struct Sphere {
    center: Point3,
    radius: f64,
}

#[derive(Default)]
pub struct Scene {
    pub objects: Vec<Box<dyn Hittable>>,
}

impl Sphere {
    pub fn new(center: Point3, radius: f64) -> Sphere {
        Sphere { center, radius }
    }
}

impl Hittable for Sphere {
    fn hit(&self, ray: &Ray, t_range: (f64, f64)) -> Option<HitRecord> {
        let mut hit_record = HitRecord::default();

        let oc = ray.origin() - self.center;
        let a = ray.direction().length_squared();
        let half_b = Vec3::dot(oc, ray.direction());
        let c = oc.length_squared() - self.radius * self.radius;

        let discriminant = half_b * half_b - a * c;
        if discriminant < 0.0 {
            return Option::None;
        }

        let sqrtd = f64::sqrt(discriminant);
        let root = (-half_b - sqrtd) / a;
        if root < t_range.0 || t_range.1 < root {
            let root = (-half_b + sqrtd) / a;
            if root < t_range.0 || t_range.1 < root {
                return Option::None;
            }
        }

        hit_record.t = root;
        hit_record.hit_point = ray.at(root);
        let outward_normal = (hit_record.hit_point - self.center) / self.radius;
        hit_record.set_face_normal(ray, outward_normal);

        return Option::Some(hit_record);
    }
}

impl Scene {
    pub fn new() -> Scene {
        Default::default()
    }

    pub fn add_object<T: Hittable + 'static>(&mut self, object: T) {
        self.objects.push(Box::new(object));
    }

    pub fn hit(&self, ray: &Ray, t_range: (f64, f64)) -> Option<HitRecord> {
        let mut result = Option::None;
        let mut closest_so_far = t_range.1;

        for object in self.objects.iter() {
            if let Some(record) = object.hit(ray, (t_range.0, closest_so_far)) {
                closest_so_far = record.t;
                result = Some(record);
            }
        }

        result
    }
}

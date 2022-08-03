use crate::{utils::*, vec3::*, Config};

#[derive(Debug, Default)]
pub struct Ray {
    orig: Point3,
    dir: Vec3,
    depth: i32,
}

#[derive(Debug, Default, Clone)]
pub struct HitRecord {
    pub hit_point: Point3,
    pub hit_normal: Vec3,
    pub t: f64,
    pub is_front_face: bool,
}

pub trait Hittable {
    fn hit(&self, ray: &Ray, t_range: (f64, f64)) -> Option<HitRecord>;
}

impl Ray {
    pub fn new(orig: Point3, dir: Vec3, depth: i32) -> Ray {
        Ray { orig, dir, depth }
    }

    pub fn origin(&self) -> Point3 {
        self.orig
    }

    pub fn direction(&self) -> Vec3 {
        self.dir
    }

    pub fn at(&self, t: f64) -> Point3 {
        self.orig + self.dir * t
    }
}

impl HitRecord {
    pub fn set_face_normal(&mut self, ray: &Ray, outward_normal: Vec3) {
        self.is_front_face = Vec3::dot(ray.direction(), outward_normal) < 0.0;
        self.hit_normal = match self.is_front_face {
            true => outward_normal,
            false => outward_normal * -1.0,
        }
    }
}

pub fn hit_sphere(center: Point3, radius: f64, ray: &Ray) -> f64 {
    let oc = ray.origin() - center;
    let a = ray.direction().length_squared();
    let half_b = Vec3::dot(oc, ray.direction());
    let c = oc.length_squared() - radius * radius;
    let discriminant = half_b * half_b - a * c;
    if discriminant < 0.0 {
        -1.0
    } else {
        (-half_b - f64::sqrt(discriminant)) / a
    }
}

pub fn ray_color(ray: &Ray, config: &Config) -> Color {
    if ray.depth <= 0 {
        return Color::new_color(0.0, 0.0, 0.0);
    }

    if let Some(hit_record) = config.scene.hit(ray, (f64::MIN_POSITIVE, f64::INFINITY)) {
        return ray_color(
            &Ray::new(
                hit_record.hit_point,
                random_hemisphere(hit_record.hit_normal),
                ray.depth - 1,
            ),
            config,
        ) * 0.5;
    }

    let unit_direction = ray.direction().unit_vector();
    let t = 0.5 * (unit_direction.y() + 1.0);
    return Color::new_color(1.0, 1.0, 1.0) * (1.0 - t) + Color::new_color(0.5, 0.7, 1.0) * t;
}

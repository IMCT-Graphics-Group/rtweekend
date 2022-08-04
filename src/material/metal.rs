use crate::*;

#[derive(Default)]
pub struct Metal {
    albedo: Color,
    fuzz: f64,
}

impl Metal {
    pub fn new(albedo: Color, fuzz: f64) -> Metal {
        Metal {
            albedo,
            fuzz: {
                if fuzz < 1.0 {
                    fuzz
                } else {
                    1.0
                }
            },
        }
    }
}

impl Material for Metal {
    fn scatter(&self, ray_in: Ray, hit_record: &HitRecord) -> Option<(Ray, Color)> {
        let scattered = Ray::new(
            hit_record.hit_point,
            Vec3::reflect(ray_in.dir.unit_vector(), hit_record.hit_normal)
                + random_unit_sphere() * self.fuzz,
            ray_in.depth - 1,
        );

        match Vec3::dot(scattered.dir, hit_record.hit_normal) > 0.0 {
            true => Option::Some((scattered, self.albedo)),
            false => Option::None,
        }
    }
}

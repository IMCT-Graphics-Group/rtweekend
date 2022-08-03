use crate::*;

#[derive(Default)]
pub struct Lambertian {
    albedo: Color,
}

impl Lambertian {
    pub fn new(albedo: Color) -> Lambertian {
        Lambertian { albedo }
    }
}

impl Material for Lambertian {
    fn scatter(&self, ray_in: Ray, hit_record: &HitRecord) -> Option<(Ray, Color)> {
        let mut scattered = Ray::new(
            hit_record.hit_point,
            random_hemisphere(hit_record.hit_normal),
            ray_in.depth - 1,
        );

        if scattered.dir.near_zero() {
            scattered.dir = hit_record.hit_normal;
        }

        Option::Some((scattered, self.albedo))
    }
}

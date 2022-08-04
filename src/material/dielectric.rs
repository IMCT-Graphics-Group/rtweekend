use crate::*;

#[derive(Default)]
pub struct Dielectric {
    ior: f64,
}

impl Dielectric {
    pub fn new(ior: f64) -> Dielectric {
        Dielectric { ior }
    }
}

impl Material for Dielectric {
    fn scatter(&self, ray_in: Ray, hit_record: &HitRecord) -> Option<(Ray, Color)> {
        let attenuation = Color::new_color(1.0, 1.0, 1.0);
        let refraction_ratio = match hit_record.front_face {
            true => 1.0 / self.ior,
            false => self.ior,
        };

        let unit_direction = ray_in.dir.unit_vector();

        let scattered = Ray::new(
            hit_record.hit_point,
            match is_reflect(unit_direction, hit_record.hit_normal, refraction_ratio) {
                true => Vec3::reflect(unit_direction, hit_record.hit_normal),
                false => Vec3::refract(
                    ray_in.dir.unit_vector(),
                    hit_record.hit_normal,
                    refraction_ratio,
                ),
            },
            ray_in.depth - 1,
        );

        Option::Some((scattered, attenuation))
    }
}

fn is_reflect(ray_in: Vec3, normal: Vec3, ior: f64) -> bool {
    let cos_theta = Vec3::dot(ray_in * -1.0, normal).min(1.0);

    let sin_theta = (1.0 - cos_theta.powf(2.0)).sqrt();

    let reflectance = ((1.0 - ior) / (1.0 + ior)).powf(2.0);
    let reflectance = reflectance + (1.0 - reflectance) * (1.0 - cos_theta).powf(5.0);

    ior * sin_theta > 1.0 || reflectance > random_01()
}

use std::f64::consts::PI;

use crate::*;

pub struct Lambertian {
    albedo:TextureType,
}

impl Lambertian {
    pub fn new(albedo: Color) -> Lambertian {
        Lambertian { albedo:Arc::new(Box::new(SolidColor::new(albedo))) }
    }

    pub fn new_texture(a: TextureType) -> Lambertian{
        Lambertian { albedo: a}
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

        let attenuation = self.albedo.value(hit_record.u,hit_record.v,&hit_record.hit_point);

        Option::Some((scattered, attenuation))
    }

    fn scatter_mc(&self, ray_in: &Ray, hit_record: &HitRecord) -> Option<ScatterRecord> {
        let srec = ScatterRecord{
            specular_ray: Ray::new_default(),
            is_specular: false,
            attenuation: self.albedo.value(hit_record.u, hit_record.v, &hit_record.hit_point),
            pdf_ptr: Arc::new(Box::new(Cosine_pdf::new(&hit_record.hit_normal)))
        };

        Some(srec)
    }

    fn scattering_pdf(&self, ray_in: &Ray, hit_record: &HitRecord, scattered: &Ray) -> f64 {
        let cosine = Vec3::dot(hit_record.hit_normal, scattered.dir.unit_vector());
        if cosine < 0.0{
            return 0.0;
        } else {
            return cosine / PI;
        }
    }
}

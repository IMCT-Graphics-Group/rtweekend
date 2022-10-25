use crate::*;

pub struct Isotropic{
    albedo:TextureType
}

impl Isotropic{
    pub fn new(c:Color)->Self{
        Isotropic { albedo: Arc::new(Box::new(SolidColor::new(c))) }
    }

    pub fn new_texture(albedo: TextureType) -> Self{
        Isotropic { albedo }
    }
}

impl Material for Isotropic {
    fn scatter(&self, ray_in: Ray, hit_record: &HitRecord) -> Option<(Ray, Color)> {
        let scattered = Ray::new(hit_record.hit_point, random_unit_sphere(), ray_in.depth);
        let attenuation = self.albedo.value(hit_record.u, hit_record.v, &hit_record.hit_point);
        Some((scattered, attenuation))
    }
}
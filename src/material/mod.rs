pub mod dielectric;
pub mod lambertian;
pub mod metal;
pub mod diffuselight;
pub mod isotropic;

use crate::*;

pub struct ScatterRecord{
    pub specular_ray: Ray,
    pub is_specular: bool,
    pub attenuation: Color,
    pub pdf_ptr: Arc<Box<dyn Pdf>>
}

pub trait Material {
    fn scatter(&self, ray_in: Ray, hit_record: &HitRecord) -> Option<(Ray, Color)>;
    
    fn emitted(&self, u:f64,v:f64,p:&Point3) -> Color{
        Color::new_color(0.0, 0.0, 0.0)
    }
    
    //采用重要性采样
    fn scatter_mc(&self, ray_in: &Ray, hit_record: &HitRecord) -> Option<ScatterRecord>{
        None
    }
    
    fn scattering_pdf(&self, ray_in: &Ray, hit_record: &HitRecord, scattered: &Ray) -> f64{
        0.0
    }
}

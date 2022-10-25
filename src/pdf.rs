use std::f64::consts::PI;

use crate::*;

pub trait Pdf {
    fn value(&self, direction: &Vec3) -> f64;

    fn generate(&self) -> Vec3;
}

pub struct Cosine_pdf{
    pub uvw:Onb
}

impl Cosine_pdf{
    pub fn new(w: &Vec3) -> Self{
        let uvw = Onb::build_from_w(w);
        Cosine_pdf{uvw}
    }
}

impl Pdf for Cosine_pdf {
    fn value(&self, direction: &Vec3) -> f64 {
        let cosine = Vec3::dot(direction.unit_vector(), self.uvw.w());
        if cosine <= 0.0{
            return 0.0;
        } else {
            return cosine / PI;
        }
    }

    fn generate(&self) -> Vec3 {
        self.uvw.local(&random_cosine_direction())
    }
}

pub struct Hittable_pdf{
    o:Point3,
    hittable_ptr: HittableType
}

impl Hittable_pdf {
    pub fn new(hittable_ptr: HittableType,origin:&Point3) -> Self{
        let o = *origin;
        Hittable_pdf { o, hittable_ptr }
    }
}

impl Pdf for Hittable_pdf {
    fn value(&self, direction: &Vec3) -> f64 {
        self.hittable_ptr.pdf_value(&self.o, direction)
    }

    fn generate(&self) -> Vec3 {
        self.hittable_ptr.random(&self.o)
    }
}

pub struct Mixture_pdf{
    p0 : Arc<Box<dyn Pdf>>,
    p1 : Arc<Box<dyn Pdf>>
}

impl Mixture_pdf{
    pub fn new(p0: Arc<Box<dyn Pdf>>, p1: Arc<Box<dyn Pdf>>) -> Self{
        Mixture_pdf{p0,p1}
    }
}

impl Pdf for Mixture_pdf{
    fn value(&self, direction: &Vec3) -> f64 {
        0.5 * self.p0.value(direction) + 0.5 * self.p1.value(direction)
    }

    fn generate(&self) -> Vec3 {
        if random_01() < 0.5{
            return self.p0.generate(); 
        } else{
            return self.p1.generate();
        }
    }
}
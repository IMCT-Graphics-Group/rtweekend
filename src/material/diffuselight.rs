use  crate::*;

pub struct DiffuseLight{
    pub emit:TextureType,
}

impl DiffuseLight {
    pub fn new_color(c:Color) -> DiffuseLight{
        DiffuseLight { 
            emit: Arc::new(Box::new(SolidColor::new(c))) 
        }
    }

    pub fn new_texture(a:TextureType) -> DiffuseLight{
        DiffuseLight { emit: a }
    }
}

impl Material for DiffuseLight {
    fn scatter(&self, ray_in: Ray, hit_record: &HitRecord) -> Option<(Ray, Color)> {
        None
    }

    fn emitted(&self, u:f64,v:f64,p:&Point3) -> Color {
        self.emit.value(u, v, p)
    }
}
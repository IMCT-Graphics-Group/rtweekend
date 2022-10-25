use crate::*;

pub struct ConstantMedium{
    boundary:ObjectType,
    phase_function:MaterialType,
    neg_inv_density:f64,
}

impl ConstantMedium {
    pub fn new(boundary:ObjectType,neg_inv_density:f64,c:Color) -> ConstantMedium{
        ConstantMedium { 
            boundary, 
            phase_function: Arc::new(Box::new(Isotropic::new(c))), 
            neg_inv_density 
        }
    }
}

impl Hittable for ConstantMedium{
    fn hit(&self, ray: &Ray, t_range: (f64, f64)) -> Option<HitRecord> {
        let enableDebug = true;
        let debugging = enableDebug && random_01() < 0.00001;

        let mut rec1 = HitRecord::new_default();
        let mut rec2 = HitRecord::new_default();

        if let Some(mut rec) = self.boundary.hit(ray, (-f64::INFINITY, f64::INFINITY)){
            rec1 = rec;
        } else {
            return None;
        }

        if let Some(mut rec) = self.boundary.hit(ray, (rec1.t+0.001, f64::INFINITY)) {
            rec2 = rec;
        }else{
            return None;
        }
        
        if rec1.t < t_range.0{
            rec1.t = t_range.0;
        }
        if rec2.t > t_range.1{
            rec2.t = t_range.1;
        }

        if rec1.t >= rec2.t{
            return None;
        }

        if rec1.t < 0.0{
            rec1.t = 0.0;
        }

        let ray_length = ray.dir.length();
        let distance_inside_boundary = (rec2.t-rec1.t) * ray_length;
        let hit_distance = self.neg_inv_density * random_01().log10();

        if hit_distance > distance_inside_boundary{
            return None;
        }

        let t = rec1.t + hit_distance / ray_length;
        let p = ray.at(t);

        let rec = HitRecord::new(
            p, 
            Vec3(1.0, 0.0, 0.0), 
            self.phase_function.clone(), 
            t, 
            0.0, 
            0.0, 
            true
        );

        Some(rec)    
    }
}

impl Bounded for ConstantMedium{
    fn bounding_box(&self) -> AABB {
        self.boundary.bounding_box()
    }
}
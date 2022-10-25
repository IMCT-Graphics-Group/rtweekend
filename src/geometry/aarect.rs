use crate::*;

pub struct XYrect{
    material: MaterialType,
    x0:f64,
    x1:f64,
    y0:f64,
    y1:f64,
    k:f64,
}

impl XYrect {
    pub fn new(x0:f64,x1:f64,y0:f64,y1:f64,k:f64,material:MaterialType)->XYrect{
        XYrect { material, x0, x1, y0, y1, k }
    }
}

impl Bounded for XYrect {
    fn bounding_box(&self) -> AABB {
        AABB::new(
            Point3::new_point3(self.x0, self.y0, self.k-0.001), 
            Point3::new_point3(self.x1, self.y1, self.k+0.001),
        )
    }
}

impl Hittable for XYrect {
    fn hit(&self, ray: &Ray, t_range: (f64, f64)) -> Option<HitRecord> {
        let t = (self.k - ray.orig.z()) / ray.dir.z();
        if t<t_range.0 || t>t_range.1{
            return None;
        }
        let x = ray.orig.x() + t*ray.dir.x();
        let y = ray.orig.y() + t*ray.dir.y();
        if x<self.x0 || x>self.x1 || y<self.y0 || y>self.y1{
            return None;
        }
        let u = (x-self.x0) / (self.x1-self.x0);
        let v = (y-self.y0) / (self.y1-self.y0);
        let outward_normal = Vec3(0.0,0.0,1.0);
        let (front_face,hit_normal) = Vec3::set_face_normal(ray.dir, outward_normal);
        let hit_point = ray.at(t);

        let rec = HitRecord::new(
            hit_point, 
            hit_normal, 
            self.material.clone(), 
            t, 
            u, 
            v, 
            front_face
        );

        Some(rec)
        
    }
}

pub struct XZrect{
    material: MaterialType,
    x0:f64,
    x1:f64,
    z0:f64,
    z1:f64,
    k:f64,
}

impl XZrect {
    pub fn new(x0:f64,x1:f64,z0:f64,z1:f64,k:f64,material:MaterialType)->XZrect{
        XZrect { material, x0, x1, z0, z1, k }
    }
}

impl Bounded for XZrect {
    fn bounding_box(&self) -> AABB {
        AABB::new(
            Point3::new_point3(self.x0, self.k-0.001, self.z0), 
            Point3::new_point3(self.x1, self.k+0.001, self.z1),
        )
    }
}

impl Hittable for XZrect {
    fn hit(&self, ray: &Ray, t_range: (f64, f64)) -> Option<HitRecord> {
        let t = (self.k - ray.orig.y()) / ray.dir.y();
        if t<t_range.0 || t>t_range.1{
            return None;
        }
        let x = ray.orig.x() + t*ray.dir.x();
        let z = ray.orig.z() + t*ray.dir.z();
        if x<self.x0 || x>self.x1 || z<self.z0 || z>self.z1{
            return None;
        }
        let u = (x-self.x0) / (self.x1-self.x0);
        let v = (z-self.z0) / (self.z1-self.z0);
        let outward_normal = Vec3(0.0,1.0,0.0);
        let (front_face,hit_normal) = Vec3::set_face_normal(ray.dir, outward_normal);
        let hit_point = ray.at(t);

        let rec = HitRecord::new(
            hit_point, 
            hit_normal, 
            self.material.clone(), 
            t, 
            u, 
            v, 
            front_face
        );

        Some(rec)
        
    }

    fn pdf_value(&self, o: &Point3, v: &Vec3) -> f64 {
        let ray = Ray::new(*o, *v, 0);
        if let Some(rec) = self.hit(&ray, (0.001,f64::INFINITY)){
            let area = (self.x1-self.x0) * (self.z1-self.z0);
            let distance_squared = rec.t * rec.t * v.length_squared();
            let cosine = (Vec3::dot(*v, rec.hit_normal) / v.length()).abs();

            return distance_squared / (cosine * area);
        } else {
            return 0.0;
        }
    }

    fn random(&self, o: &Vec3) -> Vec3 {
        let random_point = Point3::new_point3(
            random_range(self.x0,self.x1), 
            self.k, 
            random_range(self.z0,self.z1)
        );

        random_point - *o
    }
}

pub struct YZrect{
    material: MaterialType,
    y0:f64,
    y1:f64,
    z0:f64,
    z1:f64,
    k:f64,
}

impl YZrect {
    pub fn new(y0:f64,y1:f64,z0:f64,z1:f64,k:f64,material:MaterialType)->YZrect{
        YZrect { material, y0, y1, z0, z1, k }
    }
}

impl Bounded for YZrect {
    fn bounding_box(&self) -> AABB {
        AABB::new(
            Point3::new_point3(self.k-0.001, self.y0, self.z0), 
            Point3::new_point3(self.k+0.001, self.y1, self.z1),
        )
    }
}

impl Hittable for YZrect {
    fn hit(&self, ray: &Ray, t_range: (f64, f64)) -> Option<HitRecord> {
        let t = (self.k - ray.orig.x()) / ray.dir.x();
        if t<t_range.0 || t>t_range.1{
            return None;
        }
        let y = ray.orig.y() + t*ray.dir.y();
        let z = ray.orig.z() + t*ray.dir.z();
        if y<self.y0 || y>self.y1 || z<self.z0 || z>self.z1{
            return None;
        }
        let u = (y-self.y0) / (self.y1-self.y0);
        let v = (z-self.z0) / (self.z1-self.z0);
        let outward_normal = Vec3(1.0,0.0,0.0);
        let (front_face,hit_normal) = Vec3::set_face_normal(ray.dir, outward_normal);
        let hit_point = ray.at(t);

        let rec = HitRecord::new(
            hit_point, 
            hit_normal, 
            self.material.clone(), 
            t, 
            u, 
            v, 
            front_face
        );

        Some(rec)
        
    }
}
use std::fmt::Display;

use crate::*;

pub trait Hittable {
    fn hit(&self, ray: &Ray, t_range: (f64, f64)) -> Option<HitRecord>;

    fn pdf_value(&self, o: &Point3, v: &Vec3) -> f64{
        0.0
    }

    fn random(&self, o: &Vec3) -> Vec3{
        Vec3(1.0, 0.0, 0.0)
    }
}

pub struct HitRecord {
    pub hit_point: Point3,
    pub hit_normal: Vec3,
    pub hit_material: MaterialType,
    pub t: f64,
    pub u: f64,
    pub v: f64,
    pub front_face: bool,
}

impl HitRecord {
    pub fn new(
        hit_point: Vec3,
        hit_normal: Vec3,
        hit_material: MaterialType,
        t: f64,
        u: f64,
        v: f64,
        front_face: bool,
    ) -> HitRecord {
        HitRecord {
            hit_point,
            hit_normal,
            hit_material,
            t,
            u,
            v,
            front_face,
        }
    }

    pub fn new_default() -> Self{
        HitRecord {
            hit_point:Point3::new_point3(0.0,0.0,0.0),
            hit_normal:Vec3(0.0, 0.0, 0.0),
            hit_material:Arc::new(Box::new(Lambertian::new(Color::new_color(0.0, 0.0, 0.0)))),
            t:0.0,
            u:0.0,
            v:0.0,
            front_face:true,
        }
    }
}

impl Display for HitRecord {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Hit_point:{}\nHit_normal:{}\nt:{}\n",
            self.hit_point, self.hit_normal, self.t
        )
    }
}

pub struct Translate{
    object:ObjectType,
    offset:Vec3,
}

impl Translate {
    pub fn new(object:ObjectType,offset:Vec3) -> Translate{
        Translate { object, offset }
    }
}

impl Hittable for Translate {
    fn hit(&self, ray: &Ray, t_range: (f64, f64)) -> Option<HitRecord> {
        let move_ray = Ray::new(ray.orig - self.offset, ray.dir,ray.depth);
        match self.object.hit(&move_ray, t_range) {
            None => { return None;},
            Some(mut rec) => {
                rec.hit_point += self.offset;
                let (front_face, hit_normal) = Vec3::set_face_normal(move_ray.dir, rec.hit_normal);
                rec.front_face = front_face;
                rec.hit_normal = hit_normal;
                return Some(rec);
            }
        }
    }
}

impl Bounded for Translate {
    fn bounding_box(&self) -> AABB {
        AABB::new(
            self.object.bounding_box().min() + self.offset, 
            self.object.bounding_box().max() + self.offset,
        )
    }
}


pub struct RotateY{
    object:ObjectType,
    sin_theta:f64,
    cos_theta:f64,
    bbox:AABB,
}

impl RotateY{
    pub fn new(object:ObjectType,angle:f64) -> RotateY{
        let radians = degree_to_radians(angle);
        let sin_theta = radians.sin();
        let cos_theta = radians.cos();
        let mut bbox = object.bounding_box();

        let mut min = Point3::new_point3(f64::INFINITY, f64::INFINITY, f64::INFINITY);
        let mut max = Point3::new_point3(-f64::INFINITY, -f64::INFINITY, -f64::INFINITY);

        for i in 0..2{
            for j in 0..2{
                for k in 0..2{
                    let x = i as f64 * bbox.max().x() + (1-i) as f64*bbox.min().x();
                    let y = j as f64 * bbox.max().y() + (1-j) as f64*bbox.min().y();
                    let z = k as f64 * bbox.max().z() + (1-k) as f64*bbox.min().z();

                    let newx = cos_theta * x + sin_theta * z;
                    let newz = -sin_theta * x + cos_theta * z;

                    let tester = Vec3(newx,y,newz);

                    for c in 0..3{
                        min.set(c, f64::min(min.get(c), tester.get(c)));
                        max.set(c, f64::max(max.get(c), tester.get(c)));
                    }

                    // let min0 = f64::min(min.0, tester.0);
                    // let min1 = f64::min(min.1, tester.1);
                    // let min2 = f64::min(min.2, tester.2);
                    // let max0 = f64::max(max.0, tester.0);
                    // let max1 = f64::max(max.1, tester.1);
                    // let max2 = f64::max(max.2, tester.2);

                    // min = Point3::new_point3(min0,min1,min2);
                    // max = Point3::new_point3(max0, max1, max2);
                }
            }
        }

        bbox = AABB::new(min, max);
        
        RotateY { 
            object, 
            sin_theta, 
            cos_theta, 
            bbox
        }

    }
}

impl Hittable for RotateY {
    fn hit(&self, ray: &Ray, t_range: (f64, f64)) -> Option<HitRecord> {
        let mut origin = ray.orig;
        let mut direction = ray.dir;

        origin.set(
            0, 
            self.cos_theta * ray.orig.get(0) - self.sin_theta * ray.orig.get(2)
        );
        origin.set(
            2, 
            self.sin_theta * ray.orig.get(0) + self.cos_theta * ray.orig.get(2)
        );

        direction.set(
            0, 
            self.cos_theta * ray.dir.get(0) - self.sin_theta * ray.dir.get(2)
        );
        direction.set(
            2, 
            self.sin_theta * ray.dir.get(0) + self.cos_theta * ray.dir.get(2)
        );

        let rotated_r = Ray::new(origin, direction, ray.depth);
        match self.object.hit(&rotated_r, t_range) {
            None => {return None;}
            Some(mut rec) => {
                //println!("Hit!!!!");
                
                let mut p = rec.hit_point;
                let mut normal = rec.hit_normal;

                p.set(
                    0, 
                    self.cos_theta * rec.hit_point.get(0) + self.sin_theta * rec.hit_point.get(2)
                );
                p.set(
                    2, 
                    -self.sin_theta * rec.hit_point.get(0) + self.cos_theta * rec.hit_point.get(2)
                );

                normal.set(
                    0, 
                    self.cos_theta * rec.hit_normal.get(0) + self.sin_theta * rec.hit_normal.get(2)
                );
                normal.set(
                    2, 
                    -self.sin_theta * rec.hit_normal.get(0) + self.cos_theta * rec.hit_normal.get(2)
                );

                rec.hit_point = p;
                let (front_face, hit_normal) = Vec3::set_face_normal(rotated_r.dir, normal);
                rec.front_face = front_face;
                rec.hit_normal = hit_normal;

                return Some(rec);
            }
        }


    }
}

impl Bounded for RotateY {
    fn bounding_box(&self) -> AABB {
        self.bbox
    }
}


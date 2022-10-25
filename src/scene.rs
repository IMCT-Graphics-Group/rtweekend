use crate::*;

pub struct Scene {
    pub objects: Vec<ObjectType>,
    pub bvh: BVH,
}

impl Scene {
    pub fn new() -> Scene {
        Scene {
            objects: Vec::new(),
            bvh: BVH::default(),
        }
    }

    pub fn add_object(&mut self, object: ObjectType) {
        self.objects.push(object);
    }

    pub fn add_objects(&mut self, objects: & mut Vec<ObjectType>){
        self.objects.append(objects);
    }

    pub fn build_bvh(&mut self) {
        self.bvh = BVH::build(self.objects.clone());
    }
}

impl Hittable for Scene {
    // BVH查询
    fn hit(&self, ray: &Ray, t_range: (f64, f64)) -> Option<HitRecord> {
        self.bvh.hit(ray, t_range)
    }

    // 迭代查询
    // fn hit(&self, ray: &Ray, t_range: (f64, f64)) -> Option<HitRecord> {
    //     let mut result = Option::None;
    //     let mut closest_so_far = t_range.1;

    //     for object in self.objects.iter() {
    //         if let Some(record) = object.hit(ray, (t_range.0, closest_so_far)) {
    //             closest_so_far = record.t;
    //             result = Some(record);
    //         }
    //     }

    //     result
    // }
}

use crate::*;


pub struct MyBox{
    pub box_min:Point3,
    pub box_max:Point3,
    pub sides: Vec<ObjectType>,
}

impl MyBox {
    pub fn new(p0:Point3,p1:Point3,material:MaterialType) -> MyBox{
        let box_min = p0;
        let box_max = p1;

        let mut sides:Vec<ObjectType> = Vec::new();
        sides.push(Arc::new(Box::new(XYrect::new(p0.x(), p1.x(), p0.y(), p1.y(), p1.z(), material.clone()))));
        sides.push(Arc::new(Box::new(XYrect::new(p0.x(), p1.x(), p0.y(), p1.y(), p0.z(), material.clone()))));

        sides.push(Arc::new(Box::new(XZrect::new(p0.x(), p1.x(), p0.z(), p1.z(), p1.y(), material.clone()))));
        sides.push(Arc::new(Box::new(XZrect::new(p0.x(), p1.x(), p0.z(), p1.z(), p0.y(), material.clone()))));

        sides.push(Arc::new(Box::new(YZrect::new(p0.y(), p1.y(), p0.z(), p1.z(), p1.x(), material.clone()))));
        sides.push(Arc::new(Box::new(YZrect::new(p0.y(), p1.y(), p0.z(), p1.z(), p0.x(), material.clone()))));

        MyBox { box_min, box_max, sides }
    }
}

impl Hittable for MyBox {
    fn hit(&self, ray: &Ray, t_range: (f64, f64)) -> Option<HitRecord> {
        let mut result = Option::None;
        let mut closest_so_far = t_range.1;

        for object in self.sides.iter() {
            if let Some(record) = object.hit(ray, (t_range.0, closest_so_far)) {
                closest_so_far = record.t;
                result = Some(record);
            }
        }

        result
    }
}

impl Bounded for MyBox {
    fn bounding_box(&self) -> AABB {
        AABB::new(
            self.box_min,
            self.box_max,
        )
    }
}


use crate::*;

#[derive(Default)]
pub struct Scene {
    pub objects: Vec<Box<dyn Hittable>>,
}

impl Scene {
    pub fn new() -> Scene {
        Default::default()
    }

    pub fn add_object<T: Hittable + 'static>(&mut self, object: T) {
        self.objects.push(Box::new(object));
    }

    pub fn hit(&self, ray: &Ray, t_range: (f64, f64)) -> Option<HitRecord> {
        let mut result = Option::None;
        let mut closest_so_far = t_range.1;

        for object in self.objects.iter() {
            if let Some(record) = object.hit(ray, (t_range.0, closest_so_far)) {
                closest_so_far = record.t;
                result = Some(record);
            }
        }

        result
    }
}

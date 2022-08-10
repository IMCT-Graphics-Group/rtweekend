use crate::*;

type BoundedObject = Arc<Box<dyn Bounded + Send + Sync>>;
type BoundedList = Vec<BoundedObject>;

pub struct BVH {
    root: Node,
}

impl BVH {
    pub fn default() -> BVH {
        let material_default: MaterialType = Arc::new(Box::new(Lambertian::new(Color::new_color(
            0.999, 0.0, 0.999,
        ))));
        let sphere_default: BoundedObject = Arc::new(Box::new(Sphere::new(
            Vec3(0.0, 0.0, 0.0),
            1.0,
            material_default,
        )));
        BVH {
            root: Node {
                left: sphere_default.clone(),
                right: sphere_default.clone(),
                aabb: sphere_default.bounding_box(),
            },
        }
    }

    pub fn build(objects: BoundedList) -> BVH {
        BVH {
            root: recursive_build(objects),
        }
    }
}

impl Hittable for BVH {
    fn hit(&self, ray: &Ray, t_range: (f64, f64)) -> Option<HitRecord> {
        self.root.hit(ray, t_range)
    }
}

fn recursive_build(mut list: BoundedList) -> Node {
    let length = list.len();
    if length == 1 {
        return Node {
            left: list[0].clone(),
            right: list[0].clone(),
            aabb: list[0].bounding_box(),
        };
    } else if length == 2 {
        return Node {
            left: list[0].clone(),
            right: list[1].clone(),
            aabb: list[0].bounding_box() + list[1].bounding_box(),
        };
    }

    let axis = random_int(0, 2);

    list.sort_by_key(|key| key.bounding_box().min().get(axis as usize) as i32);

    let (left, right) = list.split_at_mut(length / 2);

    let left_node = recursive_build(left.to_vec());
    let right_node = recursive_build(right.to_vec());
    let aabb = left_node.bounding_box() + right_node.bounding_box();

    let result = Node {
        left: Arc::new(Box::new(left_node)),
        right: Arc::new(Box::new(right_node)),
        aabb,
    };

    result
}

struct Node {
    left: BoundedObject,
    right: BoundedObject,
    aabb: AABB,
}

impl Hittable for Node {
    fn hit(&self, ray: &Ray, t_range: (f64, f64)) -> Option<HitRecord> {
        let range = self.aabb.hit(ray, t_range)?;

        let mut left_result = self.left.hit(ray, range);
        let right_result = self.right.hit(
            ray,
            match left_result {
                Some(temp_result) => {
                    let t = temp_result.t;
                    left_result = Option::Some(temp_result);
                    (range.0, t)
                }
                None => range,
            },
        );

        match right_result {
            Some(_) => right_result,
            None => left_result,
        }
    }
}

impl Bounded for Node {
    fn bounding_box(&self) -> AABB {
        self.aabb
    }
}

use crate::*;

#[derive(Clone, Debug, Default)]
pub struct Ray {
    pub orig: Point3,
    pub dir: Vec3,
    pub depth: u32,
}

impl Ray {
    pub fn new(orig: Point3, dir: Vec3, depth: u32) -> Ray {
        Ray { orig, dir, depth }
    }

    pub fn at(&self, t: f64) -> Point3 {
        self.orig + self.dir * t
    }
}

pub fn ray_color(ray: Ray, config: &Config) -> Color {
    if ray.depth <= 0 {
        return Color::new_color(0.0, 0.0, 0.0);
    }

    if let Some(hit_record) = config.scene.hit(&ray, (1e-8, f64::INFINITY)) {
        if let Some((scattered, attenuation)) = hit_record.hit_material.scatter(ray, &hit_record) {
            return attenuation * ray_color(scattered, config);
        }
        return Color::new_color(0.0, 0.0, 0.0);
    } else {
        let t = 0.5 * (ray.dir.unit_vector().y() + 1.0);
        return Color::new_color(1.0, 1.0, 1.0) * (1.0 - t) + Color::new_color(0.5, 0.7, 1.0) * t;
    }
}

use crate::*;

pub struct Camera {
    origin: Point3,
    horizontal: Vec3,
    vertical: Vec3,
    uvw: (Vec3, Vec3, Vec3),
    aperture: f64,
    lower_left_corner: Point3,
    upper_left_corner: Point3,
    ray_depth: u32,
}

impl Camera {
    pub fn new(
        look_from: Point3,
        look_at: Point3,
        view_up: Vec3,
        field_of_view: f64,
        aspect_ratio: f64,
        aperture: f64,
        focus_distance: f64,
        ray_depth: u32,
    ) -> Camera {
        let theta = field_of_view.to_radians();
        let h = (theta / 2.0).tan();
        let viewport_height = 2.0 * h;
        let viewport_width = viewport_height * aspect_ratio;

        let w = (look_from - look_at).unit_vector();
        let u = Vec3::cross(view_up, w).unit_vector();
        let v = Vec3::cross(w, u);

        let origin = look_from;
        let horizontal = u * viewport_width * focus_distance;
        let vertical = v * viewport_height * focus_distance;
        let lower_left_corner = origin - horizontal / 2.0 - vertical / 2.0 - w * focus_distance;
        let upper_left_corner = origin - horizontal / 2.0 + vertical / 2.0 - w * focus_distance;

        Camera {
            origin,
            horizontal,
            vertical,
            uvw: (u, v, w),
            aperture,
            lower_left_corner,
            upper_left_corner,
            ray_depth,
        }
    }

    pub fn get_ray_lower_left(&self, u: f64, v: f64) -> Ray {
        let rd = random_unit_disk() * self.aperture / 2.0;
        let offset = self.uvw.0 * rd.x() + self.uvw.1 * rd.y();

        Ray {
            orig: self.origin + offset,
            dir: self.lower_left_corner + self.horizontal * u + self.vertical * v
                - self.origin
                - offset,
            depth: self.ray_depth,
        }
    }

    pub fn get_ray_upper_left(&self, u: f64, v: f64) -> Ray {
        let rd = random_unit_disk() * self.aperture / 2.0;
        let offset = self.uvw.0 * rd.x() + self.uvw.1 * rd.y();

        Ray {
            orig: self.origin + offset,
            dir: self.upper_left_corner + self.horizontal * u
                - self.vertical * v
                - self.origin
                - offset,
            depth: self.ray_depth,
        }
    }
}

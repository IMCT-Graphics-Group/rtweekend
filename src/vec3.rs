use std::{
    fmt::Display,
    ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Sub, SubAssign},
};

#[derive(Default, Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct Vec3(pub f64, pub f64, pub f64);
pub type Point3 = Vec3;
pub type Color = Vec3;

impl Point3 {
    pub fn new_point3(x: f64, y: f64, z: f64) -> Vec3 {
        Vec3(x, y, z)
    }

    pub fn x(&self) -> f64 {
        self.0
    }

    pub fn y(&self) -> f64 {
        self.1
    }

    pub fn z(&self) -> f64 {
        self.2
    }
}

impl Color {
    pub fn new_color(r: f64, g: f64, b: f64) -> Vec3 {
        Vec3(r, g, b)
    }

    pub fn r(&self) -> f64 {
        self.0
    }

    pub fn g(&self) -> f64 {
        self.1
    }

    pub fn b(&self) -> f64 {
        self.2
    }
}

impl Vec3 {
    pub fn length(&self) -> f64 {
        f64::sqrt(self.length_squared())
    }

    pub fn length_squared(&self) -> f64 {
        self.0 * self.0 + self.1 * self.1 + self.2 * self.2
    }

    pub fn dot(lhs: Vec3, rhs: Vec3) -> f64 {
        lhs.0 * rhs.0 + lhs.1 * rhs.1 + lhs.2 * rhs.2
    }

    pub fn cross(lhs: Vec3, rhs: Vec3) -> Self {
        Vec3(
            lhs.1 * rhs.2 - lhs.2 * rhs.1,
            lhs.2 * rhs.0 - lhs.0 * rhs.2,
            lhs.0 * rhs.1 - lhs.1 * rhs.0,
        )
    }

    pub fn unit_vector(&self) -> Self {
        *self / self.length()
    }

    pub fn normalize(&mut self) {
        *self /= self.length();
    }

    pub fn near_zero(&self) -> bool {
        self.0.abs() < 1e-8f64 && self.1.abs() < 1e-8f64 && self.2.abs() < 1e-8f64
    }

    pub fn get(&self, index: usize) -> f64 {
        self.array()[index]
    }

    pub fn array(&self) -> [f64; 3] {
        [self.0, self.1, self.2]
    }

    pub fn from_array(array: [f64; 3]) -> Vec3 {
        Vec3(array[0], array[1], array[2])
    }

    pub fn from_vec(list: Vec<f64>) -> Vec3 {
        Vec3(list[0], list[1], list[2])
    }

    pub fn reflect(v: Vec3, n: Vec3) -> Vec3 {
        v - n * Vec3::dot(v, n) * 2.0
    }

    pub fn refract(uv: Vec3, n: Vec3, etai_over_etat: f64) -> Vec3 {
        let cos_theta = f64::min(Vec3::dot(uv * -1.0, n), 1.0);
        let r_out_perp = (uv + n * cos_theta) * etai_over_etat;
        let r_out_parallel = n * (-1.0 * f64::sqrt((1.0 - r_out_perp.length_squared()).abs()));
        r_out_perp + r_out_parallel
    }

    pub fn set_face_normal(ray_in_dir: Vec3, outward_normal: Vec3) -> (bool, Vec3) {
        let is_front_face = Vec3::dot(ray_in_dir, outward_normal) < 0.0;
        (
            is_front_face,
            match is_front_face {
                true => outward_normal,
                false => outward_normal * -1.0,
            },
        )
    }
}

impl Add for Vec3 {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self(self.0 + rhs.0, self.1 + rhs.1, self.2 + rhs.2)
    }
}

impl AddAssign for Vec3 {
    fn add_assign(&mut self, rhs: Self) {
        *self = Self(self.0 + rhs.0, self.1 + rhs.1, self.2 + rhs.2)
    }
}

impl Sub for Vec3 {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self(self.0 - rhs.0, self.1 - rhs.1, self.2 - rhs.2)
    }
}

impl SubAssign for Vec3 {
    fn sub_assign(&mut self, rhs: Self) {
        *self = Self(self.0 - rhs.0, self.1 - rhs.1, self.2 - rhs.2)
    }
}

impl Mul for Vec3 {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        Self(self.0 * rhs.0, self.1 * rhs.1, self.2 * rhs.2)
    }
}

impl Mul<f64> for Vec3 {
    type Output = Self;

    fn mul(self, rhs: f64) -> Self::Output {
        Self(self.0 * rhs, self.1 * rhs, self.2 * rhs)
    }
}

impl MulAssign for Vec3 {
    fn mul_assign(&mut self, rhs: Self) {
        *self = Self(self.0 * rhs.0, self.1 * rhs.1, self.2 * rhs.2)
    }
}

impl MulAssign<f64> for Vec3 {
    fn mul_assign(&mut self, rhs: f64) {
        *self = Self(self.0 * rhs, self.1 * rhs, self.2 * rhs)
    }
}

impl Div for Vec3 {
    type Output = Self;

    fn div(self, rhs: Self) -> Self::Output {
        Self(self.0 / rhs.0, self.1 / rhs.1, self.2 / rhs.2)
    }
}

impl Div<f64> for Vec3 {
    type Output = Self;

    fn div(self, rhs: f64) -> Self::Output {
        self * (1f64 / rhs)
    }
}

impl DivAssign for Vec3 {
    fn div_assign(&mut self, rhs: Self) {
        *self = Self(self.0 / rhs.0, self.1 / rhs.1, self.2 / rhs.2)
    }
}

impl DivAssign<f64> for Vec3 {
    fn div_assign(&mut self, rhs: f64) {
        *self *= 1f64 / rhs
    }
}

impl Display for Vec3 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {}, {})", self.0, self.1, self.2)
    }
}

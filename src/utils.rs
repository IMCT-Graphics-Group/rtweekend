use crate::*;

use rand::prelude::*;

use std::f64::consts::PI;

pub fn degree_to_radians(degrees:f64) -> f64{
    degrees * PI / 180.0
}

pub fn random_01() -> f64 {
    rand::thread_rng().gen::<f64>()
}

pub fn random_range(t_min: f64, t_max: f64) -> f64 {
    rand::thread_rng().gen_range(t_min..=t_max)
}

pub fn random_int(t_min: i32, t_max: i32) -> i32 {
    rand::thread_rng().gen_range(t_min..=t_max)
}

pub fn random_unit_sphere() -> Vec3 {
    loop {
        let sample = Vec3(
            random_range(-1.0, 1.0),
            random_range(-1.0, 1.0),
            random_range(-1.0, 1.0),
        );

        if sample.length_squared() < 1.0 {
            return sample;
        }
    }
}

pub fn random_unit_vector() -> Vec3{
    random_unit_sphere().unit_vector()
}

pub fn random_cosine_direction() -> Vec3 {
    let r1 = random_01();
    let r2 = random_01();
    let z = (1.0 - r2).sqrt();

    let phi = 2.0*PI*r1;
    let x = phi.cos() * r2.sqrt();
    let y = phi.sin() * r2.sqrt();

    Vec3 (x,y,z)
}

pub fn random_unit_disk() -> Vec3 {
    loop {
        let sample = Vec3(random_range(-1.0, 1.0), random_range(-1.0, 1.0), 0.0);

        if sample.length_squared() < 1.0 {
            return sample;
        }
    }
}

pub fn random_hemisphere(normal: Vec3) -> Vec3 {
    let sample = random_unit_sphere();
    if Vec3::dot(sample, normal) > 0.0 {
        sample
    } else {
        sample * -1.0
    }
}

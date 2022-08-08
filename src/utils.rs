use crate::*;

use rand::prelude::*;

pub fn random_01() -> f64 {
    rand::thread_rng().gen::<f64>()
}

pub fn random_range(t_min: f64, t_max: f64) -> f64 {
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

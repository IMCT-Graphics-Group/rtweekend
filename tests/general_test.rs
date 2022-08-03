use rtweekend::*;

#[test]
fn vec3_add_work() {
    let lhs = Vec3(1.5f64, 2.5f64, 3.5f64);
    let rhs = Vec3(5.5f64, 6.5f64, 7.5f64);
    let result = lhs + rhs;
    assert_eq!(result, Vec3(7.0f64, 9.0f64, 11.0f64));
}

#[test]
fn vec3_sub_work() {
    let lhs = Vec3(1.5f64, 2.5f64, 3.5f64);
    let rhs = Vec3(5.5f64, 6.5f64, 7.5f64);
    let result = lhs - rhs;
    assert_eq!(result, Vec3(-4.0f64, -4.0f64, -4.0f64));
}

#[test]
fn vec3_mul_work() {
    let lhs = Vec3(1.5f64, 2.5f64, 3.5f64);
    let rhs = Vec3(5.5f64, 6.5f64, 7.5f64);
    let result = lhs * rhs;
    assert_eq!(result, Vec3(8.25f64, 16.25f64, 26.25f64));
}

#[test]
fn vec3_div_work() {
    let lhs = Vec3(2.0f64, 3.0f64, 4.0f64);
    let rhs = Vec3(5.0f64, 6.0f64, 16.0f64);
    let result = lhs / rhs;
    assert_eq!(result, Vec3(0.4f64, 0.5f64, 0.25f64));
}

#[test]
fn vec3_add_assign_work() {
    let mut lhs = Vec3(2.0f64, 3.0f64, 4.0f64);
    lhs += Vec3(1.5f64, 2.5f64, 3.5f64);
    assert_eq!(lhs, Vec3(3.5f64, 5.5f64, 7.5f64));
}

#[test]
fn vec3_sub_assign_work() {
    let mut lhs = Vec3(2.0f64, 3.0f64, 4.0f64);
    lhs -= Vec3(1.5f64, 2.5f64, 3.5f64);
    assert_eq!(lhs, Vec3(0.5f64, 0.5f64, 0.5f64));
}

#[test]
fn vec3_mul_assign_work() {
    let mut lhs = Vec3(2.0f64, 3.0f64, 4.0f64);
    lhs *= Vec3(1.5f64, 2.5f64, 3.5f64);
    assert_eq!(lhs, Vec3(3.0f64, 7.5f64, 14.0f64));

    lhs *= 2.0f64;
    assert_eq!(lhs, Vec3(6.0f64, 15.0f64, 28.0f64));
}

#[test]
fn vec3_div_assign_work() {
    let mut lhs = Vec3(2.0f64, 3.0f64, 4.0f64);
    lhs /= Vec3(0.5f64, 0.5f64, 0.5f64);
    assert_eq!(lhs, Vec3(4.0f64, 6.0f64, 8.0f64));

    lhs /= 2.0f64;
    assert_eq!(lhs, Vec3(2.0f64, 3.0f64, 4.0f64));
}

#[test]
fn vec3_dot_work() {
    let lhs = Vec3(2.0f64, 3.0f64, 4.0f64);
    let rhs = Vec3(1.5f64, 2.5f64, 3.5f64);
    let result = Vec3::dot(lhs, rhs);
    assert_eq!(result, 24.5f64);
}

#[test]
fn vec3_cross_work() {
    let lhs = Vec3(2.0f64, 3.0f64, 4.0f64);
    let rhs = Vec3(1.5f64, 2.5f64, 3.5f64);
    let result = Vec3::cross(lhs, rhs);
    assert_eq!(result, Vec3(0.5f64, -1.0f64, 0.5f64));
}

#[test]
fn vec3_length_work() {
    let rhs = Vec3(3.0f64, -4.0f64, 5.0f64);
    let result = rhs.length();
    assert_eq!(result, 7.0710678118654752440084436210485f64);
}

#[test]
fn vec3_length_squared_work() {
    let rhs = Vec3(-3.0f64, 4.0f64, 5.0f64);
    let result = rhs.length_squared();
    assert_eq!(result, 50.0f64);
}

#[test]
fn vec3_unit_vector_work() {
    let rhs = Vec3(2.0f64, 3.0f64, 4.0f64);
    let result = rhs.unit_vector();
    assert_eq!(
        result,
        Vec3(
            0.37139067635410372629315244769244f64,
            0.55708601453115558943972867153866f64,
            0.74278135270820745258630489538488f64,
        )
    )
}

#[test]
fn vec3_normalize_work() {
    let mut rhs = Vec3(2.0f64, 3.0f64, 4.0f64);
    rhs.normalize();
    assert_eq!(
        rhs,
        Vec3(
            0.37139067635410372629315244769244f64,
            0.55708601453115558943972867153866f64,
            0.74278135270820745258630489538488f64,
        )
    )
}

#[test]
fn ray_at_work() {
    let lhs = Ray::new(
        Vec3(1.0f64, 2.0f64, 3.0f64),
        Vec3(3.0f64, 4.0f64, 5.0f64),
        10,
    );
    let result = lhs.at(2.0f64);
    assert_eq!(result, Vec3(7.0f64, 10.0f64, 13.0f64));
}

#[test]
fn random_range_work() {
    for _ in 0..10 {
        let result = random_range(-1.0, 1.0);
        assert!(result > -1.0 && result < 1.0)
    }
}

#[test]
fn random_unit_sphere_work() {
    for _ in 0..10 {
        let result = random_unit_sphere();
        assert!(result.length_squared() < 1.0)
    }
}

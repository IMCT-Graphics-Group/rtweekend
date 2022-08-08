use crate::*;

pub struct Config {
    pub file_path: String,

    pub camera: Camera,

    pub image_width: u32,
    pub image_height: u32,
    pub samples_per_pixel: u32,

    pub scene: Scene,
}

impl Default for Config {
    fn default() -> Self {
        //Camera Settings
        let look_from = Point3::new_point3(13.0, 2.0, 3.0);
        let look_at = Point3::new_point3(0.0, 0.0, 0.0);
        let view_up = Vec3(0.0, 1.0, 0.0);
        let field_of_view = 20.0;
        let aspect_ratio = 3.0 / 2.0;
        let aperture = 0.1;
        let focus_distance = 10.0;
        let ray_depth: u32 = 10;

        //Film Settings
        let image_width: u32 = 1200;
        let image_height = ((image_width as f64) / aspect_ratio) as u32;
        let samples_per_pixel: u32 = 10;

        Self {
            file_path: String::from("image.png"),
            camera: Camera::new(
                look_from,
                look_at,
                view_up,
                field_of_view,
                aspect_ratio,
                aperture,
                focus_distance,
                ray_depth,
            ),
            image_width,
            image_height,
            samples_per_pixel,
            scene: test_scene(),
        }
    }
}

impl Config {
    pub fn new() -> Self {
        Self::default()
    }
}

fn initial_scene() -> Scene {
    let mut scene = Scene::new();

    let material_ground: MaterialType =
        Arc::new(Box::new(Lambertian::new(Color::new_color(0.8, 0.8, 0.0))));
    let material_center: MaterialType =
        Arc::new(Box::new(Lambertian::new(Color::new_color(0.1, 0.2, 0.5))));
    let material_left: MaterialType = Arc::new(Box::new(Dielectric::new(1.5)));
    let material_right: MaterialType =
        Arc::new(Box::new(Metal::new(Color::new_color(0.8, 0.6, 0.2), 0.0)));

    //center_sphere
    scene.add_object(Arc::new(Box::new(Sphere::new(
        Point3::new_point3(0.0, 0.0, -1.0),
        0.5,
        material_center.clone(),
    ))));

    //ground_sphere
    scene.add_object(Arc::new(Box::new(Sphere::new(
        Point3::new_point3(0.0, -100.5, -1.0),
        100.0,
        material_ground.clone(),
    ))));

    //left_sphere
    scene.add_object(Arc::new(Box::new(Sphere::new(
        Point3::new_point3(-1.0, 0.0, -1.0),
        0.5,
        material_left.clone(),
    ))));

    //left_inner_sphere
    scene.add_object(Arc::new(Box::new(Sphere::new(
        Point3::new_point3(-1.0, 0.0, -1.0),
        -0.45,
        material_left.clone(),
    ))));

    //right_sphere
    scene.add_object(Arc::new(Box::new(Sphere::new(
        Point3::new_point3(1.0, 0.0, -1.0),
        0.5,
        material_right.clone(),
    ))));

    scene
}

fn test_scene() -> Scene {
    let mut scene = Scene::new();

    let material_ground: MaterialType =
        Arc::new(Box::new(Lambertian::new(Color::new_color(0.5, 0.5, 0.5))));

    scene.add_object(Arc::new(Box::new(Sphere::new(
        Point3::new_point3(0.0, -1000.0, 0.0),
        1000.0,
        material_ground.clone(),
    ))));

    for a in -11..11 {
        for b in -11..11 {
            let choose_mat = random_01();
            let center = Point3::new_point3(
                a as f64 + 0.9 * random_01(),
                0.2,
                b as f64 + 0.9 * random_01(),
            );

            if (center - Point3::new_point3(4.0, 0.2, 0.0)).length() > 0.9 {
                let sphere_material: MaterialType;

                if choose_mat < 0.8 {
                    //diffuse
                    let albedo = Color::new_color(random_01(), random_01(), random_01());

                    sphere_material = Arc::new(Box::new(Lambertian::new(albedo)));

                    scene.add_object(Arc::new(Box::new(Sphere::new(
                        center,
                        0.2,
                        sphere_material.clone(),
                    ))));
                } else if choose_mat < 0.95 {
                    //metal
                    let albedo = Color::new_color(
                        random_range(0.5, 1.0),
                        random_range(0.5, 1.0),
                        random_range(0.5, 1.0),
                    );

                    let fuzz = random_range(0.0, 0.5);

                    sphere_material = Arc::new(Box::new(Metal::new(albedo, fuzz)));

                    scene.add_object(Arc::new(Box::new(Sphere::new(
                        center,
                        0.2,
                        sphere_material.clone(),
                    ))));
                } else {
                    //glass
                    sphere_material = Arc::new(Box::new(Dielectric::new(1.5)));
                    scene.add_object(Arc::new(Box::new(Sphere::new(
                        center,
                        0.2,
                        sphere_material.clone(),
                    ))));
                }
            }
        }
    }

    let material_1: MaterialType = Arc::new(Box::new(Dielectric::new(1.5)));
    scene.add_object(Arc::new(Box::new(Sphere::new(
        Point3::new_point3(0.0, 1.0, 0.0),
        1.0,
        material_1.clone(),
    ))));

    let material_2: MaterialType =
        Arc::new(Box::new(Lambertian::new(Color::new_color(0.4, 0.2, 0.1))));
    scene.add_object(Arc::new(Box::new(Sphere::new(
        Point3::new_point3(-4.0, 1.0, 0.0),
        1.0,
        material_2.clone(),
    ))));

    let material_3: MaterialType =
        Arc::new(Box::new(Metal::new(Color::new_color(0.7, 0.6, 0.5), 0.0)));
    scene.add_object(Arc::new(Box::new(Sphere::new(
        Point3::new_point3(4.0, 1.0, 0.0),
        1.0,
        material_3.clone(),
    ))));

    scene
}

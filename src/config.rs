use crate::{*, geometry::{sphere, triangle}};

pub struct Config {
    pub file_path: String,

    pub camera: Camera,

    pub image_width: u32,
    pub image_height: u32,
    pub samples_per_pixel: u32,

    pub scene: Scene,

    pub background: Color
}

impl Default for Config {
    fn default() -> Self {
        //Camera Settings
        let look_from = Point3::new_point3(0.0, 2.0, 10.0);
        let look_at = Point3::new_point3(0.0, 1.0, 0.0);
        let view_up = Vec3(0.0, 1.0, 0.0);
        let field_of_view = 20.0;
        let aspect_ratio = 3.0 / 2.0;
        let aperture = 0.1;
        let focus_distance = 10.0;
        let ray_depth: u32 = 20;

        //Film Settings
        let image_width: u32 = 800;
        let image_height = ((image_width as f64) / aspect_ratio) as u32;
        let samples_per_pixel: u32 = 1000;

        let background = Color::new_color(0.0, 0.0, 0.0);

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
            background,
        }
    }
}

impl Config {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn new_Config(
        file_path:String,
        camera:Camera,
        image_width:u32,
        image_height:u32,
        samples_per_pixel:u32,
        scene:Scene,
        background:Color,
    ) -> Self{
        Self { 
            file_path, 
            camera, 
            image_width, 
            image_height, 
            samples_per_pixel, 
            scene, 
            background
        }
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

    scene.build_bvh();
    scene
}

fn test_scene() -> Scene {
    let mut scene = Scene::new();

    let checker:TextureType = Arc::new(Box::new(Checker::new(
        Color::new_color(0.2, 0.3, 0.1),
        Color::new_color(0.9, 0.9, 0.9)
    )));
    
    let material_ground: MaterialType =
        Arc::new(Box::new(Lambertian::new_texture(checker.clone())));

    // let material_ground: MaterialType = 
    //     Arc::new(Box::new(Lambertian::new(Color::new_color(0.5, 0.5, 0.5))));

    scene.add_object(Arc::new(Box::new(Sphere::new(
        Point3::new_point3(0.0, -1000.0, 0.0),
        1000.0,
        material_ground.clone(),
    ))));

    // for a in -11..11 {
    //     for b in -11..11 {
    //         let choose_mat = random_01();
    //         let center = Point3::new_point3(
    //             a as f64 + 0.9 * random_01(),
    //             0.2,
    //             b as f64 + 0.9 * random_01(),
    //         );

    //         if (center - Point3::new_point3(4.0, 0.2, 0.0)).length() > 0.9 {
    //             let sphere_material: MaterialType;

    //             if choose_mat < 0.8 {
    //                 //diffuse
    //                 let albedo = Color::new_color(random_01(), random_01(), random_01());

    //                 sphere_material = Arc::new(Box::new(Lambertian::new(albedo)));

    //                 scene.add_object(Arc::new(Box::new(Sphere::new(
    //                     center,
    //                     0.2,
    //                     sphere_material.clone(),
    //                 ))));
    //             } else if choose_mat < 0.95 {
    //                 //metal
    //                 let albedo = Color::new_color(
    //                     random_range(0.5, 1.0),
    //                     random_range(0.5, 1.0),
    //                     random_range(0.5, 1.0),
    //                 );

    //                 let fuzz = random_range(0.0, 0.5);

    //                 sphere_material = Arc::new(Box::new(Metal::new(albedo, fuzz)));

    //                 scene.add_object(Arc::new(Box::new(Sphere::new(
    //                     center,
    //                     0.2,
    //                     sphere_material.clone(),
    //                 ))));
    //             } else {
    //                 //glass
    //                 sphere_material = Arc::new(Box::new(Dielectric::new(1.5)));
    //                 scene.add_object(Arc::new(Box::new(Sphere::new(
    //                     center,
    //                     0.2,
    //                     sphere_material.clone(),
    //                 ))));
    //             }
    //         }
    //     }
    // }

    // let material_1: MaterialType = Arc::new(Box::new(Dielectric::new(1.5)));
    // scene.add_object(Arc::new(Box::new(Sphere::new(
    //     Point3::new_point3(0.0, 1.0, 0.0),
    //     1.0,
    //     material_1.clone(),
    // ))));
    
    // let material_2: MaterialType =
    //     Arc::new(Box::new(Lambertian::new(Color::new_color(0.4, 0.2, 0.1))));
    // scene.add_object(Arc::new(Box::new(Sphere::new(
    //     Point3::new_point3(-4.0, 1.0, 0.0),
    //     1.0,
    //     material_2.clone(),
    // ))));

    // let material_3: MaterialType =
    //     Arc::new(Box::new(Metal::new(Color::new_color(0.7, 0.6, 0.5), 0.0)));
    // scene.add_object(Arc::new(Box::new(Sphere::new(
    //     Point3::new_point3(4.0, 1.0, 0.0),
    //     1.0,
    //     material_3.clone(),
    // ))));

    // let earth_texture:TextureType = Arc::new(Box::new(ImageTexture::new_from_file(String::from("earthmap.jpg"))));
    // let material_earth: MaterialType =
    //     Arc::new(Box::new(Lambertian::new_texture(earth_texture.clone())));
    // scene.add_object(Arc::new(Box::new(Sphere::new(
    //     Point3::new_point3(4.0, 0.5, 2.0),
    //     0.5,
    //     material_earth.clone(),
    // ))));

    // scene.add_object(Arc::new(Box::new(YZrect::new(
    //     0.0,5.0,-5.0,5.0,0.0,material_3.clone()
    // ))));


    //测试三角形
    // let material_4: MaterialType =
    //     Arc::new(Box::new(Lambertian::new(Color::new_color(0.8, 0.2, 0.1))));

    // let p0 = Point3::new_point3(6.0, 2.0, 1.0);
    // let p1 = Point3::new_point3(6.0, 0.0, 2.0);
    // let p2 = Point3::new_point3(6.0, 0.0, 0.0);

    // let triangle:ObjectType = Arc::new(Box::new(
    //     Triangle::new_vertices(p0,p1,p2,material_4.clone())
    // ));
    // scene.add_object(triangle);

    let light:MaterialType = 
        Arc::new(Box::new(DiffuseLight::new_color(Color::new_color(6.0, 6.0, 6.0))));

    scene.add_object(Arc::new(Box::new(XZrect::new(
             -1.0,1.0,-2.0,0.0,3.0,light.clone()
    ))));

    let material_1: MaterialType = Arc::new(Box::new(Dielectric::new(1.5)));

    let material_3: MaterialType =
         Arc::new(Box::new(Metal::new(Color::new_color(0.8, 0.8, 0.8), 0.0)));

    let material_4: MaterialType =
         Arc::new(Box::new(Lambertian::new(Color::new_color(0.8, 0.8, 0.8))));
    // let bunny:ObjectType = Arc::new(Box::new(
    //     MeshObject::new_from_file(
    //         String::from("stanford_bunny.ply"), 
    //         material_4.clone(),
    //         1.0
    //     )
    // ));

    let mut bunny = create_ply_mesh_triangles(
        String::from("stanford_bunny.ply"), 
        material_4.clone(), 
        false,
        1.0
    );

    scene.add_objects(& mut bunny);

    
    // scene.add_object(Arc::new(Box::new(Sphere::new(
    //     Point3::new_point3(0.0, 1.0, 0.0),
    //     1.0,
    //     material_3.clone(),
    // ))));


    scene.build_bvh();
    scene
}

pub fn simple_light() -> Scene{
    let mut scene = Scene::new();

    let material_metal: MaterialType =
        Arc::new(Box::new(Metal::new(Color::new_color(0.7, 0.6, 0.5), 0.0)));
    
    let earth_texture:TextureType = Arc::new(Box::new(ImageTexture::new_from_file(String::from("earthmap.jpg"))));
    let material_earth: MaterialType =
        Arc::new(Box::new(Lambertian::new_texture(earth_texture.clone())));
    
    scene.add_object(Arc::new(Box::new(Sphere::new(
        Point3::new_point3(0.0, -1000.0, 0.0),
        1000.0,
        material_metal.clone(),
    ))));

    scene.add_object(Arc::new(Box::new(Sphere::new(
        Point3::new_point3(0.0, 2.0, 0.0),
        2.0,
        material_earth.clone(),
    ))));

    let difflight:MaterialType = 
        Arc::new(Box::new(DiffuseLight::new_color(Color::new_color(4.0, 4.0, 4.0))));
    
    scene.add_object(Arc::new(Box::new(XYrect::new(
        3.0,5.0,1.0,3.0,-2.0,difflight
    ))));
    scene.build_bvh();
    scene

}

fn cornell_box() -> Scene{
    let mut scene = Scene::new();

    let red:MaterialType = 
        Arc::new(Box::new(Lambertian::new(Color::new_color(0.65, 0.05, 0.05))));
    let white:MaterialType = 
        Arc::new(Box::new(Lambertian::new(Color::new_color(0.73, 0.73, 0.73))));
    let green:MaterialType = 
        Arc::new(Box::new(Lambertian::new(Color::new_color(0.12, 0.45, 0.15))));
    let light:MaterialType = 
        Arc::new(Box::new(DiffuseLight::new_color(Color::new_color(15.0, 15.0, 15.0))));

    scene.add_object(Arc::new(Box::new(YZrect::new(
        0.0,555.0,0.0,555.0,555.0,green.clone()
    ))));
    scene.add_object(Arc::new(Box::new(YZrect::new(
        0.0,555.0,0.0,555.0,0.0,red.clone()
    ))));
    scene.add_object(Arc::new(Box::new(XZrect::new(
        213.0,343.0,227.0,332.0,554.0,light.clone()
    ))));
    scene.add_object(Arc::new(Box::new(XZrect::new(
        0.0,555.0,0.0,555.0,0.0,white.clone()
    ))));
    scene.add_object(Arc::new(Box::new(XZrect::new(
        0.0,555.0,0.0,555.0,555.0,white.clone()
    ))));
    scene.add_object(Arc::new(Box::new(XYrect::new(
        0.0,555.0,0.0,555.0,555.0,white.clone()
    ))));

    let mut box1:ObjectType = Arc::new(Box::new(MyBox::new(
        Point3::new_point3(0.0,0.0,0.0),
        Point3::new_point3(165.0, 330.0, 165.0),
        white.clone()
    )));
    box1 = Arc::new(Box::new(RotateY::new(box1, 15.0)));
    box1 = Arc::new(Box::new(Translate::new(box1, Vec3(265.0, 0.0, 295.0))));
    scene.add_object(box1);
    
    
    let mut box2:ObjectType = Arc::new(Box::new(MyBox::new(
        Point3::new_point3(0.0,0.0,0.0),
        Point3::new_point3(165.0, 165.0, 165.0),
        white.clone()
    )));
    box2 = Arc::new(Box::new(RotateY::new(box2, -18.0)));
    box2 = Arc::new(Box::new(Translate::new(box2, Vec3(130.0, 0.0, 65.0))));
    scene.add_object(box2);
    
    scene.build_bvh();
    scene
    
}

fn final_scene() -> Scene{
    let mut boxes1:Vec<ObjectType> = Vec::new();
    let ground:MaterialType = Arc::new(Box::new(Lambertian::new(Color::new_color(0.48, 0.83, 0.53))));

    let boxes_per_side = 20;
    for i in 0..boxes_per_side{
        for j in 0..boxes_per_side{
            let w = 100.0;
            let x0 = -1000.0 + i as f64*w;
            let z0 = -1000.0 + j as f64*w;
            let y0 = 0.0;
            let x1 = x0 + w;
            let y1 = random_range(1.0, 81.0);
            let z1 = z0 + w;
            
            boxes1.push(Arc::new(Box::new(
                MyBox::new(
                    Point3::new_point3(x0, y0, z0), 
                    Point3::new_point3(x1, y1, z1), 
                    ground.clone()
                )
            )))
        }
    }

    let mut scene = Scene::new();
    for box1 in boxes1{
        scene.add_object(box1);
    }

    let light:MaterialType = Arc::new(Box::new(
        DiffuseLight::new_color(Color::new_color(7.0, 7.0, 7.0))
    ));
    scene.add_object(
        Arc::new(Box::new(XZrect::new(123.0, 423.0, 147.0, 412.0, 554.0, light.clone())))
    );

    let center1 = Point3::new_point3(400.0,400.0,200.0);
    //let center2 = center1 + Vec3(30.0,0.0,0.0);

    let sphere_material:MaterialType = Arc::new(Box::new(Lambertian::new(Color::new_color(0.7, 0.3, 0.1))));
    scene.add_object(Arc::new(Box::new(Sphere::new(center1, 50.0, sphere_material.clone()))));

    scene.add_object(Arc::new(Box::new(Sphere::new(
        Point3::new_point3(260.0, 150.0, 45.0), 
        50.0, 
        Arc::new(Box::new(Dielectric::new(1.5)))
    ))));
    scene.add_object(Arc::new(Box::new(Sphere::new(
        Point3::new_point3(0.0, 150.0, 145.0), 
        50.0, 
        Arc::new(Box::new(Metal::new(Color::new_color(0.8,0.8, 0.9), 1.0)))
    ))));

    let boundary1:ObjectType = Arc::new(Box::new(
        Sphere::new(
            Point3::new_point3(360.0, 150.0, 145.0), 
            70.0, 
            Arc::new(Box::new(Dielectric::new(1.5)))
        )
    ));
    scene.add_object(boundary1.clone());
    let medium1:ObjectType = Arc::new(Box::new(
        ConstantMedium::new(
            boundary1.clone(), 
            0.2, 
            Color::new_color(0.2, 0.2, 1.0))
    ));
    scene.add_object(medium1);
    
    // let boundary2:ObjectType = Arc::new(Box::new(
    //     Sphere::new(
    //         Point3::new_point3(0.0, 0.0, 0.0), 
    //         5000.0, 
    //         Arc::new(Box::new(Dielectric::new(1.5)))
    //     )
    // ));
    // scene.add_object(boundary2.clone());
    // let medium2:ObjectType = Arc::new(Box::new(
    //     ConstantMedium::new(
    //         boundary2.clone(), 
    //         0.0001, 
    //         Color::new_color(1.0, 1.0, 1.0))
    // ));
    // scene.add_object(medium2);

    let emat:MaterialType = Arc::new(Box::new(
        Lambertian::new_texture(Arc::new(Box::new(ImageTexture::new_from_file(String::from("earthmap.jpg")))))
    ));
    scene.add_object(Arc::new(Box::new(
        Sphere::new(
            Point3::new_point3(400.0, 200.0, 400.0), 
            100.0, 
            emat.clone())
    )));

    let metalMat:MaterialType = Arc::new(Box::new(
        Metal::new(Color::new_color(0.7, 0.6, 0.5), 0.4)
    ));
    scene.add_object(Arc::new(Box::new(
        Sphere::new(
            Point3::new_point3(220.0, 280.0, 300.0), 
            80.0, 
            metalMat.clone())
    )));


    let mut boxes2:Vec<ObjectType> = Vec::new();
    let white:MaterialType = Arc::new(Box::new(
        Lambertian::new(Color::new_color(0.73, 0.73, 0.73))
    ));
    let ns = 1000;
    for j in 0..ns{
        let sphereInBox:ObjectType = Arc::new(Box::new(
            Sphere::new(
                Point3::new_point3(random_range(0.0,165.0), random_range(0.0,165.0), random_range(0.0,165.0)), 
                10.0, 
                white.clone()
            )
        ));

        let TranslateSphere:ObjectType = Arc::new(Box::new(
            Translate::new(sphereInBox, Vec3(-100.0, 270.0, 395.0))
        ));
        
        boxes2.push(TranslateSphere)
    }

    for box2 in boxes2{
        scene.add_object(box2);
    }


    scene.build_bvh();
    scene


}

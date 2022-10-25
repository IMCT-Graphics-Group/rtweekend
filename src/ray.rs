use crate::*;

#[derive(Clone, Debug, Default)]
pub struct Ray {
    pub orig: Point3,
    pub dir: Vec3,
    pub depth: u32,
}

impl Ray {
    pub fn new_default() -> Ray{
        Ray { 
            orig: Point3::new_point3(0.0, 0.0, 0.0), 
            dir: Vec3(1.0, 0.0, 0.0), 
            depth: 0 
        }
    }
    
    pub fn new(orig: Point3, dir: Vec3, depth: u32) -> Ray {
        Ray { orig, dir, depth }
    }

    pub fn at(&self, t: f64) -> Point3 {
        self.orig + self.dir * t
    }
}

pub fn ray_color(ray: Ray, config: &Config, lights: HittableType) -> Color {
    if ray.depth == 0 {
        return Color::new_color(0.0, 0.0, 0.0);
    }

    if let Some(hit_record) = config.scene.hit(&ray, (1e-8, f64::INFINITY)) {
        let emitted = hit_record.hit_material.emitted(hit_record.u, hit_record.v, &hit_record.hit_point);
        
        if let Some(srec) = hit_record.hit_material.scatter_mc(&ray, &hit_record) {
            //return albedo * ray_color(scattered, config);
            // let light_pdf = Hittable_pdf::new(lights.clone(), &hit_record.hit_point);
            // scattered.dir = light_pdf.generate();
            // pdf_val = light_pdf.value(&scattered.dir);
            
            //cosine pdf
            // let pdf = Cosine_pdf::new(&hit_record.hit_normal);
            // scattered.dir = pdf.generate();
            // pdf_val = pdf.value(&scattered.dir);

            //mixture pdf
            // let p0:Arc<Box<dyn Pdf + 'static>> = Arc::new(Box::new(Hittable_pdf::new(
            //     lights.clone(), &hit_record.hit_point
            // )));
            // let p1:Arc<Box<dyn Pdf + 'static>> = Arc::new(Box::new(Cosine_pdf::new(&hit_record.hit_normal)));
            // let mixed_pdf = Mixture_pdf::new(p0, p1);
            // scattered.dir = mixed_pdf.generate();
            // pdf_val = mixed_pdf.value(&scattered.dir);

            //lambertian模型中srec的attenuation相当于微表面模型中慢反射项的kd，所以乘于scattering_pdf = cos\theta/PI，其中cos\theta其实就是渲染方程中的余弦项，attenuation/PI才是BRDF值

            if srec.is_specular{
                return srec.attenuation * ray_color(srec.specular_ray, config, lights);
            }

            let light_pdf:Arc<Box<dyn Pdf + 'static>> = Arc::new(Box::new(Hittable_pdf::new(
                     lights.clone(), &hit_record.hit_point
            )));
            let mixed_pdf = Mixture_pdf::new(light_pdf, srec.pdf_ptr.clone());

            let scattered = Ray::new(hit_record.hit_point, mixed_pdf.generate(), ray.depth-1);
            let pdf_val = mixed_pdf.value(&scattered.dir);

            return emitted + srec.attenuation * hit_record.hit_material.scattering_pdf(&ray, &hit_record, &scattered)
            * ray_color(scattered, config, lights) / pdf_val;

        }
        else{
            return emitted;
        }
    } else {
        // let t = 0.5 * (ray.dir.unit_vector().y() + 1.0);
        // Color::new_color(1.0, 1.0, 1.0) * (1.0 - t) + Color::new_color(0.5, 0.7, 1.0) * t
        return config.background;
    }
}


//基于多重重要性采样MIS的路径追踪
pub fn ray_color_mis(ray: &Ray, config: &Config, lights: HittableType, mis_weight: f64) -> Color {
    if ray.depth == 0 {
        return Color::new_color(0.0, 0.0, 0.0);
    }

    if let Some(hit_record) = config.scene.hit(&ray, (1e-8, f64::INFINITY)) {
        let l_e = hit_record.hit_material.emitted(hit_record.u, hit_record.v, &hit_record.hit_point) * mis_weight;
        
        if let Some(srec) = hit_record.hit_material.scatter_mc(&ray, &hit_record) {

            //lambertian模型中srec的attenuation相当于微表面模型中慢反射项的kd，所以乘于scattering_pdf = cos\theta/PI，其中cos\theta其实就是渲染方程中的余弦项，attenuation/PI才是BRDF值

            if srec.is_specular{
                return srec.attenuation * ray_color(srec.specular_ray, config, lights);
            }

            let light_pdf:Arc<Box<dyn Pdf + 'static>> = Arc::new(Box::new(Hittable_pdf::new(
                     lights.clone(), &hit_record.hit_point
            )));

            //MIS
            let n = config.samples_per_pixel as f64;
            //sampling the light
            let scattered1 = Ray::new(hit_record.hit_point, light_pdf.generate(), ray.depth-1);
            let mis_weight1 = (n * light_pdf.value(&scattered1.dir)).powf(2.0) / 
            ((n * light_pdf.value(&scattered1.dir)).powf(2.0) + (n * srec.pdf_ptr.value(&scattered1.dir).powf(2.0)));                                              
            let l_dir = srec.attenuation * hit_record.hit_material.scattering_pdf(&ray, &hit_record, &scattered1) 
            * ray_color_mis(&scattered1, config, lights.clone(), mis_weight1) / light_pdf.value(&scattered1.dir);
            
            //samping the cosine
            let scattered2 = Ray::new(hit_record.hit_point, srec.pdf_ptr.generate(), ray.depth-1);
            let mis_weight2 = (n * srec.pdf_ptr.value(&scattered2.dir)).powf(2.0) / 
            ((n * light_pdf.value(&scattered2.dir)).powf(2.0) + (n * srec.pdf_ptr.value(&scattered2.dir).powf(2.0)));
            let l_ind = srec.attenuation * hit_record.hit_material.scattering_pdf(&ray, &hit_record, &scattered2) 
            * ray_color_mis(&scattered2, config, lights.clone(), mis_weight2) / srec.pdf_ptr.value(&scattered2.dir);

            return l_e + l_dir + l_ind;
        }
        else{
            return l_e;
        }
    } else {
        // let t = 0.5 * (ray.dir.unit_vector().y() + 1.0);
        // Color::new_color(1.0, 1.0, 1.0) * (1.0 - t) + Color::new_color(0.5, 0.7, 1.0) * t
        return config.background;
    }
}
use crate::*;
use std::collections::HashMap;
use std::io::BufReader;
use ply_rs::parser;
use ply_rs::ply;
use std::hash::BuildHasher;
use std::fs::File;

#[derive(Clone)]
pub struct TriangleMesh{
    pub n_triangles: u32, //网格中的三角形总数
    pub vertex_indices: Vec<u32>, //存放顶点索引的容器
    pub n_vertices: u32, //网格中的顶点总数
    pub p: Vec<Point3>, //存放顶点的容器
    pub n: Vec<Vec3>, //存放各顶点法线的容器
    pub s: Vec<Vec3>, //存放各顶点切线的容器
    pub uv: Vec<Point2>, //存放各顶点uv的容器

    pub is_smooth: bool, //十分平滑处理
    
    // pub alpha_mask: Option<Arc<dyn Texture<Float> + Send + Sync>>,
    // pub shadow_alpha_mask: Option<Arc<dyn Texture<Float> + Send + Sync>>,
    // // inherited from class Shape (see shape.h)
    //pub object_to_world: Transform, // TODO: not pub?
    //pub world_to_object: Transform, // TODO: not pub?
    pub reverse_orientation: bool,
    //pub transform_swaps_handedness: bool, // TODO: not pub?
    pub material: MaterialType,
}

impl TriangleMesh {
    pub fn new(
        reverse_orientation: bool,
        n_triangles: u32,
        vertex_indices: Vec<u32>,
        n_vertices: u32,
        p: Vec<Point3>,
        s: Vec<Vec3>,
        n: Vec<Vec3>,
        uv: Vec<Point2>,
        is_smooth: bool,
        material: MaterialType,
    ) -> Self{
        TriangleMesh {
            n_triangles,
            vertex_indices,
            n_vertices,
            p,
            n,
            s,
            uv,
            is_smooth,
            reverse_orientation,
            material,
        }
    }
}

// pub struct MeshObject{
//     triangles: Vec<Triangle>
// }

// impl MeshObject {
//     pub fn new(triangles:Vec<Triangle>) -> Self{
//         MeshObject {triangles}
//     }

//     pub fn new_from_file(filename:String, material:MaterialType, scale: f64) -> Self{
//         create_ply_mesh(filename, material, scale)
//     }

// }

// impl Hittable for MeshObject {
//     fn hit(&self, ray: &Ray, t_range: (f64, f64)) -> Option<HitRecord> {
//         for triangle in self.triangles.iter(){
//             if let Some(rec) = triangle.hit(ray, t_range) {
//                 return Some(rec);
//             }
//         }
        
//         None
//     }
// }

// impl Bounded for MeshObject {
//     fn bounding_box(&self) -> AABB {
//         let mut aabb = self.triangles[0].bounding_box();
//         for id in 1..self.triangles.len(){
//             aabb = aabb + self.triangles[id].bounding_box();
//         }

//         aabb
//     }
// }


#[derive(Clone)]
pub struct Triangle{
    mesh: Arc<TriangleMesh>, //指向三角形网格的指针
    pub id:u32, //在网格顶点数组中第一个顶点索引
    
    //临时测试用
    // pub p0: Point3,
    // pub p1: Point3,
    // pub p2: Point3,

    // pub material:MaterialType,
}

impl Triangle {
    pub fn new(mesh:Arc<TriangleMesh>, id:u32) -> Self{
        Triangle { mesh, id}
    }

    // pub fn new_vertices(p0:Point3,p1:Point3,p2:Point3,material:MaterialType) -> Self{
    //     Triangle{p0,p1,p2,material}
    // }

    // pub fn object_bound(&self) -> AABB{
    //     let (p0,p1,p2) = self.get_vertices();

    //     AABB::union_point3(
    //         &AABB::new(
    //             self.mesh.world_to_object.transform_point(p0), 
    //             self.mesh.world_to_object.transform_point(p1),
    //         ),
    //         &self.mesh.world_to_object.transform_point(p2),
    //     )
    // }

    pub fn world_bound(&self) -> AABB{
        let (p0,p1,p2) = self.get_vertices();

        //let offset = Vec3(0.001,0.0,0.0);

        AABB::union_point3(&AABB::new_abitary(&p0 , &p1), p2)
    }

    pub fn get_vertices(&self) -> (&Point3,&Point3,&Point3){
        let idx1 = (self.id * 3) as usize;
        let idx = &self.mesh.vertex_indices[idx1..(idx1 + 3)];
        let p0: &Point3 = &self.mesh.p[idx[0] as usize];
        let p1: &Point3 = &self.mesh.p[idx[1] as usize];
        let p2: &Point3 = &self.mesh.p[idx[2] as usize];

        (p0,p1,p2)
    }

    pub fn get_uvs(&self) -> [Point2; 3]{
        if self.mesh.uv.is_empty(){
            [
                Point2(0.0,0.0),
                Point2(0.0,0.0),
                Point2(0.0,0.0),
            ]
        } else {
            [
                self.mesh.uv[self.mesh.vertex_indices[(self.id * 3) as usize] as usize],
                self.mesh.uv[self.mesh.vertex_indices[(self.id * 3) as usize + 1] as usize],
                self.mesh.uv[self.mesh.vertex_indices[(self.id * 3) as usize + 2] as usize],
            ]
        }
    }

    pub fn get_normals(&self) -> [Vec3; 3]{
        if self.mesh.n.is_empty(){
            [
                Vec3(0.0,0.0,0.0),
                Vec3(0.0,0.0,0.0),
                Vec3(0.0,0.0,0.0),
            ]
        } else {
            [
                self.mesh.n[self.mesh.vertex_indices[(self.id * 3) as usize] as usize],
                self.mesh.n[self.mesh.vertex_indices[(self.id * 3) as usize + 1] as usize],
                self.mesh.n[self.mesh.vertex_indices[(self.id * 3) as usize + 2] as usize],
            ]
        }
    }
    
}

impl Hittable for Triangle {
    fn hit(&self, ray: &Ray, t_range: (f64, f64)) -> Option<HitRecord> {
        
        let(tmin, tmax) = t_range;
        
        //获得三角形三个顶点坐标
        let (p0,p1,p2) = self.get_vertices();
        // let p0 = &self.p0;
        // let p1 = &self.p1;
        // let p2 = &self.p2;

        //将三个顶点变换到射线空间：射线起点在坐标原点，方向沿+z
        let mut p0t:Point3 = *p0 - Vec3(ray.orig.0, ray.orig.1, ray.orig.2);
        let mut p1t:Point3 = *p1 - Vec3(ray.orig.0, ray.orig.1, ray.orig.2);
        let mut p2t:Point3 = *p2 - Vec3(ray.orig.0, ray.orig.1, ray.orig.2);
        // permute components of triangle vertices and ray direction
        let kz:usize = ray.dir.abs().max_dimension();
        let mut kx = kz + 1;
        if kx ==3{
            kx = 0;
        }
        let mut ky = kx + 1;
        if ky == 3 {
            ky = 0;
        }
        let d = ray.dir.permute(kx, ky, kz);
        p0t = p0t.permute(kx, ky, kz);
        p1t = p1t.permute(kx, ky, kz);
        p2t = p2t.permute(kx, ky, kz);
        // Apply shear transformation to translated vertex positions
        let sx = -d.0 /d.2;
        let sy = -d.1 /d.2;
        let sz = 1.0 /d.2;
        p0t.0 += sx * p0t.2;
        p0t.1 += sy * p0t.2;
        p1t.0 += sx * p1t.2;
        p1t.1 += sy * p1t.2;
        p2t.0 += sx * p2t.2;
        p2t.1 += sy * p2t.2;
        // compute edge function coefficients _e0_, _e1_, and _e2_
        let e0: Float = p1t.0 * p2t.1 - p1t.1 * p2t.0;
        let e1: Float = p2t.0 * p0t.1 - p2t.1 * p0t.0;
        let e2: Float = p0t.0 * p1t.1 - p0t.1 * p1t.0;
        
        // perform triangle edge and determinant tests
        if (e0 < 0.0 || e1 < 0.0 || e2 < 0.0) && (e0 > 0.0 || e1 > 0.0 || e2 > 0.0) {
            return None;
        }
        let det: Float = e0 + e1 + e2;
        if det == 0.0 {
            return None;
        }
        // compute scaled hit distance to triangle and test against ray $t$ range
        p0t.2 *= sz;
        p1t.2 *= sz;
        p2t.2 *= sz;
        let t_scaled: Float = e0 * p0t.2 + e1 * p1t.2 + e2 * p2t.2;
        if det < 0.0 && (t_scaled >= 0.0 || t_scaled < tmax * det) {
            return None;
        } else if det > 0.0 && (t_scaled <= 0.0 || t_scaled > tmax * det) {
            return None;
        }
        // compute barycentric coordinates and $t$ value for triangle intersection
        let inv_det: Float = 1.0 / det;
        let b0: Float = e0 * inv_det;
        let b1: Float = e1 * inv_det;
        let b2: Float = e2 * inv_det;
        let t: Float = t_scaled * inv_det;

        let p_hit: Point3 = *p0 * b0 + *p1 * b1 + *p2 * b2;

        let mut surface_normal = Vec3(1.0,0.0,0.0);

        if self.mesh.is_smooth{
            let normals:[Vec3; 3] = self.get_normals();
            surface_normal = normals[0] * b0 + normals[1] * b1 + normals[2] * b2;
        }
        else{
            // override surface normal in _isect_ for triangle
            let dp02: Vec3 = *p0 - *p2;
            let dp12: Vec3 = *p1 - *p2;
            surface_normal =
               Vec3::cross(dp02, dp12).unit_vector();
        }

        
        //临时测试用
        let (front_face,hit_normal) = Vec3::set_face_normal(ray.dir, surface_normal);
        // if self.mesh.reverse_orientation ^ self.mesh.transform_swaps_handedness {
        //     surface_normal = surface_normal * -1.0;
        // } 
        
        //计算uv
        let uv: [Point2; 3] = self.get_uvs();
        let hit_uv = uv[0] * b0 + uv[1] * b1 + uv[2] *b2;

        //Tomas Moller的射线三角形相交
        // let edge1 = *p1-*p0;
        // let edge2 = *p2-*p0;

        // let pvec = Vec3::cross_borrow(&ray.dir, &edge2);

        // let det = Vec3::dot_borrow(&edge1, &pvec);

        // if det < 1e-8{
        //     return None;
        // }
        // let inv_det = 1.0/det;

        // let tvec = ray.orig - *p0;

        // let u = Vec3::dot_borrow(&tvec, &pvec) *inv_det;
        // if u<0.0 || u>1.0{
        //     return None;
        // }

        // let qvec = Vec3::cross_borrow(&tvec, &edge1);

        // let v = Vec3::dot_borrow(&ray.dir, &qvec) * inv_det;
        // if v < 0.0 || u+v >1.0{
        //     return None;
        // }

        // let t = Vec3::dot_borrow(&edge2, &qvec) * inv_det;

        // let p_hit = ray.at(t);

        // let surface_normal: Vec3 =
        //      Vec3::cross(edge1, edge2).unit_vector() ;

        //临时测试用
        //let (front_face,hit_normal) = Vec3::set_face_normal(ray.dir, surface_normal);
        
        //println!("Hit point:{}, t is:{}, front_face:{}",p_hit,t,front_face);

        // if p_hit.y() < 3.0{
        //     println!("Hit point:{}, t is:{}, front_face:{}",p_hit,t,front_face);
        // }

        let rec = HitRecord::new(
            p_hit,
            hit_normal,
            self.mesh.material.clone(),
            t,
            hit_uv.0,
            hit_uv.1,
            front_face,
        );

        //println!("HitRecord is {}",rec);
        
        Option::Some(rec)


    }
}

impl Bounded for Triangle {
    fn bounding_box(&self) -> AABB {
        let aabb = self.world_bound();
        //println!("Traingle Bounded is:{},{}",aabb.min(),aabb.max());
        aabb
    }
}

// pub fn create_ply_mesh(filename:String, material:MaterialType, is_smooth: bool ,scale:f64) -> MeshObject{
//     let result = File::open(&filename);
//     if result.is_err(){
//         panic!("Couldn't open PLY file {:?}", filename);
//     }
//     let f = result.unwrap();
//     let mut buf_reader = BufReader::new(f);
//     let p = parser::Parser::<ply::DefaultElement>::new();
//     //header
//     let result = p.read_header(&mut buf_reader);
//     if result.is_err(){
//         panic!("Unable to read the header of PLY file  {:?}", filename);
//     }
//     let header = result.unwrap();
//     //payload
//     let result = p.read_payload(&mut buf_reader, &header);
//     if result.is_err() {
//         panic!("Unable to read the payload of PLY file  {:?}", filename);
//     }
//     let payload = result.unwrap();

//     let mut p:Vec<Point3> = Vec::new();
//     let mut n:Vec<Vec3> = Vec::new();
//     let mut uvs:Vec<Point2> = Vec::new();
//     let mut has_normals:bool = false;
//     let mut has_uvs: bool = false;
//     let mut tm_vertex_indices: Vec<u32> = Vec::new();
//     for (name, list) in payload.into_iter() {
//         match name.as_ref() {
//             "vertex" => {
//                 for elem in list.into_iter(){
//                     let mut pnt: Point3 = Point3::default();
//                     let mut nrm: Vec3 = Vec3::default();
//                     let mut pt2: Point2 = Point2::default();
//                     for (name2,list2) in elem.into_iter(){
//                         match name2.as_ref(){
//                             "x" => {
//                                 if let ply::Property::Float(x) = list2{
//                                     pnt.0 = x as f64 * scale;
//                                 }
//                             }
//                             "y" => {
//                                 if let ply::Property::Float(y) = list2{
//                                     pnt.1 = y as f64 * scale;
//                                 }
//                             }
//                             "z" => {
//                                 if let ply::Property::Float(z) = list2{
//                                     pnt.2 = z as f64 * scale;
//                                 }
//                             }
//                             "nx" => {
//                                 has_normals = true;
//                                 if let ply::Property::Float(x) = list2{
//                                     nrm.0 = x as f64;
//                                 }
//                             }
//                             "ny" => {
//                                 has_normals = true;
//                                 if let ply::Property::Float(y) = list2{
//                                     nrm.1 = y as f64;
//                                 }
//                             }
//                             "nz" => {
//                                 has_normals = true;
//                                 if let ply::Property::Float(z) = list2{
//                                     nrm.2 = z as f64;
//                                 }
//                             }
//                             "u" | "s" => {
//                                 has_uvs = true;
//                                 if let ply::Property::Float(x) = list2 {
//                                     pt2.0 = x as f64;
//                                 }
//                             }
//                             "v" | "t" => {
//                                 has_uvs = true;
//                                 if let ply::Property::Float(y) = list2 {
//                                     pt2.1 = y as f64;
//                                 }
//                             }
//                             _=> {
//                                 println!("name2 = {:?}",name2);
//                                 //unreachable!();
//                             }
//                         }
//                     }
//                     p.push(pnt);
//                     if has_normals{
//                         n.push(nrm);
//                     }
//                     if has_uvs{
//                         uvs.push(pt2);
//                     }
//                 }
//             }
//             "face" => {
//                 for elem in list.into_iter(){
//                     let mut nrm: Vec3 = Vec3::default();
//                     for (name2, list2) in elem.into_iter(){
//                         match name2.as_ref(){
//                             "vertex_indices" => {
//                                 if let ply::Property::ListInt(li) = list2{
//                                     let mut vertex_indices: Vec<usize> = Vec::new();
//                                     for i in li.into_iter() {
//                                         vertex_indices.push(i as usize);
//                                     }
//                                     if vertex_indices.len() != 3{
//                                         if vertex_indices.len() == 4 {
//                                             // handle quads (split it into 2 triangles)
//                                             let v1 = vertex_indices[0];
//                                             let v3 = vertex_indices[2];
//                                             let v4 = vertex_indices.pop().unwrap();
//                                             vertex_indices.push(v4);
//                                             vertex_indices.push(v1);
//                                             vertex_indices.push(v3);
//                                         } else {
//                                              panic!("plymesh: Ignoring face with {} vertices (only triangles and quads are supported!)",
//                                                     vertex_indices.len());
//                                         }
//                                     }
//                                     // now we can add the indices to the triangle mesh vertex indices
//                                     for vi in vertex_indices {
//                                         tm_vertex_indices.push(vi.try_into().unwrap());
//                                     }
                                        
//                                 } else if let ply::Property::ListUInt(li) = list2{
//                                     let mut vertex_indices: Vec<usize> = Vec::new();
//                                     for i in li.into_iter() {
//                                         vertex_indices.push(i as usize);
//                                     }
//                                     if vertex_indices.len() != 3 {
//                                         if vertex_indices.len() == 4 {
//                                             // handle quads (split it into 2 triangles)
//                                             let v1 = vertex_indices[0];
//                                             let v3 = vertex_indices[2];
//                                             let v4 = vertex_indices.pop().unwrap();
//                                             vertex_indices.push(v4);
//                                             vertex_indices.push(v1);
//                                             vertex_indices.push(v3);
//                                         } else {
//                                             panic!("plymesh: Ignoring face with {} vertices (only triangles and quads are supported!)",
//                                                    vertex_indices.len());
//                                         }
//                                     }
//                                     // now we can add the indices to the triangle mesh vertex indices
//                                     for vi in vertex_indices {
//                                         tm_vertex_indices.push(vi.try_into().unwrap());
//                                     }
//                                 }
//                             }
//                             "nx" => {
//                                 has_normals = true;
//                                 if let ply::Property::Float(x) = list2 {
//                                     nrm.0 = x as f64;
//                                 }
//                             }
//                             "ny" => {
//                                 has_normals = true;
//                                 if let ply::Property::Float(y) = list2 {
//                                     nrm.1 = y as f64;
//                                 }
//                             }
//                             "nz" => {
//                                 has_normals = true;
//                                 if let ply::Property::Float(z) = list2 {
//                                     nrm.2 = z as f64;
//                                 }
//                             }
//                             _=> unreachable!(),
//                         }
                        
//                     }
//                 }
//             }
//             _=> unreachable!(),
//         }
//     }

//     let n_vertices: usize = p.len();
//     let s: Vec<Vec3> = Vec::new();

//     //这一部分有将各个顶点转换到世界空间

//     let mesh = Arc::new(TriangleMesh::new(
//         true, 
//         (tm_vertex_indices.len() / 3).try_into().unwrap(), 
//         tm_vertex_indices, 
//         n_vertices.try_into().unwrap(), 
//         p, 
//         s, 
//         n, 
//         uvs, 
//         is_smooth,
//         material
//     ));

//     let mut triangles: Vec<Triangle> = Vec::new();
//     for id in 0..mesh.n_triangles{
//         let triangle = Triangle::new(
//             mesh.clone(), 
//             id.try_into().unwrap()
//         );
//         triangles.push(triangle);
//     }

//     MeshObject { triangles }


// }

pub fn create_ply_mesh_triangles(filename:String, material:MaterialType, is_smooth: bool, scale:f64) -> Vec<ObjectType>{
    let result = File::open(&filename);
    if result.is_err(){
        panic!("Couldn't open PLY file {:?}", filename);
    }
    let f = result.unwrap();
    let mut buf_reader = BufReader::new(f);
    let p = parser::Parser::<ply::DefaultElement>::new();
    //header
    let result = p.read_header(&mut buf_reader);
    if result.is_err(){
        panic!("Unable to read the header of PLY file  {:?}", filename);
    }
    let header = result.unwrap();
    //payload
    let result = p.read_payload(&mut buf_reader, &header);
    if result.is_err() {
        panic!("Unable to read the payload of PLY file  {:?}", filename);
    }
    let payload = result.unwrap();

    let mut p:Vec<Point3> = Vec::new();
    let mut n:Vec<Vec3> = Vec::new();
    let mut uvs:Vec<Point2> = Vec::new();
    let mut has_normals:bool = false;
    let mut has_uvs: bool = false;
    let mut tm_vertex_indices: Vec<u32> = Vec::new();
    for (name, list) in payload.into_iter() {
        match name.as_ref() {
            "vertex" => {
                for elem in list.into_iter(){
                    let mut pnt: Point3 = Point3::default();
                    let mut nrm: Vec3 = Vec3::default();
                    let mut pt2: Point2 = Point2::default();
                    for (name2,list2) in elem.into_iter(){
                        match name2.as_ref(){
                            "x" => {
                                if let ply::Property::Float(x) = list2{
                                    pnt.0 = x as f64 * scale;
                                }
                            }
                            "y" => {
                                if let ply::Property::Float(y) = list2{
                                    pnt.1 = y as f64 * scale;
                                }
                            }
                            "z" => {
                                if let ply::Property::Float(z) = list2{
                                    pnt.2 = z as f64 * scale;
                                }
                            }
                            "nx" => {
                                has_normals = true;
                                if let ply::Property::Float(x) = list2{
                                    nrm.0 = x as f64;
                                }
                            }
                            "ny" => {
                                has_normals = true;
                                if let ply::Property::Float(y) = list2{
                                    nrm.1 = y as f64;
                                }
                            }
                            "nz" => {
                                has_normals = true;
                                if let ply::Property::Float(z) = list2{
                                    nrm.2 = z as f64;
                                }
                            }
                            "u" | "s" => {
                                has_uvs = true;
                                if let ply::Property::Float(x) = list2 {
                                    pt2.0 = x as f64;
                                }
                            }
                            "v" | "t" => {
                                has_uvs = true;
                                if let ply::Property::Float(y) = list2 {
                                    pt2.1 = y as f64;
                                }
                            }
                            _=> {
                                println!("name2 = {:?}",name2);
                                //unreachable!();
                            }
                        }
                    }
                    p.push(pnt);
                    if has_normals{
                        n.push(nrm);
                    }
                    if has_uvs{
                        uvs.push(pt2);
                    }
                }
            }
            "face" => {
                for elem in list.into_iter(){
                    let mut nrm: Vec3 = Vec3::default();
                    for (name2, list2) in elem.into_iter(){
                        match name2.as_ref(){
                            "vertex_indices" => {
                                if let ply::Property::ListInt(li) = list2{
                                    let mut vertex_indices: Vec<usize> = Vec::new();
                                    for i in li.into_iter() {
                                        vertex_indices.push(i as usize);
                                    }
                                    if vertex_indices.len() != 3{
                                        if vertex_indices.len() == 4 {
                                            // handle quads (split it into 2 triangles)
                                            let v1 = vertex_indices[0];
                                            let v3 = vertex_indices[2];
                                            let v4 = vertex_indices.pop().unwrap();
                                            vertex_indices.push(v4);
                                            vertex_indices.push(v1);
                                            vertex_indices.push(v3);
                                        } else {
                                             panic!("plymesh: Ignoring face with {} vertices (only triangles and quads are supported!)",
                                                    vertex_indices.len());
                                        }
                                    }
                                    // now we can add the indices to the triangle mesh vertex indices
                                    for vi in vertex_indices {
                                        tm_vertex_indices.push(vi.try_into().unwrap());
                                    }
                                        
                                } else if let ply::Property::ListUInt(li) = list2{
                                    let mut vertex_indices: Vec<usize> = Vec::new();
                                    for i in li.into_iter() {
                                        vertex_indices.push(i as usize);
                                    }
                                    if vertex_indices.len() != 3 {
                                        if vertex_indices.len() == 4 {
                                            // handle quads (split it into 2 triangles)
                                            let v1 = vertex_indices[0];
                                            let v3 = vertex_indices[2];
                                            let v4 = vertex_indices.pop().unwrap();
                                            vertex_indices.push(v4);
                                            vertex_indices.push(v1);
                                            vertex_indices.push(v3);
                                        } else {
                                            panic!("plymesh: Ignoring face with {} vertices (only triangles and quads are supported!)",
                                                   vertex_indices.len());
                                        }
                                    }
                                    // now we can add the indices to the triangle mesh vertex indices
                                    for vi in vertex_indices {
                                        tm_vertex_indices.push(vi.try_into().unwrap());
                                    }
                                }
                            }
                            "nx" => {
                                has_normals = true;
                                if let ply::Property::Float(x) = list2 {
                                    nrm.0 = x as f64;
                                }
                            }
                            "ny" => {
                                has_normals = true;
                                if let ply::Property::Float(y) = list2 {
                                    nrm.1 = y as f64;
                                }
                            }
                            "nz" => {
                                has_normals = true;
                                if let ply::Property::Float(z) = list2 {
                                    nrm.2 = z as f64;
                                }
                            }
                            _=> unreachable!(),
                        }
                        
                    }
                }
            }
            _=> unreachable!(),
        }
    }

    let n_vertices: usize = p.len();
    let s: Vec<Vec3> = Vec::new();

    //这一部分有将各个顶点转换到世界空间

    let mesh = Arc::new(TriangleMesh::new(
        true, 
        (tm_vertex_indices.len() / 3).try_into().unwrap(), 
        tm_vertex_indices, 
        n_vertices.try_into().unwrap(), 
        p, 
        s, 
        n, 
        uvs, 
        is_smooth,
        material
    ));

    let mut triangles: Vec<ObjectType> = Vec::new();
    for id in 0..mesh.n_triangles{
        let triangle:ObjectType = Arc::new(Box::new(Triangle::new(
            mesh.clone(), 
            id.try_into().unwrap()
        )));
        triangles.push(triangle);
    }

    triangles

}
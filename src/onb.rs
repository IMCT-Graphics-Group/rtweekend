use crate::*;

pub struct Onb{
    pub axis: [Vec3; 3]
}

impl Onb{
    pub fn u(&self) -> Vec3{
        self.axis[0]
    }

    pub fn v(&self) -> Vec3{
        self.axis[1]
    }

    pub fn w(&self) -> Vec3{
        self.axis[2]
    }

    pub fn build_from_w(n:&Vec3) -> Self{
        let w = n.unit_vector();
        let a = if w.x().abs() > 0.9 {Vec3(0.0,1.0,0.0)} else {Vec3(1.0,0.0,0.0)};
        let v = Vec3::cross(w, a).unit_vector();
        let u = Vec3::cross(w, v);
        Onb { axis: [u,v,w] }
    }

    pub fn local(&self, a: &Vec3) -> Vec3{
        self.u() * a.x() + self.v() * a.y() + self.w() * a.z()
    }
}
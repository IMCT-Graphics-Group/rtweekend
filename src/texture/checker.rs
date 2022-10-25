use crate::*;

pub struct Checker{
    pub odd:Color,
    pub even:Color,
}

impl Checker{
    // pub fn new_texture(_even:&TextureType, _odd:&TextureType) -> Checker{
    //     Checker{odd:_odd.clone(), even:_even.clone()}
    // } 

    pub fn new(odd:Color,even:Color) -> Checker{
        Checker{odd,even}
    }
}

impl Texture for Checker{
    fn value(&self, u:f64,v:f64, p:&Point3) -> Color {
        let sines = (10.0*p.x()).sin()*(10.0*p.y()).sin()*(10.0*p.z()).sin();
        if sines < 0.0{
            //return self.odd.value(u, v, p);
            return self.odd;
        }
        else {
            //return self.value(u, v, p);
            return self.even;
        }
    }
}
pub mod solidcolor;
pub mod checker;
pub mod imagetexture;

use crate::*;

pub trait Texture{
    fn value(&self, u:f64,v:f64, p:&Point3) -> Color;
}
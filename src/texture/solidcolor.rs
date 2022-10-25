use crate::*;

pub struct SolidColor{
    color_value: Color,
}

impl SolidColor {
    pub fn new(c:Color) -> Self{
        SolidColor { color_value: c }
    }

    pub fn new_rgb(red:f64,green:f64,blue:f64) -> Self{
        SolidColor { color_value: Color::new_color(red,green,blue) }
    }
}

impl Texture for SolidColor {
    fn value(&self, u:f64,v:f64, p:&Point3) -> Color {
        self.color_value
    }
}

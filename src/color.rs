use crate::vec3::*;

pub fn output_color_as_u8(pixel_color: &Color) -> (u8, u8, u8) {
    (
        (255.999f64 * pixel_color.0) as u8,
        (255.999f64 * pixel_color.1) as u8,
        (255.999f64 * pixel_color.2) as u8,
    )
}

pub fn output_color_as_u16(pixel_color: &Color) -> (u16, u16, u16) {
    (
        (255.999f64 * pixel_color.0) as u16,
        (255.999f64 * pixel_color.1) as u16,
        (255.999f64 * pixel_color.2) as u16,
    )
}

pub fn output_color_as_u32(pixel_color: &Color) -> (u32, u32, u32) {
    (
        (255.999f64 * pixel_color.0) as u32,
        (255.999f64 * pixel_color.1) as u32,
        (255.999f64 * pixel_color.2) as u32,
    )
}

pub fn output_color_as_u64(pixel_color: &Color) -> (u64, u64, u64) {
    (
        (255.999f64 * pixel_color.0) as u64,
        (255.999f64 * pixel_color.1) as u64,
        (255.999f64 * pixel_color.2) as u64,
    )
}

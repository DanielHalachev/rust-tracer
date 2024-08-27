use core::fmt;

pub struct Color(f32, f32, f32);
pub struct Albedo(f32, f32, f32);
pub struct PPMColor(u8, u8, u8);

impl PPMColor {
    pub fn from_color(color: Color) -> Self {
        PPMColor(
            (color.0.clamp(0.0, 1.0) * 255.0) as u8,
            (color.1.clamp(0.0, 1.0) * 255.0) as u8,
            (color.2.clamp(0.0, 1.0) * 255.0) as u8,
        )
    }
}

impl fmt::Display for PPMColor {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{} {} {}", self.0, self.1, self.2)
    }
}

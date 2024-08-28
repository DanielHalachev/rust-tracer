use core::fmt;

pub use crate::tracer::vector::Triple;
pub use Triple as Color;
pub use Triple as Albedo;

#[derive(Clone)]
pub struct PPMColor(u8, u8, u8);

impl From<Color> for PPMColor {
    fn from(value: Color) -> Self {
        PPMColor(
            (value.0.clamp(0.0, 1.0) * 255.0) as u8,
            (value.1.clamp(0.0, 1.0) * 255.0) as u8,
            (value.2.clamp(0.0, 1.0) * 255.0) as u8,
        )
    }
}

impl fmt::Display for PPMColor {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{} {} {}", self.0, self.1, self.2)
    }
}

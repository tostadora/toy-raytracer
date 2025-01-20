use crate::space::Vec3;
use crate::interval::Interval;

pub type Color = Vec3;

use image::Rgb;

impl Color { 
    pub fn write_color(self: &Self) -> Rgb<u8> {
        
        let intensity = Interval::new(0.000, 0.999);

        let r = intensity.clamp(Self::linear_to_gamma(self.x())) * 256.0;
        let g = intensity.clamp(Self::linear_to_gamma(self.y())) * 256.0;
        let b = intensity.clamp(Self::linear_to_gamma(self.z())) * 256.0;

        Rgb ([r.floor() as u8, g.floor() as u8, b.floor() as u8])

    }

    fn linear_to_gamma(value: f64) -> f64 {
        if value > 0.0 {
            value.sqrt()
        } else {
            0.0
        }
    }
}

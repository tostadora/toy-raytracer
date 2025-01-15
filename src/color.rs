use crate::space::Vec3;
use crate::interval::Interval;

pub type Color = Vec3;

impl Color { 
    pub fn write_color(self: &Self) {
        
        let intensity = Interval::new(0.000, 0.999);

        let r = intensity.clamp(Self::linear_to_gamma(self.x())) * 256.0;
        let g = intensity.clamp(Self::linear_to_gamma(self.y())) * 256.0;
        let b = intensity.clamp(Self::linear_to_gamma(self.z())) * 256.0;

        println!("{} {} {}", r.floor(), g.floor(), b.floor());
    }

    fn linear_to_gamma(value: f64) -> f64 {
        if value > 0.0 {
            value.sqrt()
        } else {
            0.0
        }
    }
}

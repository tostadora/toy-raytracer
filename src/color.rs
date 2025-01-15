use crate::space::Vec3;
use crate::interval::Interval;

pub type Color = Vec3;

impl Color { 
    pub fn write_color(self: &Self) {
        
        let intensity = Interval::new(0.000, 0.999);

        let r = intensity.clamp(self.x()) * 256.0;
        let g = intensity.clamp(self.y()) * 256.0;
        let b = intensity.clamp(self.z()) * 256.0;

        println!("{} {} {}", r.floor(), g.floor(), b.floor());
    }
}

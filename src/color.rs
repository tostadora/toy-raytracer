
use crate::vec3::Vec3;
use crate::interval::Interval;

pub type Color = Vec3;

impl Color {

    pub fn write_color(&self) {

        let interval = Interval::new(0.0, 0.999);

        let r = 256.0 * interval.clamp(self.x);
        let g = 256.0 * interval.clamp(self.y);
        let b = 256.0 * interval.clamp(self.z);

        println!("{} {} {}", r.floor(), g.floor(), b.floor());
    }

}

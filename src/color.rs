
use crate::vec3::Vec3;

pub type Color = Vec3;

impl Color {

    pub fn write_color(&self) {
        let r = 255.0 * self.x;
        let g = 255.0 * self.y;
        let b = 255.0 * self.z;

        println!("{} {} {}", r.floor(), g.floor(), b.floor());
    }

}

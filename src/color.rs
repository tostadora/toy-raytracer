
use crate::vec3::Vec3;

pub type Color = Vec3;

impl Color {

    pub fn write_color(&self) {
        let ri = (255.999 * self.x) as u8;
        let gi = (255.999 * self.y) as u8;
        let bi = (255.999 * self.z) as u8;

        println!("{} {} {}", ri, gi, bi);
    }

}

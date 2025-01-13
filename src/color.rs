use crate::space::Vec3;

pub type Color = Vec3;

impl Color { 
    pub fn write_color(self: &Self) {
        
        let r = self.x() * 255.0;
        let g = self.y() * 255.0;
        let b = self.z() * 255.0;

        println!("{} {} {}", r.floor(), g.floor(), b.floor());
    }
}

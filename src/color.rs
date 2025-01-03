use crate::space::Vec3;

pub type Color = Vec3;

impl Color { 
    pub fn write_color(self: &Self) {
        
        let r = self.x();
        let g = self.y();
        let b = self.z();

        println!("{} {} {}", r.floor(), g.floor(), b.floor());
    }
}

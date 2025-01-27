
use crate::vec3::Vec3;
use crate::interval::Interval;

pub type Color = Vec3;

impl Color {

    pub fn write_color(&self) {

        let interval = Interval::new(0.0, 0.999);

        let r = 256.0 * interval.clamp(Color::linear_to_gamma(self.x));
        let g = 256.0 * interval.clamp(Color::linear_to_gamma(self.y));
        let b = 256.0 * interval.clamp(Color::linear_to_gamma(self.z));

        println!("{} {} {}", r.floor(), g.floor(), b.floor());
    }

    fn linear_to_gamma(linear_component: f64) -> f64 {

        if linear_component > 0.0 {
            return linear_component.sqrt();
        } else {
            return 0.0;
        }
    }
}

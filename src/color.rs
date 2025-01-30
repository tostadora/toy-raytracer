
use crate::vec3::Vec3;
use crate::interval::Interval;

use image::Rgb;

pub type Color = Vec3;

impl Color {

    pub fn write_color(&self) -> Rgb<u8> {

        let interval = Interval::new(0.0, 0.999);

        let r = 256.0 * interval.clamp(Color::linear_to_gamma(self.x));
        let g = 256.0 * interval.clamp(Color::linear_to_gamma(self.y));
        let b = 256.0 * interval.clamp(Color::linear_to_gamma(self.z));

        let ri: u8 = r.floor() as u8;
        let gi: u8 = g.floor() as u8;
        let bi: u8 = b.floor() as u8;

        Rgb { 0: [ri, gi, bi] }
    }

    fn linear_to_gamma(linear_component: f64) -> f64 {

        if linear_component > 0.0 {
            return linear_component.sqrt();
        } else {
            return 0.0;
        }
    }
}

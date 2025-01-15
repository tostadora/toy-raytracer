use crate::space::{Vec3, Point3};
use crate::color::Color;

pub struct Ray {
    origin: Point3,
    direction: Vec3,
}

impl Ray {

    pub fn new(origin: Point3, direction: Vec3) -> Ray {
        Ray {origin: origin,
            direction: direction}
    }

    pub fn origin(self: &Self) -> &Point3 {
        &self.origin
    }

    pub fn direction(self: &Self) -> &Vec3 {
        &self.direction
    }

    pub fn at(self: &Self, t: f64) -> Point3 {
        self.direction() * t + self.origin().clone()
    }

    pub fn hit_sphere(&self, center: &Point3, radius: f64) -> f64 {
        1.0
    }

    pub fn color(self: &Self) -> Color {

        let t = self.hit_sphere(&Point3::new(0.0, 0.0, -2.0), 0.5);

        if t > 0.0 {
            let n: Vec3 = (self.at(t) - Vec3::new(0.0, 0.0, -1.0)).unit_vector();
            return Color::new(0.5 * (n.x()+1.0), 0.5 * (n.y()+1.0), 0.5 * (n.z()+1.0));
        }

        let unit_direction = self.direction().unit_vector();
        let a = 0.5 * (unit_direction.y() + 1.0);
        return Color::new((1.0 - a) * 1.0 + a * 0.5,
                          (1.0 - a) * 1.0 + a * 0.7,
                          (1.0 - a) * 1.0 + a * 1.0);
    }
}

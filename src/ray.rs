use crate::space::{Vec3, Point3};

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

}

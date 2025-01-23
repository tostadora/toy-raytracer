use std::ops::{Add, Sub, Mul, Div};

#[derive(Debug, Clone)]
pub struct Vec3 {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

type Point3 = Vec3;

impl Vec3 {

    fn new(x: f64, y: f64, z: f64) -> Vec3 {
        Vec3 { x, y, z, }
    }

    fn length(&self) -> f64 {
        self.length_squared().sqrt()
    }

    fn length_squared(&self) -> f64 {
        self.x * self.x + self.y * self.y + self.z * self.z
    }

    fn dot(u: &Vec3, v: &Vec3) -> f64 {
        u.x * v.x + u.y * v.y + u.z * v.z
    }

    fn cross(u: &Vec3, v: &Vec3) -> Vec3 {
        Vec3::new(u.y * v.z - u.z * v.y,
                    u.z * v.x - u.x * v.z,
                    u.x * v.y - u.y * v.x,
                 )
    }

    fn unit_vector(&self) -> Vec3 {
        self / self.length()
    }

}

impl Add<&Vec3> for &Vec3 {

    type Output = Vec3;

    fn add(self, v: &Vec3) -> Vec3 {
        Vec3::new(self.x + v.x, self.y + v.y, self.z + v.z)
    }
}

impl Sub<&Vec3> for &Vec3 {

    type Output = Vec3;

    fn sub(self, v: &Vec3) -> Vec3 {
        Vec3::new(self.x - v.x,
                    self.y - v.y,
                    self.z - v.z,
                    )
    }

}

impl Mul<&Vec3> for f64 {

    type Output = Vec3;

    fn mul(self, v: &Vec3) -> Vec3 {
        v * self
    }

}

impl Mul<f64> for &Vec3 {

    type Output = Vec3;

    fn mul(self, t: f64) -> Vec3 {
        Vec3::new(self.x * t, self.y * t, self.z * t)
    }

}

impl Div<f64> for &Vec3 {

    type Output = Vec3;

    fn div(self, t: f64) -> Vec3 {
        Vec3::new(self.x / t,
                    self.y / t,
                    self.z / t,
        )
    }

}

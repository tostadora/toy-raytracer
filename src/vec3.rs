use std::ops::{Add, Sub, Mul, Div, Neg};
use rand::prelude::*;

use crate::interval::Interval;

#[derive(Debug, Clone, PartialEq)]
pub struct Vec3 {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

pub type Point3 = Vec3;

impl Vec3 {

    pub fn new(x: f64, y: f64, z: f64) -> Vec3 {
        Vec3 { x, y, z, }
    }

    pub fn length(&self) -> f64 {
        self.length_squared().sqrt()
    }

    pub fn length_squared(&self) -> f64 {
        self.x * self.x + self.y * self.y + self.z * self.z
    }

    pub fn dot(u: &Vec3, v: &Vec3) -> f64 {
        u.x * v.x + u.y * v.y + u.z * v.z
    }

    pub fn cross(u: &Vec3, v: &Vec3) -> Vec3 {
        Vec3::new(u.y * v.z - u.z * v.y,
                    u.z * v.x - u.x * v.z,
                    u.x * v.y - u.y * v.x,
                 )
    }

    pub fn unit_vector(&self) -> Vec3 {
        self / self.length()
    }

    pub fn random() -> Vec3 {
        Vec3::new(random::<f64>(), random::<f64>(), random::<f64>())
    }

    pub fn random_interval(interval: Interval) -> Vec3 {
        Vec3::new(thread_rng().gen_range(interval.min..interval.max), 
                    thread_rng().gen_range(interval.min..interval.max),
                    thread_rng().gen_range(interval.min..interval.max))
    }

    pub fn random_unit_vector() -> Vec3 {
        loop {
            let p = Self::random_interval(Interval::new(-1.0, 1.0));

            if p.length_squared() <= 1.0 {
                return p.unit_vector();
            }
        }
    }

    pub fn random_on_hemisphere(normal: &Vec3) -> Vec3 {
        let on_unit_sphere = Vec3::random_unit_vector();

        if Vec3::dot(&on_unit_sphere, normal) > 0.0 {
            on_unit_sphere
        } else {
            -on_unit_sphere
        }
    }

    pub fn near_zero(&self) -> bool {
        let s = 1e-8_f64;

        (self.x.abs() < s) && (self.y.abs() < s) && (self.z.abs() < s)
    }

    pub fn reflect(v: &Vec3, n: &Vec3) -> Vec3 {
        v - (2.0 * n * Vec3::dot(&v, &n))
    }

}

impl Neg for Vec3 {
    
    type Output = Vec3;

    fn neg(self) -> Vec3 {
        Vec3::new(-self.x, -self.y, -self.z)
    }

}

impl Neg for &Vec3 {
    
    type Output = Vec3;

    fn neg(self) -> Vec3 {
        Vec3::new(-self.x, -self.y, -self.z)
    }

}

impl Add<Vec3> for &Vec3 {

    type Output = Vec3;

    fn add(self, v: Vec3) -> Vec3 {
        Vec3::new(self.x + v.x, self.y + v.y, self.z + v.z)
    }
}

impl Add<&Vec3> for &Vec3 {

    type Output = Vec3;

    fn add(self, v: &Vec3) -> Vec3 {
        Vec3::new(self.x + v.x, self.y + v.y, self.z + v.z)
    }
}

impl Sub<Vec3> for &Vec3 {

    type Output = Vec3;

    fn sub(self, v: Vec3) -> Vec3 {
        Vec3::new(self.x - v.x,
                    self.y - v.y,
                    self.z - v.z,
                    )
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

impl Mul<Vec3> for Vec3 {

    type Output = Vec3;

    fn mul(self, v: Vec3) -> Vec3 {
        Vec3::new(self.x * v.x, self.y * v.y, self.z * v.z)
    }

}

impl Mul<&Vec3> for Vec3 {

    type Output = Vec3;

    fn mul(self, v: &Vec3) -> Vec3 {
        Vec3::new(self.x * v.x, self.y * v.y, self.z * v.z)
    }

}

impl Mul<&Vec3> for &Vec3 {

    type Output = Vec3;

    fn mul(self, v: &Vec3) -> Vec3 {
        Vec3::new(self.x * v.x, self.y * v.y, self.z * v.z)
    }

}

impl Mul<Vec3> for &Vec3 {

    type Output = Vec3;

    fn mul(self, v: Vec3) -> Vec3 {
        Vec3::new(self.x * v.x, self.y * v.y, self.z * v.z)
    }

}

impl Mul<Vec3> for f64 {

    type Output = Vec3;

    fn mul(self, v: Vec3) -> Vec3 {
        v * self
    }

}

impl Mul<&Vec3> for f64 {

    type Output = Vec3;

    fn mul(self, v: &Vec3) -> Vec3 {
        v * self
    }

}

impl Mul<f64> for Vec3 {

    type Output = Vec3;

    fn mul(self, t: f64) -> Vec3 {
        Vec3::new(self.x * t, self.y * t, self.z * t)
    }

}

impl Mul<f64> for &Vec3 {

    type Output = Vec3;

    fn mul(self, t: f64) -> Vec3 {
        Vec3::new(self.x * t, self.y * t, self.z * t)
    }

}

impl Div<f64> for Vec3 {

    type Output = Vec3;

    fn div(self, t: f64) -> Vec3 {
        Vec3::new(self.x / t,
                    self.y / t,
                    self.z / t,
        )
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

impl Add<Vec3> for Vec3 {

    type Output = Vec3;

    fn add(self, v: Vec3) -> Vec3 {
        Vec3::new(self.x + v.x, self.y + v.y, self.z + v.z)
    }
}

impl Add<&Vec3> for Vec3 {

    type Output = Vec3;

    fn add(self, v: &Vec3) -> Vec3 {
        Vec3::new(self.x + v.x, self.y + v.y, self.z + v.z)
    }
}

impl Sub<Vec3> for Vec3 {

    type Output = Vec3;

    fn sub(self, v: Vec3) -> Vec3 {
        Vec3::new(self.x - v.x,
                    self.y - v.y,
                    self.z - v.z,
                    )
    }

}

impl Sub<&Vec3> for Vec3 {

    type Output = Vec3;

    fn sub(self, v: &Vec3) -> Vec3 {
        Vec3::new(self.x - v.x,
                    self.y - v.y,
                    self.z - v.z,
                    )
    }

}

#[cfg(test)] 
mod tests {
    use super::*;

    #[test]
    fn test_dot_product() {
        let data: [(Vec3, Vec3, f64); 20] = [
                (Vec3::new(1.0, 2.0, 3.0), Vec3::new(4.0, 5.0, 6.0), 32.0),
                (Vec3::new(-1.0, -2.0, -3.0), Vec3::new(4.0, 5.0, 6.0), -32.0),
                (Vec3::new(0.0, 0.0, 0.0), Vec3::new(1.0, 2.0, 3.0), 0.0),
                (Vec3::new(1.0, 0.0, 0.0), Vec3::new(0.0, 1.0, 0.0), 0.0),
                (Vec3::new(1.0, 1.0, 1.0), Vec3::new(1.0, 1.0, 1.0), 3.0),
                (Vec3::new(2.0, 3.0, 4.0), Vec3::new(5.0, 6.0, 7.0), 56.0),
                (Vec3::new(-2.0, -3.0, -4.0), Vec3::new(5.0, 6.0, 7.0), -56.0),
                (Vec3::new(1.0, 2.0, 3.0), Vec3::new(-1.0, -2.0, -3.0), -14.0),
                (Vec3::new(3.0, 2.0, 1.0), Vec3::new(1.0, 2.0, 3.0), 10.0),
                (Vec3::new(1.0, 2.0, 3.0), Vec3::new(3.0, 2.0, 1.0), 10.0),
                (Vec3::new(2.0, 2.0, 2.0), Vec3::new(2.0, 2.0, 2.0), 12.0),
                (Vec3::new(1.0, 0.0, -1.0), Vec3::new(-1.0, 0.0, 1.0), -2.0),
                (Vec3::new(0.0, 1.0, 0.0), Vec3::new(0.0, 0.0, 1.0), 0.0),
                (Vec3::new(1.0, 2.0, 3.0), Vec3::new(1.0, 0.0, 0.0), 1.0),
                (Vec3::new(1.0, 2.0, 3.0), Vec3::new(0.0, 1.0, 0.0), 2.0),
                (Vec3::new(1.0, 2.0, 3.0), Vec3::new(0.0, 0.0, 1.0), 3.0),
                (Vec3::new(1.0, 1.0, 1.0), Vec3::new(2.0, 2.0, 2.0), 6.0),
                (Vec3::new(1.0, 2.0, 3.0), Vec3::new(3.0, 2.0, 1.0), 10.0),
                (Vec3::new(2.0, 3.0, 4.0), Vec3::new(4.0, 3.0, 2.0), 25.0),
                (Vec3::new(1.0, 2.0, 3.0), Vec3::new(4.0, 5.0, 6.0), 32.0),
        ];

        for (u, v, e) in data {
            let r = Vec3::dot(&u, &v);

            assert_eq!(e, r);
        }

    }

    #[test]
    fn test_cross_product() {
        
        let data: [(Vec3, Vec3, Vec3); 20] = [
            (Vec3::new(1.0, 2.0, 3.0), Vec3::new(4.0, 5.0, 6.0), Vec3::new(-3.0, 6.0, -3.0)),
            (Vec3::new(-1.0, -2.0, -3.0), Vec3::new(4.0, 5.0, 6.0), Vec3::new(3.0, -6.0, 3.0)),
            (Vec3::new(0.0, 0.0, 0.0), Vec3::new(1.0, 2.0, 3.0), Vec3::new(0.0, 0.0, 0.0)),
            (Vec3::new(1.0, 0.0, 0.0), Vec3::new(0.0, 1.0, 0.0), Vec3::new(0.0, 0.0, 1.0)),
            (Vec3::new(1.0, 1.0, 1.0), Vec3::new(1.0, 1.0, 1.0), Vec3::new(0.0, 0.0, 0.0)),
            (Vec3::new(2.0, 3.0, 4.0), Vec3::new(5.0, 6.0, 7.0), Vec3::new(-3.0, 6.0, -3.0)),
            (Vec3::new(-2.0, -3.0, -4.0), Vec3::new(5.0, 6.0, 7.0), Vec3::new(3.0, -6.0, 3.0)),
            (Vec3::new(1.0, 2.0, 3.0), Vec3::new(-1.0, -2.0, -3.0), Vec3::new(0.0, 0.0, 0.0)),
            (Vec3::new(3.0, 2.0, 1.0), Vec3::new(1.0, 2.0, 3.0), Vec3::new(4.0, -8.0, 4.0)),
            (Vec3::new(1.0, 2.0, 3.0), Vec3::new(3.0, 2.0, 1.0), Vec3::new(-4.0, 8.0, -4.0)),
            (Vec3::new(2.0, 2.0, 2.0), Vec3::new(2.0, 2.0, 2.0), Vec3::new(0.0, 0.0, 0.0)),
            (Vec3::new(1.0, 0.0, -1.0), Vec3::new(-1.0, 0.0, 1.0), Vec3::new(0.0, 0.0, 0.0)),
            (Vec3::new(0.0, 1.0, 0.0), Vec3::new(0.0, 0.0, 1.0), Vec3::new(1.0, 0.0, 0.0)),
            (Vec3::new(1.0, 2.0, 3.0), Vec3::new(1.0, 0.0, 0.0), Vec3::new(0.0, 3.0, -2.0)),
            (Vec3::new(1.0, 2.0, 3.0), Vec3::new(0.0, 1.0, 0.0), Vec3::new(-3.0, 0.0, 1.0)),
            (Vec3::new(1.0, 2.0, 3.0), Vec3::new(0.0, 0.0, 1.0), Vec3::new(2.0, -1.0, 0.0)),
            (Vec3::new(1.0, 1.0, 1.0), Vec3::new(2.0, 2.0, 2.0), Vec3::new(0.0, 0.0, 0.0)),
            (Vec3::new(1.0, 2.0, 3.0), Vec3::new(3.0, 2.0, 1.0), Vec3::new(-4.0, 8.0, -4.0)),
            (Vec3::new(2.0, 3.0, 4.0), Vec3::new(4.0, 3.0, 2.0), Vec3::new(-6.0, 12.0, -6.0)),
            (Vec3::new(1.0, 2.0, 3.0), Vec3::new(4.0, 5.0, 6.0), Vec3::new(-3.0, 6.0, -3.0)),
        ];

        for (u, v, e) in data {
            println!("[Cross product] Trying: {:?} {:?}", u, v);
            let r = Vec3::cross(&u, &v);

            assert_eq!(e, r);
        }
    }

}

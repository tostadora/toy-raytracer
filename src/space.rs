use std::ops::{Add, Div, Mul, Neg, Sub};
use rand;

#[derive(Debug, Clone)]
pub struct Vec3 {
    x: f64,
    y: f64,
    z: f64,
}

impl Vec3 {

    pub fn new(x: f64, y: f64, z: f64) -> Vec3 {
        Vec3 {x: x, y: y, z: z}
    }

    pub fn origin() -> Vec3 {
        Vec3 {x: 0.0, y: 0.0, z: 0.0}
    }

    pub fn x(self: &Self) -> f64 {
        self.x
    }

    pub fn y(self: &Self) -> f64 {
        self.y
    }

    pub fn z(self: &Self) -> f64 {
        self.z
    }

    pub fn length (self: &Self) -> f64 {
        self.length_squared().sqrt()
    }

    pub fn length_squared (&self) -> f64 {
        (self.x * self.x) + (self.y * self.y) + (self.z * self.z)
    }

    pub fn unit_vector(self: &Self) -> Vec3 {
        self / self.length()
    }

    pub fn inverted(&self) -> Vec3 {
        Vec3::new(-self.x, -self.y, -self.z)
    }

    pub fn random_unit_vector() -> Vec3 {
        loop {
            let p = Vec3::random();
            let length = p.length_squared();
            if f64::MIN < length && length < 1.0 { return p.unit_vector(); }
        }
    }

    pub fn random_on_hemisphere(&self) -> Vec3 {
        let on_unit_sphere = Vec3::random_unit_vector();

        if Vec3::dot(&on_unit_sphere, self) > 0.0 { on_unit_sphere }
        else { -on_unit_sphere }
    }

    pub fn dot (a: &Vec3, b: &Vec3) -> f64 {
        a.x() * b.x() + a.y() * b.y() + a.z() * b.z()
    }
    
    pub fn cross (a: &Vec3, b: &Vec3) -> Vec3 {
        Vec3 {x: a.y() * b.z() - a.z() * b.y(),
              y: a.z() * b.x() - a.x() * b.z(),
              z: a.x() * b.y() - a.y() * b.x(),}
    }

    pub fn random () -> Vec3 {
        Vec3::new(rand::random::<f64>(), rand::random::<f64>(), rand::random::<f64>())
    }

    pub fn random_bounded (min: f64, max: f64) -> Vec3 {
        Vec3::new(rand::random::<f64>(), rand::random::<f64>(), rand::random::<f64>()) * (max-min) + Vec3::new(min, min, min)
    }

    pub fn near_zero(&self) -> bool {
        let s = 1e-8;

        (self.x.abs() < s) && (self.y.abs() < s) && (self.z() < s)
    }

    pub fn reflect(uv: &Vec3, n: &Vec3) -> Vec3 {
        uv + &(-2.0 * Vec3::dot(uv, n) * n)
    }

    pub fn refract(uv: &Vec3, n: &Vec3, etai_over_etat: f64) -> Vec3 {
        let cos_theta = Vec3::dot(&uv.inverted(), &n).min(1.0);
        let r_out_perp = etai_over_etat * (uv + &(cos_theta * n));
        let r_out_parallel = n * -1.0 * (1.0 - r_out_perp.length_squared()).abs().sqrt();

        r_out_perp + r_out_parallel
    }

}

impl Add for Vec3 {
    type Output = Vec3;

    fn add(self, other: Vec3) -> Vec3 {
        Vec3 {
            x: self.x + other.x(),
            y: self.y + other.y(),
            z: self.z + other.z(),
        }
    }
}

impl Add for &Vec3 {
    type Output = Vec3;

    fn add(self, other: &Vec3) -> Vec3 {
        Vec3 {
            x: self.x() + other.x(),
            y: self.y() + other.y(),
            z: self.z() + other.z(),
        }
    }
}

impl Sub for Vec3 {
    type Output = Vec3;

    fn sub(self, other: Vec3) -> Vec3 {
        Vec3 {
            x: self.x - other.x(),
            y: self.y - other.y(),
            z: self.z - other.z(),
        }
    }
}

impl Sub for &Vec3 {
    type Output = Vec3;

    fn sub(self, other: &Vec3) -> Vec3 {
        Vec3 {
            x: self.x() - other.x(),
            y: self.y() - other.y(),
            z: self.z() - other.z(),
        }
    }
}

impl Neg for Vec3 {
    type Output = Vec3;

    fn neg(self) -> Vec3 {
        Vec3 {
            x: -self.x,
            y: -self.y,
            z: -self.z,
        }
    }
}

impl Mul<Vec3> for Vec3 {
    type Output = Vec3;

    fn mul(self, other: Vec3) -> Vec3 {
        Vec3 {
            x: self.x * other.x(),
            y: self.y * other.y(),
            z: self.z * other.z(),
        }
    }
}

impl Mul<f64> for Vec3 {
    type Output = Vec3;

    fn mul(self, other: f64) -> Vec3 {
        Vec3 {
            x: self.x * other,
            y: self.y * other,
            z: self.z * other,
        }
    }
}

impl Mul<f64> for &Vec3 {
    type Output = Vec3;

    fn mul(self, other: f64) -> Vec3 {
        Vec3 {
            x: self.x * other,
            y: self.y * other,
            z: self.z * other,
        }
    }
}

impl Mul<Vec3> for f64 {
    type Output = Vec3;

    fn mul(self, other: Vec3) -> Vec3 {
        Vec3 {
            x: self * other.x,
            y: self * other.y,
            z: self * other.z,
        }
    }
}

impl Mul<&Vec3> for f64 {
    type Output = Vec3;

    fn mul(self, other: &Vec3) -> Vec3 {
        Vec3 {
            x: self * other.x(),
            y: self * other.y(),
            z: self * other.z(),
        }
    }
}

impl Div<Vec3> for Vec3 {
    type Output = Vec3;

    fn div(self, other: Vec3) -> Vec3 {
        Vec3 {
            x: self.x / other.x(),
            y: self.y / other.y(),
            z: self.z / other.z(),
        }
    }
}

impl Div<f64> for Vec3 {
    type Output = Vec3;

    fn div(self, other: f64) -> Vec3 {
        Vec3 {
            x: self.x / other,
            y: self.y / other,
            z: self.z / other,
        }
    }
}

impl Div<f64> for &Vec3 {
    type Output = Vec3;

    fn div(self, other: f64) -> Vec3 {
        Vec3 {
            x: self.x() / other,
            y: self.y() / other,
            z: self.z() / other,
        }
    }
}

impl PartialEq for Vec3 {
    fn eq(&self, other: &Vec3) -> bool {
        self.x == other.x() && self.y == other.y() && self.z == other.z()
    }
}

pub type Point3 = Vec3;


#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_dot_01() {
        let u = Vec3::new(1.0, 2.0, -3.0);
        let v = Vec3::new(-1.0, 9.0, -4.0);
        let expected = 29.0; 

        assert_eq!(Vec3::dot(&u, &v), expected);
    }
    
    #[test]
    fn test_dot_02() {
        let u = Vec3::new(1.0, 2.0, 3.0);
        let v = Vec3::new(4.0, 5.0, 6.0);
        let expected = 32.0; 

        assert_eq!(Vec3::dot(&u, &v), expected);
    }
    
    #[test]
    fn test_dot_03() {
        let u = Vec3::new(7.0, 8.0, 9.0);
        let v = Vec3::new(10.0, 11.0, 12.0);
        let expected = 266.0; 

        assert_eq!(Vec3::dot(&u, &v), expected);
    }
    
    #[test]
    fn test_dot_04() {
        let u = Vec3::new(2.0, -3.0, 4.0);
        let v = Vec3::new(-1.0, 5.0, -6.0);
        let expected = -41.0; 

        assert_eq!(Vec3::dot(&u, &v), expected);
    }
    
    #[test]
    fn test_dot_05() {
        let u = Vec3::new(0.0, 0.0, 0.0);
        let v = Vec3::new(1.0, 2.0, 3.0);
        let expected = 0.0; 

        assert_eq!(Vec3::dot(&u, &v), expected);
    }
    
    #[test]
    fn test_dot_06() {
        let u = Vec3::new(1.0, -1.0, 1.0);
        let v = Vec3::new(-1.0, 1.0, -1.0);
        let expected = -3.0; 

        assert_eq!(Vec3::dot(&u, &v), expected);
    }
    
    #[test]
    fn test_dot_07() {
        let u = Vec3::new(-2.0, -4.0, -6.0);
        let v = Vec3::new(3.0, 6.0, 9.0);
        let expected = -84.0; 

        assert_eq!(Vec3::dot(&u, &v), expected);
    }
    
    #[test]
    fn test_dot_08() {
        let u = Vec3::new(1.0, 0.0, -1.0);
        let v = Vec3::new(0.0, 1.0, 0.0);
        let expected = 0.0; 

        assert_eq!(Vec3::dot(&u, &v), expected);
    }
    
    #[test]
    fn test_dot_09() {
        let u = Vec3::new(5.0, 5.0, 5.0);
        let v = Vec3::new(1.0, 2.0, 3.0);
        let expected = 30.0; 

        assert_eq!(Vec3::dot(&u, &v), expected);
    }
    
    #[test]
    fn test_dot_10() {
        let u = Vec3::new(-3.0, -2.0, -1.0);
        let v = Vec3::new(1.0, 0.0, -1.0);
        let expected = -2.0; 

        assert_eq!(Vec3::dot(&u, &v), expected);
    }

    #[test]
    fn test_reflect() {

        let data: [(Vec3, Vec3, Vec3); 10] = [
            (Vec3::new(1.0, 2.0, 3.0), Vec3::new(1.0, 0.0, 0.0), Vec3::new(-1.0, 2.0, 3.0)),
            (Vec3::new(-4.0, 5.0, -6.0), Vec3::new(0.0, 1.0, 0.0), Vec3::new(-4.0, -5.0, -6.0)),
            (Vec3::new(7.0, -8.0, 9.0), Vec3::new(0.0, 0.0, 1.0), Vec3::new(7.0, -8.0, -9.0)),
            (Vec3::new(-1.0, 2.0, -3.0), Vec3::new(1.0, 1.0, 1.0), Vec3::new(3.0, 6.0, 1.0)),
            (Vec3::new(4.0, -5.0, 6.0), Vec3::new(1.0, -1.0, 0.0), Vec3::new(-14.0, 13.0, 6.0)),
            (Vec3::new(0.0, 3.0, -7.0), Vec3::new(1.0, 1.0, 1.0), Vec3::new(8.0, 11.0, 1.0)),
            (Vec3::new(2.0, -2.0, 2.0), Vec3::new(1.0, 0.0, 1.0), Vec3::new(-6.0, -2.0, -6.0)),
            (Vec3::new(3.0, 0.0, -3.0), Vec3::new(0.0, 1.0, 1.0), Vec3::new(3.0, 6.0, 3.0)),
            (Vec3::new(-6.0, 7.0, 8.0), Vec3::new(1.0, 1.0, 0.0), Vec3::new(-8.0, 5.0, 8.0)),
            (Vec3::new(1.0, 1.0, 1.0), Vec3::new(1.0, 1.0, 1.0), Vec3::new(-5.0, -5.0, -5.0)),
        ];

        for (uv, n, expected) in data {
            let actual = Vec3::reflect(&uv, &n);
            assert_eq!(actual, expected);
        }
    }

    #[test]
    fn test_refract_01() {
        let uv = Point3::new(1.0, 1.0, 0.0);
        let n = Point3::new(-1.0, 0.0, 0.0);
        let etai_over_etat = 1.0;
        let expected = Point3::new(0.0, 1.0, 0.0);
        let actual = Vec3::refract(&uv, &n, etai_over_etat);
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_refract_02() {
        let uv = Point3::new(-2.0, -2.0, -2.0);
        let n = Point3::new(1.0, 0.0, 0.0);
        let etai_over_etat = 0.9;
        let expected = Point3::new(-3.4079872407968907, -1.8, -1.8);
        let actual = Vec3::refract(&uv, &n, etai_over_etat);
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_refract_03() {
        let uv = Point3::new(-1.34, -2.32, 5.32);
        let n = Point3::new(1.1, -0.45, 3.21);
        let etai_over_etat = 1.5;
        let expected = Point3::new(-115.6544941015172, 43.010929405166124, -323.65529642351834);
        let actual = Vec3::refract(&uv, &n, etai_over_etat);
        assert_eq!(actual, expected);
    }
}

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

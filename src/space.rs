use std::ops;

#[derive(Debug)]
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

    pub fn length_squared (self: &Self) -> f64 {
        (self.x * self.x) + (self.y * self.y) + (self.z * self.z)
    }

    pub fn unit_vector(self: &Self) -> Vec3 {
        self / self.length()
    }
}

pub fn dot (a: &Vec3, b: &Vec3) -> f64 {
    a.x() * b.x() + a.y() * b.y() + a.z() * b.z()
}

pub fn cross (a: &Vec3, b: &Vec3) -> Vec3 {
    Vec3 {x: a.y() * b.z() - a.z() * b.y(),
          y: a.z() * b.x() - a.x() * b.z(),
          z: a.x() * b.y() - a.y() * b.x(),}
}

impl_op!(+ |a: &Vec3, b: &Vec3| -> Vec3 { Vec3 {x: a.x() + b.x(),
                                                y: a.y() + b.y(),
                                                z: a.z() + b.z(),}
                                                });
impl_op!(- |a: &Vec3, b: &Vec3| -> Vec3 { Vec3 {x: a.x() - b.x(),
                                                y: a.y() - b.y(),
                                                z: a.z() - b.z(),}
                                                });
impl_op!(* |a: &Vec3, b: &Vec3| -> Vec3 { Vec3 {x: a.x() * b.x(),
                                                y: a.y() * b.y(),
                                                z: a.z() * b.z(),}
                                                });
impl_op!(* |a: &Vec3, b: f64| -> Vec3 { Vec3 {x: a.x() * b,
                                                y: a.y() * b,
                                                z: a.z() * b,}
                                                });
impl_op!(/ |a: &Vec3, b: f64| -> Vec3 { Vec3 {x: a.x() / b,
                                                y: a.y() / b,
                                                z: a.z() /b,}
                                                });
impl_op!(+= |a: &mut Vec3, b: Vec3|    {       a.x += b.x();
                                                a.y += b.y();
                                                a.z += b.z();
                                                });
impl_op!(+= |a: &mut Vec3, b: &Vec3|    {       a.x += b.x();
                                                a.y += b.y();
                                                a.z += b.z();
                                                });
impl_op!(*= |a: &mut Vec3, b: f64|    {       a.x *= b;
                                                a.y *= b;
                                                a.z *= b;
                                                });
impl_op!(/= |a: &mut Vec3, b: f64|    {       *a *= 1.0/b;
                                                });
pub type Point3 = Vec3;

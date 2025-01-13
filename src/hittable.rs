use crate::Ray;
use crate::space::{Vec3, Point3};

#[derive(PartialEq)]
pub enum Face {
    Front,
    Back,
}

pub struct HitRecord {
    pub p: Point3,
    pub normal: Vec3,
    pub t: f64,
    pub face: Face,
}

impl HitRecord {

    pub fn new(p: Point3, normal: Vec3, t: f64, face: Face) -> HitRecord {
        HitRecord { 
            p: p,
            normal: normal,
            t: t,
            face: face,
        }
    }

    pub fn set_face_normal(&mut self, ray: &Ray) {
        
        self.face = if Vec3::dot(ray.direction(), &self.normal) < 0.0 { Face::Front } else { Face::Back };

        self.normal = if self.face == Face::Front { self.normal.clone() } else {-(self.normal.clone()) };
    }
}

pub trait Hittable {

    fn hit(&self, ray: &Ray, ray_tmin: f64, ray_tmax: f64) -> Option<HitRecord>;

}

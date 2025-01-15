use crate::ray::Ray;
use crate::space::Vec3;
use crate::interval::Interval;

#[derive(PartialEq)]
pub enum Face {
    Front,
    Back,
}

pub struct HitRecord {
    pub normal: Vec3,
    pub t: f64,
    pub face: Face,
}

impl HitRecord {

    pub fn new(normal: Vec3, t: f64, face: Face) -> HitRecord {
        HitRecord { 
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

    fn hit(&self, ray: &Ray, ray_t: Interval) -> Option<HitRecord>;

}

use crate::ray::Ray;
use crate::space::{Vec3, Point3};
use crate::interval::Interval;
use crate::material::Material;

use std::rc::Rc;

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
    pub material: Rc<dyn Material>,
}

impl HitRecord {

    pub fn new(p: Point3, normal: Vec3, t: f64, face: Face, material: Rc<dyn Material>) -> HitRecord 
    {
        HitRecord { 
            p: p,
            normal: normal,
            t: t,
            face: face,
            material: material,
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

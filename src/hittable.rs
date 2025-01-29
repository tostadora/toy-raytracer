use crate::vec3::{Vec3, Point3};
use crate::ray::Ray;
use crate::interval::Interval;
use crate::material::Material;

use std::sync::Arc;

pub enum Face {
    Front,
    Back
}

pub struct HitRecord {
    pub p: Point3,
    pub normal: Vec3,
    pub material: Arc<dyn Material>,
    pub t: f64,
    pub face: Face,
}

pub trait Hittable {
    fn hit(&self, r: &Ray, ray_t: Interval) -> Option<HitRecord>;
}

impl HitRecord {
    pub fn new(p: Point3, normal: Vec3, material: Arc<dyn Material>, t: f64, face: Face) -> HitRecord {
        HitRecord {
            p,
            normal,
            material,
            t,
            face,
        }
    }

    pub fn set_face_normal(&mut self, r: &Ray, outward_normal: &Vec3) {
        (self.face, self.normal) = if Vec3::dot(&r.direction, outward_normal) < 0.0 {
                                        (Face::Front, outward_normal.clone())
                                    } else {
                                        (Face::Back, -outward_normal)
                                    };
    }

}

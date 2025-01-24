
use crate::vec3::{Vec3, Point3};
use crate::ray::Ray;

pub enum Face {
    Front,
    Back
}

pub struct HitRecord {
    pub p: Point3,
    pub normal: Vec3,
    pub t: f64,
    pub face: Face,
}

pub trait Hittable {
    fn hit(&self, r: &Ray, ray_tmin: f64, ray_tmax: f64) -> Option<HitRecord>;
}

impl HitRecord {
    pub fn new(p: Point3, normal: Vec3, t: f64, face: Face) -> HitRecord {
        HitRecord {
            p,
            normal,
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


use crate::vec3::{Vec3, Point3};
use crate::ray::Ray;

pub struct HitRecord {
    pub p: Point3,
    pub normal: Vec3,
    pub t: f64
}

pub trait Hittable {
    fn hit(r: &Ray, ray_tmin: f64, ray_tmax: f64) -> Option<HitRecord>;
}

impl HitRecord {
    pub fn new(p: Point3, normal: Vec3, t: f64) -> HitRecord {
        HitRecord {
            p,
            normal,
            t,
        }
    }

}

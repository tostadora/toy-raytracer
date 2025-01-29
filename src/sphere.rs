use crate::hittable::{Hittable, HitRecord, Face};
use crate::vec3::{Vec3, Point3};
use crate::ray::Ray;
use crate::interval::Interval;
use crate::material::Material;

use std::sync::Arc;

pub struct Sphere {
    pub center: Point3,
    pub radius: f64,
    pub material: Arc<dyn Material>,
}

impl Sphere {
    pub fn new(center: Point3, radius: f64, material: Arc<dyn Material>) -> Sphere {
        Sphere {
            center,
            radius,
            material,
        }
    }
}

impl Hittable for Sphere {

    fn hit(&self, r: &Ray, ray_t: Interval) -> Option<HitRecord> {
        
        let oc = &self.center - &r.origin;
        let a = r.direction.length_squared();
        let h = Vec3::dot(&r.direction, &oc);
        let c = oc.length_squared() - self.radius*self.radius;
        let discriminant = h*h - a*c;

        if discriminant < 0.0 {
            return None;
        }

        let sqrtd = discriminant.sqrt();

        let mut root = (h - sqrtd) / a;

        if !ray_t.surrounds(root) {
            root = (h + sqrtd) / a;
            if !ray_t.surrounds(root) {
                return None;
            }
        }

        let p = r.at(root);
        let mut hr = HitRecord::new(p.clone(), (&p - &self.center) / self.radius, self.material.clone(), root, Face::Front);
        let outward_normal = (p - &self.center) / self.radius; 
        hr.set_face_normal(&r, &outward_normal);
        Some(hr)
    }
}

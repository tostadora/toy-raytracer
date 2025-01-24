use crate::hittable::{Hittable, HitRecord, Face};
use crate::vec3::{Vec3, Point3};
use crate::ray::Ray;

pub struct Sphere {
    pub center: Point3,
    pub radius: f64,
}

impl Sphere {
    pub fn new(center: Point3, radius: f64) -> Sphere {
        Sphere {
            center,
            radius,
        }
    }
}

impl Hittable for Sphere {

    fn hit(&self, r: &Ray, ray_tmin: f64, ray_tmax: f64) -> Option<HitRecord> {
        
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

        if root <= ray_tmin || ray_tmax <= root {
            root = (h + sqrtd) / a;
            if root <= ray_tmin || ray_tmax <= root {
                return None;
            }
        }

        let p = r.at(root);
        let mut hr = HitRecord::new(p.clone(), (&p - &self.center) / self.radius, root, Face::Front);
        let outward_normal = (p - &self.center) / self.radius; 
        hr.set_face_normal(&r, &outward_normal);

        Some(hr)
    }
}

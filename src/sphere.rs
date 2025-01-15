use crate::space::{Point3, Vec3};
use crate::hittable::*;
use crate::ray::Ray;
use crate::interval::Interval;

pub struct Sphere {
    pub center: Point3,
    pub radius: f64,
}

impl Sphere {

    pub fn new(center: Point3, radius:f64) -> Sphere {
        Sphere {
            center: center,
            radius: radius,
        }
    }

}

impl Hittable for Sphere {


    fn hit(&self, ray: &Ray, ray_t: Interval) -> Option<HitRecord> {
        let oc = &self.center - ray.origin();

        let a = ray.direction().length_squared();
        let h = Vec3::dot(ray.direction(), &oc);
        let c = oc.length_squared() - self.radius*self.radius;

        let discriminant = h*h - a*c;

        if discriminant < 0.0 { return None; }

        let sqrtd = discriminant.sqrt();
        let root = (h - sqrtd) / a;

        if ray_t.surrounds(root) == false {
            let root = (h + sqrtd) / a;
            if ray_t.surrounds(root) == false { return None; }
        }

        let outward_normal = (ray.at(root) - self.center.clone()) / self.radius;

        let mut hr = HitRecord::new(outward_normal, root, Face::Front);

        hr.set_face_normal(ray);

        Some(hr)
    }
}

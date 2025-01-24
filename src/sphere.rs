use crate::hittable::{Hittable, HitRecord};
use crate::vec3::Point3;

pub struct Sphere {
    pub center: Point3,
    pub radius: f64,
}

impl Sphere {
    pub fn new(center: &Point3, radius: f64) -> Sphere {
        Sphere {
            center,
            radius.
        }
    }
}

impl Hittable for Sphere {

    pub fn hit(&self, r: &Ray, ray_tmin: f64, ray_tmax: f64) -> Option<HitRecord> {
        
        let oc = self.center - &r.origin;
        let a = r.direction.length_squared();
        let h = Vec3::dot(&r.direction, &oc);
        let c = oc.length_squared() - radius*radius;
        let discriminant = h*h - a*c;

        if discriminant < 0.0 {
            None
        }

        let sqrtd = discriminant.sqrt();

        let root = (h - sqrtd) / a;

        if root <= ray_tmin || ray_tmax <= root {
            root = (h + sqrtd) / a;
            if root <= ray_tmin || ray_tmax <= root {
                None
            }
        }

        let p = r.at(root);

        HitRecord::new(p, &p - self.center) self.radius, root)
        
    }
}

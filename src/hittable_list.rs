use crate::hittable::{Hittable, HitRecord, Face};
use crate::ray::Ray;
use crate::vec3::{Vec3, Point3};


pub type HittableList = Vec<Box<dyn Hittable>>;

impl Hittable for HittableList {

    fn hit(self: &Self, r: &Ray, ray_tmin: f64, ray_tmax: f64) -> Option<HitRecord> {
        let mut hr = HitRecord::new(Point3::new(0.0, 0.0, 0.0), Vec3::new(0.0, 0.0, 0.0), 0.0, Face::Front);
        let mut hit_anything = false;
        let mut closest_so_far = ray_tmax;

        for object in self {
            match object.hit(r, ray_tmin, closest_so_far) {
                Some(hit) => {
                    hit_anything = true;
                    closest_so_far = hit.t;
                    hr = hit;
                },
                None => (),
            }
        }
        if hit_anything {
            Some(hr)
        } else {
            None
        }
    }

}


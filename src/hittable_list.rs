use crate::hittable::{Hittable, HitRecord, Face};
use crate::ray::Ray;
use crate::vec3::{Vec3, Point3};
use crate::interval::Interval;

pub type HittableList = Vec<Box<dyn Hittable>>;

impl Hittable for HittableList {

    fn hit(self: &Self, r: &Ray, ray_t: Interval) -> Option<HitRecord> {
        let mut hr = HitRecord::new(Point3::new(0.0, 0.0, 0.0), Vec3::new(0.0, 0.0, 0.0), 0.0, Face::Front);
        let mut hit_anything = false;
        let mut closest_so_far = ray_t.max;

        for object in self {
            match object.hit(r, Interval::new(ray_t.min, closest_so_far)) {
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


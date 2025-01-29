use crate::hittable::{Hittable, HitRecord};
use crate::ray::Ray;
use crate::interval::Interval;

pub type HittableList = Vec<Box<dyn Hittable>>;

impl Hittable for HittableList {

    fn hit(self: &Self, r: &Ray, ray_t: Interval) -> Option<HitRecord> {
        let mut closest_so_far = ray_t.max;
        let mut result: Option<HitRecord> = None;

        for object in self {
            match object.hit(r, Interval::new(ray_t.min, closest_so_far)) {
                Some(hit) => {
                    closest_so_far = hit.t;
                    result = Some(hit);
                },
                None => (),
            }
        }
        result
    }

}


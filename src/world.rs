use crate::sphere::Sphere;
use crate::hittable::{HitRecord, Hittable};
use crate::ray::Ray;
use crate::interval::Interval;

pub type World = Vec<Sphere>;

impl Hittable for &Vec<Sphere> {

    fn hit(&self, ray: &Ray, ray_t: Interval) -> Option<HitRecord> {
        let mut closest_so_far = ray_t.right;
        let mut result: Option<HitRecord> = None;

        for object in *self {
            let hit = object.hit(ray, Interval::new(ray_t.left, closest_so_far));
            match hit {
                Some(h) => { closest_so_far = h.t; result = Some(h)}
                None => ()
            }
        }

        result
    }

}


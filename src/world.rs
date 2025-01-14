use crate::sphere::Sphere;
use crate::hittable::{HitRecord, Hittable};
use crate::ray::Ray;

pub type World = Vec<Sphere>;

impl Hittable for &Vec<Sphere> {

    fn hit(&self, ray: &Ray, ray_tmin: f64, ray_tmax: f64) -> Option<HitRecord> {
        let mut closest_so_far = ray_tmax;
        let mut result: Option<HitRecord> = None;

        for object in *self {
            let hit = object.hit(ray, ray_tmin, closest_so_far);
            match hit {
                Some(h) => { closest_so_far = h.t; result = Some(h)}
                None => ()
            }
        }

        result
    }

}


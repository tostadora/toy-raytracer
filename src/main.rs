mod vec3;
mod color;
mod ray;
mod hittable;
mod sphere;
mod hittable_list;
mod interval;
mod camera;

use vec3::{Vec3, Point3};
use ray::Ray;
use hittable::{HitRecord, Hittable};
use hittable_list::HittableList;
use sphere::Sphere;
use interval::Interval;
use camera::Camera;

fn hit_sphere(center: Point3, radius: f64, r: &Ray) -> f64 {
    let oc = &center - &r.origin;

    let a = r.direction.length_squared();
    let h = Vec3::dot(&r.direction, &oc);
    let c = oc.length_squared() - radius*radius;
    let discriminant = h*h - a*c;

    if discriminant < 0.0 {
        -1.0
    } else {
        (h-discriminant.sqrt()) / a
    }
}

fn main() {

    // Image

    let aspect_ratio = 16.0 / 9.0;

    let image_width: u32 = 400;

    // World

    let mut world: HittableList = vec![];

    world.push(Box::new(Sphere::new(Point3::new(0.0, 0.0, -1.0), 0.5)));
    world.push(Box::new(Sphere::new(Point3::new(0.0, -100.5, -1.0), 100.0)));

    // Camera
    
    let camera = Camera::new(16.0 / 9.0, 400, 100);

    camera.render(&world);

}

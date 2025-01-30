mod vec3;
mod color;
mod ray;
mod hittable;
mod sphere;
mod hittable_list;
mod interval;
mod camera;
mod material;

use vec3::{Vec3, Point3};
use color::Color;
use hittable_list::HittableList;
use sphere::Sphere;
use camera::Camera;
use material::{Lambertian, Metal, Dielectric};
use interval::Interval;

use std::sync::Arc;

fn main() {

    // World

    let mut world: HittableList = vec![];

    world.push(Box::new(Sphere::new(Point3::new( 0.0, -100.5, -1.0), 100.0, Arc::new(Lambertian::new(Color::new(0.8, 0.8, 0.0))))));

    for a in -11..11 {
        for b in -11..11 {
            let choose_mat = Vec3::random_double();
            let center = Point3::new(a as f64 + 0.9 * Vec3::random_double(), 
                                        0.2,
                                        b as f64 + 0.9 * Vec3::random_double());
            if (&center - Point3::new(4.0, 0.2, 0.0)).length() > 0.9 {
                world.push(Box::new(Sphere::new(center, 0.2,
                    match choose_mat {
                        _x if choose_mat < 0.8 => Arc::new(Lambertian::new(Vec3::random() * Vec3::random())),
                        _x if choose_mat >= 0.8 && choose_mat < 0.95 => Arc::new(Metal::new(Vec3::random_interval(Interval::new(0.5, 1.0)), Vec3::random_double_interval(&Interval::new(0.5, 1.0)))),
                        _ => Arc::new(Dielectric::new(1.5)),
                    })));
            }
        }
    }

    world.push(Box::new(Sphere::new(Point3::new(0.0, 1.0, 0.0), 1.0, Arc::new(Dielectric::new(1.5)))));
    world.push(Box::new(Sphere::new(Point3::new(-4.0, 1.0, 0.0), 1.0, Arc::new(Lambertian::new(Color::new(0.4, 0.2, 0.1))))));
    world.push(Box::new(Sphere::new(Point3::new(4.0, 1.0, 0.0), 1.0, Arc::new(Metal::new(Color::new(0.7, 0.6, 0.5), 0.0)))));

    // Camera
    
    let camera = Camera::new(16.0 / 9.0, 400, 500, 50, 20.0, Point3::new(13.0, 2.0, 3.0), Point3::new(0.0, 0.0, 0.0), Vec3::new(0.0, 1.0, 0.0), 0.6, 10.0);

    camera.render(&world);

}

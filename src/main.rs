mod vec3;
mod color;
mod ray;
mod hittable;
mod sphere;
mod hittable_list;
mod interval;
mod camera;
mod material;

use vec3::Point3;
use color::Color;
use hittable_list::HittableList;
use sphere::Sphere;
use camera::Camera;
use material::{Lambertian, Metal, Dielectric};

use std::sync::Arc;

fn main() {

    let r = std::f64::consts::FRAC_PI_4.cos();

    // World

    let mut world: HittableList = vec![];

    // world.push(Box::new(Sphere::new(Point3::new( 0.0, -100.5, -1.0), 100.0, Arc::new(Lambertian::new(Color::new(0.8, 0.8, 0.0))))));
    // world.push(Box::new(Sphere::new(Point3::new( 0.0,    0.0, -1.2),   0.5, Arc::new(Lambertian::new(Color::new(0.1, 0.2, 0.5))))));
    // world.push(Box::new(Sphere::new(Point3::new(-1.0,    0.0, -1.0),   0.5, Arc::new(Dielectric::new(1.5)))));
    // world.push(Box::new(Sphere::new(Point3::new(-1.0,    0.0, -1.0),   0.4, Arc::new(Dielectric::new(1.00 / 1.50)))));
    // world.push(Box::new(Sphere::new(Point3::new( 1.0,    0.0, -1.0),   0.5, Arc::new(Metal::new(Color::new(0.8, 0.6, 0.2), 1.0)))));

    world.push(Box::new(Sphere::new(Point3::new(-r, 0.0, -1.0), r, Arc::new(Lambertian::new(Color::new(0.0, 0.0, 1.0))))));
    world.push(Box::new(Sphere::new(Point3::new( r, 0.0, -1.0), r, Arc::new(Lambertian::new(Color::new(1.0, 0.0, 0.0))))));

    // Camera
    
    let camera = Camera::new(16.0 / 9.0, 400, 100, 50, 90.0);

    camera.render(&world);

}

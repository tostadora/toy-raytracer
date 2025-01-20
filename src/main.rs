mod space;
mod color;
mod ray;
mod hittable;
mod sphere;
mod world;
mod interval;
mod camera;
mod material;

use sphere::Sphere;
use world::World;
use space::Point3;
use camera::Camera;
use material::{Lambertian, Metal, Dielectric};
use color::Color;

fn main() {

    // World

    let mut world: World = vec!();

    // Ground
    world.push(Sphere::new(Point3::new(0.0, -100.5, -1.0), 100.0, Lambertian::new(Color::new(0.8, 0.8, 0.0))));
    // Center
    world.push(Sphere::new(Point3::new(0.0, 0.0, -1.2), 0.5, Lambertian::new(Color::new(0.1, 0.2, 0.5))));
    // Left
    world.push(Sphere::new(Point3::new(-1.0, 0.0, -1.0), 0.5, Dielectric::new(1.5)));
    // Right
    world.push(Sphere::new(Point3::new(1.0, 0.0, -1.0), 0.5, Metal::new(Color::new(0.8, 0.6, 0.2), 1.0)));

    // Camera
    
    let mut camera = Camera::new();

    camera.aspect_ratio = 16.0 / 9.0;
    camera.image_width = 400;
    camera.samples_per_pixel = 100;
    camera.max_depth = 50;
    
    camera.render(&world);
}

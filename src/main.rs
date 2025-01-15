mod space;
mod color;
mod ray;
mod hittable;
mod sphere;
mod world;
mod interval;
mod camera;

use sphere::Sphere;
use world::World;
use space::Point3;
use camera::Camera;

fn main() {

    // World

    let mut world: World = vec!();
    world.push(Sphere::new(Point3::new(0.0, 0.0, -1.0), 0.5));
    world.push(Sphere::new(Point3::new(0.0, -100.5, -1.0), 100.0));

    // Camera
    
    let mut camera = Camera::new();

    camera.aspect_ratio = 16.0 / 9.0;
    camera.image_width = 400;
    camera.samples_per_pixel = 100;
    
    camera.render(&world);
}

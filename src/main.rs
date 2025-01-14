mod space;
mod color;
mod ray;
mod hittable;
mod sphere;
mod world;

use std::io::Write;
use space::{Vec3, Point3};
use color::Color;
use ray::Ray;
use sphere::Sphere;
use world::World;
use hittable::Hittable;

use std::f64::consts::PI;

fn ray_color (r: &Ray, w: &World) -> Color {

    match w.hit(r, 0.0, f64::INFINITY) {
        Some(hit) => Color::new(0.5 * (hit.normal.x() + 1.0), 0.5 * (hit.normal.y() + 1.0), 0.5 * (hit.normal.z() + 1.0)),
        None => {
                    let unit_direction = r.direction().unit_vector();
                    let a = 0.5 * (unit_direction.y() + 1.0);
                    Color::new(1.0-a+a*0.5, 1.0-a+a*0.7, 1.0-a+a*1.0)
        }
    }
}

fn main() {
    let aspect_ratio: f64 = 16.0 / 9.0;

    let image_width: usize = 400;
    let image_height: usize = if ((image_width as f64 * aspect_ratio) as usize) < 1 { 1 } else { (image_width as f64 / aspect_ratio) as usize };

    // World

    let mut world: World = vec!();
    world.push(Sphere::new(Point3::new(0.0, 0.0, -1.0), 0.5));
    world.push(Sphere::new(Point3::new(0.0, -100.5, -1.0), 100.0));

    // Camera
    
    let focal_length: f64 = 2.0;
    let viewport_height: f64 = 2.0;
    let viewport_width = viewport_height * (image_width as f64 / image_height as f64);
    let camera_center = Point3::new(0.0, 0.0, 0.0);

    // calculate the vectors across the viewport edges

    let viewport_u: Vec3 = Vec3::new(viewport_width, 0.0, 0.0);
    let viewport_v: Vec3 = Vec3::new(0.0, -viewport_height, 0.0);

    let pixel_delta_u = &viewport_u / image_width as f64;
    let pixel_delta_v = &viewport_v / image_height as f64;

    let viewport_upper_left = camera_center.clone() - Vec3::new(0.0, 0.0, focal_length) - &viewport_u / 2.0 - &viewport_v / 2.0;
    let pixel00_loc = viewport_upper_left + 0.5_f64 * (&pixel_delta_u + &pixel_delta_v);

    eprintln!("\r{} {}", image_height, image_width);
    println!("P3\n{} {}\n255\n", image_width, image_height);
    for j in 0..image_height {
        eprint!("\rScanlines remaining: {} ", image_height - j);
        for i in 0..image_width {
            let pixel_center = pixel00_loc.clone() + i as f64 * &pixel_delta_u + j as f64 * &pixel_delta_v;
            let ray_direction = pixel_center - camera_center.clone();

            let ray = Ray::new(camera_center.clone(), ray_direction);

            let color = ray_color(&ray, &world);
            color.write_color();
        }
    }
    eprintln!("\rDone");
    let _ = std::io::stderr().flush();
}

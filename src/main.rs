mod vec3;
mod color;
mod ray;

use vec3::{Vec3, Point3};
use color::Color;
use ray::Ray;

fn hit_sphere(center: Point3, radius: f64, r: &Ray) -> bool {
    let oc = &center - &r.origin;

    let a = Vec3::dot(&r.direction, &r.direction);
    let b = -2.0 * Vec3::dot(&r.direction, &oc);
    let c = Vec3::dot(&oc, &oc) - radius*radius;
    let discriminant = b*b - 4.0*a*c;

    discriminant >= 0.0
}

fn ray_color(r: &Ray) -> Color{
    
    if hit_sphere(Point3::new(0.0, 0.0, -1.0), 0.5, &r) {
        Color::new(1.0, 0.0, 0.0)
    }
    else {
        let unit_direction = r.direction.unit_vector();
        let a = 0.5 * (unit_direction.y + 1.0);

        &((1.0 - a) * &Color::new(1.0, 1.0, 1.0)) + &(a * &Color::new(0.5, 0.7, 1.0))
    }
}

fn main() {

    // Image

    let aspect_ratio = 16.0 / 9.0;

    let image_width: u32 = 400;

    let image_height: u32 = if image_width as f64 / aspect_ratio < 1.0 { 1 } else { (image_width as f64 / aspect_ratio) as u32 };

    // Camera

    let focal_length: f64 = 1.0;
    let viewport_height: f64 = 2.0;
    let viewport_width = viewport_height * (image_width as f64 / image_height as f64);
    let camera_center = Point3::new(0.0, 0.0, 0.0);

    // Calculate the vectors across the horizontal and down the vertical viewport edges

    let viewport_u = Vec3::new(viewport_width, 0.0, 0.0);
    let viewport_v = Vec3::new(0.0, -viewport_height, 0.0);

    // Calculate the horizontal and vertical delta vectors from pixel to pixel
    
    let pixel_delta_u = &viewport_u / image_width as f64;
    let pixel_delta_v = &viewport_v / image_height as f64;

    // Calculate the location of the upper left pixel.

    let viewport_upper_left = &camera_center - Vec3::new(0.0, 0.0, focal_length) - &viewport_u/2.0_f64 - &viewport_v/2.0_f64;
    let pixel00_loc = &viewport_upper_left + (0.5 * &pixel_delta_u + &pixel_delta_v);

    println!("P3");
    println!("{} {}", image_width, image_height);
    println!("255");

    for j in 0..image_height {
        eprint!("\rScanlines remaining: {}", image_height - j);
        for i in 0..image_width {
            let pixel_center = &pixel00_loc + i as f64 * &pixel_delta_u + j as f64 * &pixel_delta_v;
            let ray_direction = &pixel_center - &camera_center;
            let r = Ray::new(camera_center.clone(), ray_direction.clone());

            let color = ray_color(&r);
            color.write_color();
        }
    }

    eprintln!("Done");
}

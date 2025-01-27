use crate::vec3::{Vec3, Point3};
use crate::hittable::Hittable;
use crate::hittable_list::HittableList;
use crate::ray::Ray;
use crate::color::Color;
use crate::interval::Interval;

use rand::prelude::*;

pub struct Camera {
    pub aspect_ratio: f64,
    pub image_width: u32,
    pub samples_per_pixel: u32,
    pub max_depth: u32,
    image_height: u32,
    pixel_samples_scale: f64,
    center: Point3,
    pixel00_loc: Point3,
    pixel_delta_u: Vec3,
    pixel_delta_v: Vec3,
}

impl Camera {

    pub fn new(aspect_ratio: f64, image_width: u32, samples_per_pixel: u32, max_depth: u32) -> Camera {
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

        Camera {
            aspect_ratio: aspect_ratio,
            image_width: image_width,
            samples_per_pixel: samples_per_pixel,
            max_depth: max_depth,
            image_height: image_height,
            pixel_samples_scale: 1.0 / samples_per_pixel as f64,
            center: camera_center,
            pixel00_loc: pixel00_loc,
            pixel_delta_u: pixel_delta_u,
            pixel_delta_v: pixel_delta_v,
        }

    }

    pub fn render(&self, world: &HittableList) {

        println!("P3");
        println!("{} {}", self.image_width, self.image_height);
        println!("255");

        for j in 0..self.image_height {
            eprint!("\rScanlines remaining: {}", self.image_height - j);
            for i in 0..self.image_width {
                let mut color = Color::new(0.0, 0.0, 0.0);

                for _ in 0..self.samples_per_pixel {
                    let r = self.get_ray(i, j);
                    color = color + Self::ray_color(&r, &world, self.max_depth);
                }
                color = color * self.pixel_samples_scale;
                color.write_color();
            }
        }

        eprintln!("Done");
    }


    fn ray_color(r: &Ray, world: &HittableList, depth: u32) -> Color {
        
        if depth == 0 {
            return Color::new(0.0, 0.0, 0.0);
        }

        match world.hit(&r, Interval::new(0.001, f64::INFINITY)) {
            Some(hr) => {
                let direction = Vec3::random_on_hemisphere(&hr.normal);
                0.5 * Self::ray_color(&Ray::new(hr.p, direction), &world, depth - 1)
            },
            None => {
                let unit_direction = r.direction.unit_vector();
                let a = 0.5 * (unit_direction.y + 1.0);
                (1.0 - a) * Color::new(1.0, 1.0, 1.0) + (a * Color::new(0.5, 0.7, 1.0))
            },
        }
    }

    fn get_ray(&self, i: u32, j: u32) -> Ray {
       let offset = Self::sample_square();
       let pixel_sample = &self.pixel00_loc 
                                + ((i as f64 + offset.x) * &self.pixel_delta_u)
                                + ((j as f64 + offset.y) * &self.pixel_delta_v);
       let ray_direction = pixel_sample - &self.center;
       Ray::new(self.center.clone(), ray_direction)
    }

    fn sample_square() -> Vec3 {
        Vec3::new(random::<f64>() - 0.5, random::<f64>() - 0.5, 0.0)
    }

}

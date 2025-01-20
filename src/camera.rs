use crate::space::{Vec3, Point3};
use crate::world::World;
use crate::hittable::Hittable;
use crate::ray::Ray;
use crate::color::Color;
use crate::interval::Interval;

use std::io::Write;

use rand;
use image;

pub struct Camera {
    pub aspect_ratio: f64,
    pub image_width: usize,
    pub samples_per_pixel: usize,
    pub max_depth: usize,
    image_height: usize,
    center: Point3,
    pixel00_loc: Point3,
    pixel_delta_u: Vec3,
    pixel_delta_v: Vec3,
}

impl Camera {

    pub fn new() -> Camera {
        Camera { 
            aspect_ratio: 16.0/9.0,
            image_width: 0,
            image_height: 0,
            samples_per_pixel: 0,
            max_depth: 0,
            center: Point3::new(0.0, 0.0, 0.0),
            pixel00_loc: Point3::new(0.0, 0.0, 0.0),
            pixel_delta_u: Vec3::new(0.0, 0.0, 0.0),
            pixel_delta_v: Vec3::new(0.0, 0.0, 0.0),
        }
    }

    fn ray_color (r: &Ray, w: &World, invocation_count: usize) -> Color {

        if invocation_count == 0 { return Color::new(0.0, 0.0, 0.0); }
    
        match w.hit(r, Interval::new(0.001, f64::INFINITY)) {
            Some(hit) => {
                        match hit.material.scatter(r, &hit) {
                            Some((color, ray)) => {
                                color * Self::ray_color(&ray, w, invocation_count - 1)
                            },
                            None => Color::new(0.0, 0.0, 0.0),
                        }
            },
            None => {
                        let unit_direction = r.direction().unit_vector();
                        let a = 0.5 * (unit_direction.y() + 1.0);
                        Color::new(1.0-a+a*0.5, 1.0-a+a*0.7, 1.0-a+a*1.0)
            }
        }
    }

    pub fn initialize(&mut self) {
        self.image_height = if ((self.image_width as f64 * self.aspect_ratio) as usize) < 1 { 1 } else { (self.image_width as f64 / self.aspect_ratio) as usize };
        self.center = Point3::new(0.0, 0.0, 0.0);
    
        let focal_length: f64 = 1.0;
        let viewport_height: f64 = 2.0;
        let viewport_width = viewport_height * (self.image_width as f64 / self.image_height as f64);
    
        // calculate the vectors across the viewport edges
    
        let viewport_u: Vec3 = Vec3::new(viewport_width, 0.0, 0.0);
        let viewport_v: Vec3 = Vec3::new(0.0, -viewport_height, 0.0);
    
        self.pixel_delta_u = &viewport_u / self.image_width as f64;
        self.pixel_delta_v = &viewport_v / self.image_height as f64;
    
        let viewport_upper_left = self.center.clone() - Vec3::new(0.0, 0.0, focal_length) - &viewport_u / 2.0 - &viewport_v / 2.0;
        self.pixel00_loc = viewport_upper_left + 0.5_f64 * (&self.pixel_delta_u + &self.pixel_delta_v);
    }

    pub fn render(&mut self, world: &World) {
        self.initialize();

        let mut img = image::RgbImage::new(self.image_width as u32, self.image_height as u32);

        eprintln!("\r{} {}", self.image_height, self.image_width);
        for j in 0..self.image_height {
            eprint!("\rScanlines remaining: {} ", self.image_height - j);
            for i in 0..self.image_width {
                let mut color = Color::new(0.0, 0.0, 0.0);

                for _ in 0..self.samples_per_pixel {
                    let r = self.get_ray(i, j);
                    color = color + Self::ray_color(&r, world, self.max_depth);
                }

                color = color * (1.0 / self.samples_per_pixel as f64);
                img.put_pixel(i as u32, j as u32, color.write_color());
            }
        }
        eprintln!("\rDone");
        let _ = std::io::stderr().flush();

        img.save("img.png").unwrap();
    }

    fn get_ray(&self, i: usize, j: usize) -> Ray {
        
        let offset = Self::sample_square();

        let pixel_sample = &self.pixel00_loc + &((i as f64 + offset.x()) * &self.pixel_delta_u) + ((j as f64 + offset.y()) * &self.pixel_delta_v);
        
        Ray::new(self.center.clone(), &pixel_sample - &self.center)
    }

    fn sample_square() -> Vec3 {
        Vec3::new(rand::random::<f64>() - 0.5, rand::random::<f64>() - 0.5, 0.0)
    }

}    

use crate::vec3::{Vec3, Point3};
use crate::hittable::Hittable;
use crate::hittable_list::HittableList;
use crate::ray::Ray;
use crate::color::Color;
use crate::interval::Interval;

pub struct Camera {
    pub aspect_ratio: f64,
    pub image_width: u32,
    pub samples_per_pixel: u32,
    pub max_depth: u32,
    pub vfov: f64,
    pub lookfrom: Point3,
    pub lookat: Point3,
    pub vup: Vec3,
    pub defocus_angle: f64,
    pub focus_dist: f64,
    image_height: u32,
    pixel_samples_scale: f64,
    center: Point3,
    pixel00_loc: Point3,
    pixel_delta_u: Vec3,
    pixel_delta_v: Vec3,
    u: Vec3,
    v: Vec3,
    w: Vec3,
    defocus_disk_u: Vec3,
    defocus_disk_v: Vec3,
}

impl Camera {

    pub fn new(aspect_ratio: f64, image_width: u32, samples_per_pixel: u32, max_depth: u32, vfov: f64, lookfrom: Point3, lookat: Point3, vup: Vec3, defocus_angle: f64, focus_dist: f64) -> Camera {

        let image_height: u32 = if image_width as f64 / aspect_ratio < 1.0 { 1 } else { (image_width as f64 / aspect_ratio) as u32 };

        // Camera

        let camera_center = lookfrom.clone();
        let theta = vfov.to_radians();
        let h = (theta/2.0).tan();
        let viewport_height = 2.0 * h * focus_dist;
        let viewport_width = viewport_height * (image_width as f64 / image_height as f64);

        // Calculate the u,v,w unit basis vectors for the camera coordinate frame.
        let w = (&lookfrom - &lookat).unit_vector();
        let u = Vec3::cross(&vup, &w).unit_vector();
        let v = Vec3::cross(&w, &u);
        
        // Calculate the vectors across the horizontal and down the vertical viewport edges

        let viewport_u = viewport_width * &u;
        let viewport_v = viewport_height * -&v;

        // Calculate the horizontal and vertical delta vectors from pixel to pixel
        
        let pixel_delta_u = &viewport_u / image_width as f64;
        let pixel_delta_v = &viewport_v / image_height as f64;

        // Calculate the location of the upper left pixel.

        let viewport_upper_left = &camera_center - (focus_dist * &w) - &viewport_u/2.0_f64 - &viewport_v/2.0_f64;
        let pixel00_loc = &viewport_upper_left + 0.5 * (&pixel_delta_u + &pixel_delta_v);

        // Calculate the camera defocus disk basis vectors.
        let defocus_radius = focus_dist * (defocus_angle / 2.0).to_radians().tan();
        let defocus_disk_u = &u * defocus_radius;
        let defocus_disk_v = &v * defocus_radius;

        Camera {
            aspect_ratio: aspect_ratio,
            image_width: image_width,
            samples_per_pixel: samples_per_pixel,
            max_depth: max_depth,
            vfov: vfov,
            lookfrom: lookfrom,
            lookat: lookat,
            vup: vup,
            defocus_angle: defocus_angle,
            focus_dist: focus_dist,
            image_height: image_height,
            pixel_samples_scale: 1.0 / samples_per_pixel as f64,
            center: camera_center,
            pixel00_loc: pixel00_loc,
            pixel_delta_u: pixel_delta_u,
            pixel_delta_v: pixel_delta_v,
            u: u,
            v: v,
            w: w,
            defocus_disk_u: defocus_disk_u,
            defocus_disk_v: defocus_disk_v,
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
        
        if depth <= 0 {
            return Color::new(0.0, 0.0, 0.0);
        }

        match world.hit(&r, Interval::new(0.001, f64::INFINITY)) {
            Some(hr) => {
                match hr.material.scatter(&r, &hr) {
                    Some((attenuation, scattered)) => {
                        attenuation * &Self::ray_color(&scattered, &world, depth - 1)
                    },
                    None => Color::new(0.0, 0.0, 0.0),
                }
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
        let ray_origin = if self.defocus_angle <= 0.0 {
                self.center.clone()
            } else {
                self.defocus_disk_sample()
            };
        let ray_direction = pixel_sample - &ray_origin;
        Ray::new(ray_origin, ray_direction)
    }

    fn sample_square() -> Vec3 {
        Vec3::new(Vec3::random_double() - 0.5, Vec3::random_double() - 0.5, 0.0)
    }

    fn defocus_disk_sample(&self) -> Vec3 {
        let p = Vec3::random_in_unit_disk();
        &self.center + (p.x * &self.defocus_disk_u) + (p.y * &self.defocus_disk_v)
    }

}

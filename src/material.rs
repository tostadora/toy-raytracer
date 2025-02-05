use crate::color::Color;
use crate::ray::Ray;
use crate::hittable::{HitRecord, Face};
use crate::vec3::Vec3;

pub trait Material {
    fn scatter(self: &Self, r_in: &Ray, hr: &HitRecord) -> Option<(Color, Ray)>;
}

pub struct Lambertian {
    albedo: Color,
}

impl Lambertian {

    pub fn new(albedo: Color) -> Lambertian {
        Lambertian {
            albedo,
        }
    }
    
}

impl Material for Lambertian {

    fn scatter(&self, r_in: &Ray, hr: &HitRecord) -> Option<(Color, Ray)> {
        let mut scatter_direction = &hr.normal + Vec3::random_unit_vector();

        if scatter_direction.near_zero() {
            scatter_direction = hr.normal.clone();
        }

        let scattered = Ray::new(hr.p.clone(), scatter_direction);
        let attenuation = self.albedo.clone();

        Some((attenuation, scattered))
    }

}

pub struct Metal {
    albedo: Color,
    fuzz: f64,
}

impl Metal {

    pub fn new(albedo: Color, fuzz: f64) -> Metal {
        Metal {
            albedo,
            fuzz,
        }
    }
    
}
impl Material for Metal {

    fn scatter(&self, r_in: &Ray, hr: &HitRecord) -> Option<(Color, Ray)> {
        let reflected = Vec3::reflect(&r_in.direction, &hr.normal);

        let scattered = Ray::new(hr.p.clone(), reflected.unit_vector() + (self.fuzz * Vec3::random_unit_vector()));
        let attenuation = self.albedo.clone();

        if Vec3::dot(&scattered.direction, &hr.normal) > 0.0 {
            Some((attenuation, scattered))
        } else {
            None
        }
    }

}

pub struct Dielectric {
    refraction_index: f64,
}

impl Dielectric {

    pub fn new(refraction_index: f64) -> Dielectric {
        Dielectric {
            refraction_index,
        }
    }
    
    fn reflectance(cosine: f64, refraction_index: f64) -> f64 {
        let mut r0 = (1.0 - refraction_index) / (1.0 + refraction_index);
        r0 = r0 * r0;
        r0 + (1.0-r0)*(1.0-cosine).powi(5)
    }
}

impl Material for Dielectric {

    fn scatter(&self, r_in: &Ray, hr: &HitRecord) -> Option<(Color, Ray)> {
        let attenuation = Color::new(1.0, 1.0, 1.0);

        let ri = match hr.face {
                    Face::Front => 1.0 / self.refraction_index,
                    Face::Back => self.refraction_index,
        };

        let unit_direction = r_in.direction.unit_vector();
        let cos_theta = Vec3::dot(&(-&unit_direction), &hr.normal).min(1.0);
        let sin_theta = (1.0 - cos_theta * cos_theta).sqrt();

        let direction = if (ri * sin_theta > 1.0) || (Self::reflectance(cos_theta, ri) > Vec3::random_double()) {
                            Vec3::reflect(&unit_direction, &hr.normal)
                        } else {
                            Vec3::refract(&unit_direction, &hr.normal, ri)
                        };

        let scattered = Ray::new(hr.p.clone(), direction);

        Some((attenuation, scattered))

    }

}


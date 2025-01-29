use crate::color::Color;
use crate::ray::Ray;
use crate::hittable::HitRecord;
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

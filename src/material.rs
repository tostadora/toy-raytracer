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
}

impl Metal {

    pub fn new(albedo: Color) -> Metal {
        Metal {
            albedo,
        }
    }
    
}
impl Material for Metal {

    fn scatter(&self, r_in: &Ray, hr: &HitRecord) -> Option<(Color, Ray)> {
        let reflected = Vec3::reflect(&r_in.direction, &hr.normal);

        let scattered = Ray::new(hr.p.clone(), reflected);
        let attenuation = self.albedo.clone();

        Some((attenuation, scattered))
    }

}

use crate::ray::Ray;
use crate::hittable::{HitRecord, Face};
use crate::color::Color;
use crate::space::Vec3;

pub trait Material {
    fn scatter(&self, ray: &Ray, rec: &HitRecord) -> Option<(Color, Ray)>;
}

pub struct Lambertian {
    albedo: Color,
}

impl Lambertian {

    pub fn new(albedo: Color) -> Lambertian {
        Lambertian { albedo }
    }

}
impl Material for Lambertian {

    fn scatter(&self, _ray: &Ray, rec: &HitRecord) -> Option<(Color, Ray)> {
        let scatter_direction = &rec.normal + &Vec3::random_unit_vector();

        let scattered = Ray::new(rec.p.clone(), if scatter_direction.near_zero() { rec.normal.clone() } else { scatter_direction } );
        Some((self.albedo.clone(), scattered))
    }

}

pub struct Metal {
    albedo: Color,
    fuzz: f64, 
}

impl Metal {

    pub fn new(albedo: Color, fuzz: f64) -> Metal {
        Metal { albedo, fuzz: if fuzz > 1.0 { 1.0 } else { fuzz } }
    }

}

impl Material for Metal {

    fn scatter(&self, ray: &Ray, rec: &HitRecord) -> Option<(Color, Ray)> {
        let reflected = ray.direction().reflect(&rec.normal).unit_vector() + (self.fuzz * Vec3::random_unit_vector());

        let scattered = Ray::new(rec.p.clone(), reflected);
        Some((self.albedo.clone(), scattered))
    }

}


pub struct Dielectric {
    refractive_index: f64, 
}

impl Dielectric {
    pub fn new (refractive_index: f64) -> Dielectric {
        Dielectric { refractive_index }
    }
}

impl Material for Dielectric {

    fn scatter(&self, ray: &Ray, rec: &HitRecord) -> Option<(Color, Ray)> {
        
        let ri: f64 = match rec.face {
            Face::Front => 1.0 / self.refractive_index,
            Face::Back => self.refractive_index,
        };

        let unit_direction = ray.direction().unit_vector();

        let cos_theta = Vec3::dot(&unit_direction.inverted(), &rec.normal).min(1.0);
        let sin_theta = (1.0 - cos_theta*cos_theta).sqrt();

        let direction = if ri * sin_theta > 1.0 { unit_direction.reflect(&rec.normal) } else { unit_direction.refract(&rec.normal, ri) };

        Some((Color::new(1.0, 1.0, 1.0), Ray::new(rec.p.clone(), direction)))
                
    }

}

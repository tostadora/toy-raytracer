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
        let reflected = Vec3::reflect(ray.direction(), &rec.normal).unit_vector() + (self.fuzz * Vec3::random_unit_vector());

        let scattered = Ray::new(rec.p.clone(), reflected);
        Some((self.albedo.clone(), scattered))
    }

}


pub struct Dielectric {
    refraction_index: f64, 
}

impl Dielectric {
    pub fn new (refraction_index: f64) -> Dielectric {
        Dielectric { refraction_index }
    }
}

impl Material for Dielectric {

    fn scatter(&self, ray: &Ray, rec: &HitRecord) -> Option<(Color, Ray)> {
        
        let ri: f64 = match rec.face {
            Face::Front => 1.0 / self.refraction_index,
            Face::Back => self.refraction_index,
        };

        let unit_direction = ray.direction().unit_vector();

        let direction = Vec3::refract(&unit_direction, &rec.normal, ri);

        Some((Color::new(1.0, 1.0, 1.0), Ray::new(rec.p.clone(), direction)))
        // Some((Color::new(1.0, 1.0, 1.0), Ray::new(rec.p.clone(), Vec3::new(direction.x()*-1.0, direction.y(), direction.z()*-1.0))))
                
    }

}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::space::Point3;
    use std::rc::Rc;
    
    #[test]
    fn glass_scatter_01() {
        let glass = Dielectric::new(1.5);

        let ray = Ray::new(Point3::new(12.9916, 2.02697, 3.01851), Vec3::new(-9.86105, -0.0266471, -2.22016));
        let hr = HitRecord::new(Point3::new(0.0646456, 1.99204, 0.108083), Vec3::new(0.0646456, 0.992038, 0.108083), 0.0, Face::Front, Rc::new(glass));
        
        match hr.material.scatter(&ray, &hr) {
            Some((color, ray)) => {
                let expected = (Color::new(1.0, 1.0, 1.0), Ray::new(Point3::new(0.0646456, 1.99204, 0.108083), Vec3::new(-0.6948681618142796, -0.6844002884391897, -0.22080457960185934)));
                assert_eq!(color, expected.0);
                assert_eq!(ray.direction(), expected.1.direction());
            },
            None => panic!(),
        }
    }

}

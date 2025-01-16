use crate::ray::Ray;
use crate::hittable::HitRecord;
use crate::color::Color;
use crate::space::Vec3;

pub trait Material {
    fn scatter(&self, ray: &Ray, rec: &HitRecord) -> Option<(Color, Ray)>;
}

pub struct Lambertian {
    pub albedo: Color,
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

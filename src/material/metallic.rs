use crate::material::Material;
use crate::ray::*;
use crate::color::Color;
use crate::hittable::HitRecord;
use crate::vector::Vec3;

use rand::rngs::ThreadRng;

pub struct Metallic {
    pub albedo: Color,
    pub fuzz: f64 // Radius of a sphere where the reflected direction is moved in (0.0 to 1.0)
}

impl Material for Metallic {
    fn scatter(&self, rng: &mut ThreadRng, ray_in: &Ray, hit_record: &HitRecord) -> (Color, Ray, bool) {
        let reflected = Vec3::reflect(&ray_in.dir.unit_vector(), &hit_record.normal);
        let scattered = Ray{ orig: hit_record.p, dir: reflected + Vec3::random_vector_in_unit_sphere(rng)*self.fuzz};
        let b = Vec3::dot(&scattered.dir, &hit_record.normal) > 0.0;


        (self.albedo, scattered, b)
    }
}

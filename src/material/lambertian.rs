use crate::material::Material;
use crate::ray::*;
use crate::color::Color;
use crate::hittable::HitRecord;
use crate::vector::Vec3;

use rand::rngs::ThreadRng;

pub struct Lambertian {
    pub albedo: Color
}

impl Material for Lambertian {
    fn scatter(&self, rng: &mut ThreadRng, ray_in: &Ray, hit_record: &HitRecord) -> (Color, Ray, bool) {
        let mut scatter_direction = hit_record.normal + Vec3::random_unit_vector(rng);

        // If the random unit vector generated is exactly opposite the normal vector, 
        // the two will sum to zero, which will result in a zero scatter direction vector
        if scatter_direction.near_zero() {
            scatter_direction = hit_record.normal;
        }

        let scattered = Ray{
            orig: hit_record.p,
            dir: scatter_direction
        };
        
        (self.albedo, scattered, true)
    }
}

use crate::ray::*;
use crate::color::Color;
use crate::hittable::HitRecord;

use rand::rngs::ThreadRng;

pub mod lambertian;
pub mod metallic;

pub trait Material {
    // Produce a scattered ray, alongside how much it should be attenuated
    fn scatter(&self, rng: &mut ThreadRng, ray_in: &Ray, hit_record: &HitRecord) -> (Color, Ray, bool);
}

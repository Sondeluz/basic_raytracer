use crate::utils::*;

use std::ops;
use rand::rngs::ThreadRng;

#[derive(Debug, Copy, Clone)]
pub struct Vec3 {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl Vec3 {
    pub fn random_vector(rng : &mut ThreadRng) -> Vec3 {

        return Vec3 {
            x: random_f64(rng),
            y: random_f64(rng),
            z: random_f64(rng),
        }
    }

    pub fn random_unit_vector(rng : &mut ThreadRng) -> Vec3 {
        // By picking random points in the unit sphere and then normalizing those we get
        // a random point on the unit sphere. This creates a Lambertian distribution where
        // the probability of ray scattering close to the normal is higher, but the distribution
        // is more uniform
        Vec3::unit_vector(&Vec3::random_vector_in_unit_sphere(rng))
    }

    pub fn random_vector_bounded(rng : &mut ThreadRng, min: f64, max: f64) -> Vec3 {
        return Vec3 {
            x: random_f64_bounded(rng, min, max),
            y: random_f64_bounded(rng, min, max),
            z: random_f64_bounded(rng, min, max),
        }
    }

    // Get a random vector located inside a sphere of radius 1
    pub fn random_vector_in_unit_sphere(rng : &mut ThreadRng) -> Vec3 {
        loop {
            let v = Vec3::random_vector_bounded(rng, -1.0, 1.0);
            
            if v.length_squared() < 1.0 {
                return v
            }
        }
    }

    // Alternative diffuse method where the scatter direction is any angle away from the hit point
    pub fn random_vector_in_hemisphere(rng : &mut ThreadRng, normal : &Vec3) -> Vec3 {
        let in_unit_sphere = Vec3::random_vector_in_unit_sphere(rng);
        
        if Vec3::dot(&in_unit_sphere, normal) > 0.0 { // In the same hemisphere as the normal
            return in_unit_sphere;
        } else{
            return -in_unit_sphere;
        }
    }

    pub fn dot(&self, rhs: &Vec3) -> f64 {
        self.x * rhs.x + self.y * rhs.y + self.z * rhs.z
    }

    pub fn cross(&self, rhs: Vec3) -> Vec3 {
        Vec3 {
            x: self.y * rhs.z - self.z * rhs.y,
            y: self.z * rhs.x - self.x * rhs.z,
            z: self.x * rhs.y - self.y * rhs.x,
        }
    }

    pub fn unit_vector(&self) -> Vec3 {
        *self / self.length()
    }

    // Distance of the vector from its origin
    pub fn length(&self) -> f64 {
        f64::sqrt(self.length_squared())
    }

    pub fn length_squared(&self) -> f64 {
        (self.x * self.x) + (self.y * self.y) + (self.z * self.z)
    }

    pub fn near_zero(&self) -> bool {
        let s = 1e-8;

        (f64::abs(self.x) < s) && (f64::abs(self.y) < s) && (f64::abs(self.z) < s) 
    }

    pub fn reflect(&self, normal : &Vec3) -> Vec3 {
        *self - (*normal*self.dot(normal))*2.0
    }
}

impl ops::Index<usize> for Vec3 {
    type Output = f64;

    fn index(&self, idx: usize) -> &Self::Output {
        match idx {
            0 => &self.x,
            1 => &self.y,
            2 => &self.z,
            _ => panic!("Wrong Vec3 index: {}", idx),
        }
    }
}

impl ops::Neg for Vec3 {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Vec3 {
            x: -self.x,
            y: -self.y,
            z: -self.z,
        }
    }
}

impl ops::Add for Vec3 {
    type Output = Self;

    fn add(self, rhs: Self) -> Self {
        Vec3 {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}

impl ops::Sub for Vec3 {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self {
        Vec3 {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
        }
    }
}

impl ops::Mul<f64> for Vec3 {
    type Output = Self;

    fn mul(self, rhs: f64) -> Self {
        Vec3 {
            x: self.x * rhs,
            y: self.y * rhs,
            z: self.z * rhs,
        }
    }
}

impl ops::Mul<Vec3> for Vec3 {
    type Output = Self;

    fn mul(self, rhs: Vec3) -> Self {
        Vec3 {
            x: self.x * rhs.x,
            y: self.y * rhs.y,
            z: self.z * rhs.z,
        }
    }
}

impl ops::Div<f64> for Vec3 {
    type Output = Self;

    fn div(self, rhs: f64) -> Self {
        self * (1_f64 / rhs)
    }
}

impl ops::AddAssign<f64> for Vec3 {
    fn add_assign(&mut self, rhs: f64) {
        self.x += rhs;
        self.y += rhs;
        self.z += rhs;
    }
}

impl ops::AddAssign<Vec3> for Vec3 {
    fn add_assign(&mut self, rhs: Vec3) {
        *self = *self + rhs
    }
}

impl ops::MulAssign<f64> for Vec3 {
    fn mul_assign(&mut self, rhs: f64) {
        self.x *= rhs;
        self.y *= rhs;
        self.z *= rhs;
    }
}

impl ops::DivAssign<f64> for Vec3 {
    fn div_assign(&mut self, rhs: f64) {
        *self *= 1_f64 / rhs;
    }
}

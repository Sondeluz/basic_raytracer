use crate::point::Point;
use crate::vector::Vec3;
use crate::ray::Ray;

pub struct HitRecord {
    pub p : Point,
    pub normal : Vec3,
    pub t : f64, // Closest hit point (result of solving the equation P(t)=A+tb)
    pub front_face : bool
}

impl HitRecord {
    pub fn new() -> HitRecord {
        HitRecord {
            p:Vec3 { x: 0.0, y: 0.0, z: 0.0 },
            normal:Vec3 { x: 0.0, y: 0.0, z: 0.0 },
            t: 0.0,
            front_face:false
        }
    }

    pub fn set_face_normal(&mut self, ray : &Ray, outward_normal : &Vec3) {
        // If the ray and the normal face in the same direction 
        // (their dot product is positive) the ray is inside the object, else it's outside
        self.front_face = Vec3::dot(&ray.dir, outward_normal) < 0.0;

        if self.front_face { // Outside
            self.normal = *outward_normal; // If it's outside the object, it has the same normal
        } else { // Inside
            self.normal = -(*outward_normal); // If it's inside, it has its negative
        }
    }

}

pub trait Hittable {
    fn hit(&self, ray : &Ray, t_min : f64, t_max : f64) -> (bool, HitRecord);
}

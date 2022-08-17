use crate::hittable::Hittable;
use crate::hittable::HitRecord;
use crate::ray::Ray;
use crate::point::Point;
use crate::vector::Vec3;


pub struct Sphere {
    pub center: Point,
    pub radius: f64
}



impl Hittable for Sphere {
    fn hit(&self, ray : &Ray, t_min : f64, t_max : f64) -> (bool, HitRecord) {
        let mut hr = HitRecord::new();

        let oc: Vec3 = ray.orig - self.center; // (A-C)

        let a = ray.dir.length_squared(); //Vec3::dot(&ray.dir, ray.dir); // (t^2*B^2)
        let half_b = Vec3::dot(&oc, &ray.dir); //2.0 * Vec3::dot(&oc, ray.dir); // 2*((A-C)*(t*B))
        let c = oc.length_squared() - self.radius*self.radius; //Vec3::dot(&oc, oc) - (radius * radius); // (A-C)*(A-C) - R^2

        let discriminant = half_b * half_b -  a * c;

        if discriminant < 0.0 { // No solution (roots)
            return (false, hr);
        } else {
            let sqrtd = f64::sqrt(discriminant);

            // Find the nearest root that lies in the acceptable range.
            let root = (-half_b - sqrtd) / a;
            if root < t_min || t_max < root {
                let root = (-half_b + sqrtd) / a;

                if root < t_min || t_max < root {
                    return (false, hr);
                }
            }

            hr.t = root;
            hr.p = ray.at(hr.t);
            hr.normal = (hr.p - self.center) / self.radius; // Always points outward
            let outward_normal = hr.normal;
            hr.set_face_normal(ray, &outward_normal); // So we need to know whether the ray comes from inside or outside the sphere
            
            return (true, hr);
        }
    }
}

use crate::color::Color;
use crate::point::Point;
use crate::vector::Vec3;
use crate::hittable::Hittable;
use crate::hittable::HitRecord;

pub struct Ray {
    pub orig: Point,
    pub dir: Vec3,
}

impl Ray {
    pub fn at(&self, t: f64) -> Vec3 {
        self.orig + (self.dir * t)
    }

    fn hits_any_object(&self, objects : &Vec<Box<dyn Hittable>>, t_min : f64, t_max : f64) -> (bool, HitRecord) {
        let mut result_record = HitRecord::new();
        let mut hit_anything = false;
        let mut closest_so_far = t_max;

        for object in objects {
            // Check if every object hits, taking into account that objects further from 
            // the closest one (t_max > closest_so_far) won't
            let (hits, temp_record) = object.hit(&self,  t_min, closest_so_far);

            if hits {
                hit_anything = true;
                closest_so_far = temp_record.t;
                result_record = temp_record;
            }
        }

        (hit_anything, result_record)
    }

    pub fn ray_color(&self, objects : &Vec<Box<dyn Hittable>>) -> Color {
        //if let Some(t) = self.hits_sphere(Point {x: 0.0, y: 0.0,z: -1.0,},0.5) {
        let (hits, hit_record) = self.hits_any_object(objects, 0.0, f64::INFINITY);

        if hits {
            return Color{x: hit_record.normal.x+1.0, y: hit_record.normal.y+1.0, z: hit_record.normal.z+1.0} * 0.5;
        }

        /*
            Blends white and blue depending on the height of the y coordinate
            after scaling the ray direction to unit length (so −1.0 < y < 1.0)
        */
        let unit_direction = Vec3::unit_vector(&self.dir);
        let t = 0.5 * (unit_direction.y + 1.0);

        /* If t = 1.0 -> blue
           If t = 0.0 -> white
           else: blend between blue and white ->
                     blendedValue =
                     (1−t) ⋅ startValue
                     +
                     t ⋅ endValue
        */
        (Color {
            x: 1.0,
            y: 1.0,
            z: 1.0,
        } * (1.0 - t))
            + (Color {
                x: 0.5,
                y: 0.7,
                z: 1.0,
            } * t)
    }

    pub fn hits_sphere(&self, center: Point, radius: f64) -> Option<f64> {
        /*
            Eq. for sphere at center C(C_x, C_y, C_z):
                (x − C_x)^2 + (y − C_y)^2 + (z − C_z)^2 = R2

            In vector terms, for a point P(x, y, z):
                (P - C)*(P-C) = R^2

            We want to know if a ray P(t) = A + tB satisfies the equation (hits the sphere):
                (P(t)-C)*(P(t)-C) = R^2

            Expanding P(t):
                (A + tB - C)*(A + tB - C) = R^2

            Expanding further and creating an equation:
                t^2*B^2 + 2*t*B * (A-C) + (A-C)*(A-C) - R^2 = 0


            Optimized so that multiplications and divisions by two are removed in the quadratic formula
        */

        let oc: Vec3 = self.orig - center; // (A-C)

        let a = self.dir.length_squared(); //Vec3::dot(&self.dir, self.dir); // (t^2*B^2)
        let half_b = Vec3::dot(&oc, &self.dir); //2.0 * Vec3::dot(&oc, self.dir); // 2*((A-C)*(t*B))
        let c = oc.length_squared() - radius*radius; //Vec3::dot(&oc, oc) - (radius * radius); // (A-C)*(A-C) - R^2

        let discriminant = half_b * half_b -  a * c;

        if discriminant < 0.0 { // No solution (roots)
            return None
        } else {
            return Some((-half_b - f64::sqrt(discriminant)) / a);
        }
    }
}

use crate::color::Color;
use crate::point::Point;
use crate::vector::Vec3;
use crate::hittable::Hittable;
use crate::hittable::HitRecord;

use rand::rngs::ThreadRng;

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

    pub fn ray_color(&self, rng : &mut ThreadRng, objects : &Vec<Box<dyn Hittable>>, allowed_bounces_remaining : usize) -> Color {
        if allowed_bounces_remaining == 0 {
            return Color{x:0.0, y:0.0, z:0.0};
        }

        // min value of t=0.001 to avoid the shadow acne problem, where some rays may hit the object at very close values to t=0
        let (hits, hit_record) = self.hits_any_object(objects, 0.001, f64::INFINITY); 

        if hits {
            let target = hit_record.p + hit_record.normal + Vec3::random_unit_vector(rng);//Vec3::random_vector_in_hemisphere(rng, &hit_record.normal);
            
            let ray = Ray {
                orig: hit_record.p,
                dir: target - hit_record.p

            };

            return ray.ray_color(rng, objects, allowed_bounces_remaining-1) * 0.5;
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
}

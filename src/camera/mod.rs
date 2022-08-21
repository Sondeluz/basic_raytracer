use crate::point::Point;
use crate::vector::Vec3;
use crate::ray::Ray;

pub struct Camera {
    pub origin : Point,
    pub lower_left_corner : Point,
    pub horizontal : Vec3,
    pub vertical : Vec3
}

pub fn new() -> Camera {
        let aspect_ratio = 16.0 / 9.0;
        let viewport_height = 2.0;
        let viewport_width = aspect_ratio * viewport_height;
        let focal_length = 1.0; // Distance between the projection plane and the camera

        let origin = Point { // Eye of the camera
            x: 0.0,
            y: 0.0,
            z: 0.0,
        };


        let horizontal = Vec3 {
            x: viewport_width,
            y: 0.0,
            z: 0.0,
        };
        let vertical = Vec3 {
            x: 0.0,
            y: viewport_height,
            z: 0.0,
        };

        // Starting transversal point
        let lower_left_corner = origin
            - horizontal / 2.0
            - vertical / 2.0
            - Vec3 {
                x: 0.0,
                y: 0.0,
                z: focal_length, // -1, which is the plane's position in the z-axis (camera is at z=0)
            };

        
        Camera {
            origin,
            lower_left_corner,
            horizontal,
            vertical
        }
    }

impl Camera {
    pub fn get_ray(&self, u : f64, v: f64) -> Ray {
        Ray {
            orig: self.origin,
            dir: self.lower_left_corner + (self.horizontal * u) + (self.vertical * v) - self.origin
        }
    }
}

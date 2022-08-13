use crate::color::Color;
use crate::point::Point;
use crate::vector::Vec3;

pub struct Ray {
    pub orig: Point,
    pub dir: Vec3,
}

impl Ray {
    pub fn at(&self, t: f64) -> Vec3 {
        self.orig + (self.dir * t)
    }

    pub fn ray_color(&self) -> Color {
        if self.hits_sphere(
            Point {
                x: 0.0,
                y: 0.0,
                z: -1.0,
            },
            0.5,
        ) {
            return Color {
                x: 1.0,
                y: 0.0,
                z: 0.0,
            };
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

    pub fn hits_sphere(&self, center: Point, radius: f64) -> bool {
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


        */

        let oc: Vec3 = self.orig - center; // (A-C)

        let a = Vec3::dot(&self.dir, self.dir); // (t^2*B^2)
        let b = 2.0 * Vec3::dot(&oc, self.dir); // 2*((A-C)*(t*B))
        let c = Vec3::dot(&oc, oc) - (radius * radius); // (A-C)*(A-C) - R^2

        let discriminant = b * b - 4.0 * a * c;
        discriminant > 0.0 // If it's positive, it has two roots and as such it goes through the sphere
    }
}

use std::{thread, time};

mod color;
mod point;
mod ray;
mod vector;

fn main() {
    // Image
    let aspect_ratio = 16.0 / 9.0;
    let image_width = 400;
    let image_height = (image_width as f64 / aspect_ratio) as i64;

    // Camera setup
    let viewport_height = 2.0;
    let viewport_width = aspect_ratio * viewport_height;
    let focal_length = 1.0; // Distance between the projection plane and the camera

    let origin = point::Point { // Eye of the camera
        x: 0.0,
        y: 0.0,
        z: 0.0,
    };


    let horizontal = vector::Vec3 {
        x: viewport_width,
        y: 0.0,
        z: 0.0,
    };
    let vertical = vector::Vec3 {
        x: 0.0,
        y: viewport_height,
        z: 0.0,
    };

    // Starting transversal point
    let lower_left_corner = origin
        - horizontal / 2.0
        - vertical / 2.0
        - vector::Vec3 {
            x: 0.0,
            y: 0.0,
            z: focal_length, // -1, which is the plane's position in the z-axis (camera is at z=0)
        };

    // Rendering
    print!("P3\n{} {}\n255\n", image_width, image_height);

    // Moves the ray across the screen, first downwards in the y-axis and then right in the x-axis
    for j in (0..image_height).rev() {
        eprint!("\rScanlines remaining: {}   ", j);

        for i in 0..image_width {
            let u = i as f64 / (image_width - 1) as f64;
            let v = j as f64 / (image_height - 1) as f64;

            let r = ray::Ray {
                orig: origin,
                dir: lower_left_corner + (horizontal * u) + (vertical * v) - origin,
            };

            r.ray_color().write_color();
        }
    }

    eprint!("\nDone           ");
}

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

    // Camera
    let viewport_height = 2.0;
    let viewport_width = aspect_ratio * viewport_height;
    let focal_length = 1.0;

    let origin = point::Point {
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
    let lower_left_corner = origin
        - horizontal / 2.0
        - vertical / 2.0
        - vector::Vec3 {
            x: 0.0,
            y: 0.0,
            z: focal_length,
        };

    // Render
    print!("P3\n{} {}\n255\n", image_width, image_height);

    for j in (0..image_height).rev() {
        eprint!("\rScanlines remaining: {}   ", j);

        for i in 0..image_width {
            let u = i as f64 / (image_width - 1) as f64;
            let v = j as f64 / (image_height - 1) as f64;

            let r = ray::Ray {
                orig: origin,
                dir: lower_left_corner + horizontal * u + vertical * v - origin,
            };

            r.ray_color().write_color();
        }
    }

    eprint!("\nDone           ");
}

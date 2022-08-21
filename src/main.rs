mod color;
mod point;
mod ray;
mod vector;
mod hittable;
mod sphere;
mod camera;
mod utils;

use crate::point::Point;
use crate::sphere::Sphere;
use crate::hittable::Hittable;

use std::time::{Instant};

const MAX_RAY_BOUNCES : usize = 50;

fn main() {
    let mut rng = Box::new(rand::thread_rng());
    let start = Instant::now();

    // Image
    let aspect_ratio = 16.0 / 9.0;
    let image_width = 600;
    let image_height = (image_width as f64 / aspect_ratio) as i64;
    let samples_per_pixel = 100;

    // Objects
    let mut objects : Vec<Box<dyn Hittable>> = Vec::new();
    objects.push(Box::new(Sphere{center:Point{x:0.0,y:0.0,z:-1.0}, radius:0.5})); // Center
    objects.push(Box::new(Sphere{center:Point{x:0.0,y:-100.5,z:-1.0}, radius:100.0})); // Bigger

    // Objects
    let mut objects : Vec<Box<dyn Hittable>> = Vec::new();
    objects.push(Box::new(Sphere{center:Point{x:0.0,y:0.0,z:-1.0}, radius:0.5})); // Center
    //objects.push(Box::new(Sphere{center:Point{x:0.6,y:0.2,z:-1.0}, radius:0.3})); // Smaller, overlapping
    objects.push(Box::new(Sphere{center:Point{x:0.0,y:-100.5,z:-1.0}, radius:100.0})); // Bigger

    // Camera setup
    let camera = camera::new();

    // Rendering
    print!("P3\n{} {}\n255\n", image_width, image_height);

    // Moves the ray across the screen, first downwards in the y-axis and then right in the x-axis
    for j in (0..image_height).rev() {
        eprint!("\rScanlines remaining: {}   ", j);

        for i in 0..image_width {
            let mut pixel_color = color::new();

            // Pass a ray through multiple random samples and accumulate its colors
            for _ in 0..samples_per_pixel {
                
                let u = (i as f64 + utils::random_f64(&mut rng)) / (image_width - 1) as f64;
                let v = (j as f64 + utils::random_f64(&mut rng)) / (image_height - 1) as f64;
                
                let r = camera.get_ray(u, v);
                pixel_color += r.ray_color(&mut rng, &objects, MAX_RAY_BOUNCES);
            }

            // The written color will be the avg from the taken samples
            pixel_color.write_color(samples_per_pixel);
        }
    }

    let duration = start.elapsed();
    eprint!("\nDone. Time elapsed: {:?}", duration);
}

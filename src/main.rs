mod color;
mod point;
mod ray;
mod vector;
mod hittable;
mod sphere;
mod camera;
mod utils;
mod material;

use crate::point::Point;
use crate::sphere::Sphere;
use crate::hittable::Hittable;
use crate::color::Color;
use crate::material::lambertian::Lambertian;
use crate::material::metallic::Metallic;

use std::time::{Instant};
use std::rc::Rc;

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


    let material_ground = Rc::new(Lambertian{albedo: Color{x:0.8, y:0.8, z:0.0}});
    let material_center = Rc::new(Lambertian{albedo: Color{x:0.7, y:0.3, z:0.3}});
    let material_left = Rc::new(Metallic{albedo: Color{x:0.8, y:0.8, z:0.8}, fuzz:0.6});
    let material_right = Rc::new(Metallic{albedo: Color{x:0.4, y:0.1, z:0.2}, fuzz:0.0});

    objects.push(Box::new(Sphere{
        center:Point{x:0.0,y:-100.5,z:-1.0},   
        radius:100.0, 
        material: material_ground}));
    
    objects.push(Box::new(Sphere{
        center:Point{x:0.0,y:0.0,z:-1.0},      
        radius:0.5,     
        material: material_center}));
    
    objects.push(Box::new(Sphere{
        center:Point{x:-1.0,y:0.0,z:-1.0},     
        radius:0.5, 
        material: material_left}));
    
    objects.push(Box::new(Sphere{
        center:Point{x:1.0,y:0.0,z:-1.0},      
        radius:0.5, 
        material: material_right}));

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

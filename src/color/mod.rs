use crate::vector::Vec3;
use crate::utils::clamp;

pub type Color = Vec3;

impl Color {
    pub fn write_color(&self, samples_per_pixel : usize) {
        // Divide each color by the number of samples and gamma-correct (square) them
        let scale = 1.0 / samples_per_pixel as f64;
        let (r, g, b) = (f64::sqrt(self.x * scale), f64::sqrt(self.y * scale), f64::sqrt(self.z * scale));
        

        println!(
            "{} {} {}",
            (256.0 * clamp(r, 0.0, 0.999)) as i32,
            (256.0 * clamp(g, 0.0, 0.999)) as i32,
            (256.0 * clamp(b, 0.0, 0.999)) as i32
        );
    }
}

pub fn new() -> Color {
    Color{x:0.0,y:0.0,z:0.0}
}

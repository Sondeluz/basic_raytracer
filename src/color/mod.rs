use crate::vector::Vec3;

pub type Color = Vec3;

impl Color {
    pub fn write_color(&self) {
        println!(
            "{} {} {}",
            (255.999 * self.x) as i32,
            (255.999 * self.y) as i32,
            (255.999 * self.z) as i32
        );
    }
}

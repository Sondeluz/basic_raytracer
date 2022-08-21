use rand::rngs::ThreadRng;
use rand::Rng;

pub fn random_f64(rng : &mut ThreadRng) -> f64 {
    rng.gen_range(0.0..1.0)
}

pub fn random_f64_bounded(rng : &mut ThreadRng, min: f64, max: f64) -> f64 {
    min + ((max-min) * rng.gen_range(0.0..1.0))
}

pub fn clamp(value: f64, min: f64, max: f64) -> f64 {
    if value < min {return min}
    if value > max {return max}
    return value;
}


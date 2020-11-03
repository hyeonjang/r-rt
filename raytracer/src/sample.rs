use rand::distributions::{Distribution, Uniform};
use rxmath::vector::*;

pub fn random_f32() -> f32 {
    let mut rng = rand::thread_rng();
    let uniform_range = Uniform::from(0.0..1.0);
    let sampled = uniform_range.sample(&mut rng);
    return sampled;
}

pub fn random_f64() -> f64 {
    let mut rng = rand::thread_rng();
    let uniform_range = Uniform::from(0.0..1.0);
    let sampled = uniform_range.sample(&mut rng);
    return sampled;
}

pub fn random_range_f32(x:f32, y:f32) -> f32 {
    let mut rng = rand::thread_rng();
    let uniform_range = Uniform::from(x..y);
    let sampled = uniform_range.sample(&mut rng);
    return sampled;
}

pub fn random_range_f64(x:f64, y:f64) -> f64 {
    let mut rng = rand::thread_rng();
    let uniform_range = Uniform::from(x..y);
    let sampled = uniform_range.sample(&mut rng);
    return sampled;
}
pub fn random_unit_sphere() -> vec3 {
    let mut p = vec3(random_range_f64(-1f64, 1f64), random_range_f64(-1f64, 1f64), random_range_f64(-1f64, 1f64));
    return p;
}
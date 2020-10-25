use rand::distributions::{Distribution, Uniform};

pub fn random_f32() -> f32 {
    let mut rng = rand::thread_rng();
    let uniform_range = Uniform::from(0.0..1.0);
    let sampled = uniform_range.sample(&mut rng);
    return sampled;
}

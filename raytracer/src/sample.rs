use rxmath::vector::*;
use rxmath::random::*;

pub enum Sampling {
    Uniform,
}

pub fn random_unit_sphere() -> vec3 {
    return vec3(random_range_f32(-1f32, 1f32), random_range_f32(-1f32, 1f32), random_range_f32(-1f32, 1f32));
}

pub fn random_unit_vector() -> vec3 {
    return random_unit_sphere().normalize();
}

pub fn random_hemisphere(normal:vec3) -> vec3 {
    let unit_sphere = random_unit_sphere();
    if dot(unit_sphere, normal) > 0.0 {
        return unit_sphere;
    }
    else {
        return -unit_sphere;
    }
}

pub fn random_unit_disk() -> vec3 {
    loop {
        let p = vec3(random_f32(), random_f32(), 0.0);
        if p.length2() >= 1.0 { continue; }
        return p;
    }
}

pub fn sample( method:Sampling ) -> f32{
    match method {
        Sampling::Uniform => return random_f32(),
    }
}
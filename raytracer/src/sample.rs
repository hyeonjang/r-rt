use rxmath::vector::*;
use rxmath::random::*;

pub fn random_unit_sphere() -> vec3 {
    return vec3(random_range_f64(-1f64, 1f64), random_range_f64(-1f64, 1f64), random_range_f64(-1f64, 1f64));
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
        let p = vec3(random_f64(), random_f64(), 0.0);
        if p.length2() >= 1.0 { continue; }
        return p;
    }
}
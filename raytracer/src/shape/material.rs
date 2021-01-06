use rxmath::vector::*;
use rxmath::random::*;

use crate::sample::*;
use crate::intersect::*;

/// material
#[allow(non_camel_case_types)]
pub trait Material {
    fn scatter(&self, r:&ray, h:&hit, attenuation:&mut vec3, scattered:&mut ray) -> bool;
}

#[allow(non_camel_case_types)]
pub struct lambertian {
    pub albedo:vec3,
}

#[allow(non_camel_case_types)]
pub struct metal {
    albedo:vec3,
    fuzz:f32,
}

#[allow(non_camel_case_types)]
pub struct dielectric {
    ir:f32,
}

impl lambertian {
    pub fn new(v:vec3)->lambertian {
        lambertian{ albedo:v }
    }
}

impl metal {
    pub fn new(v:vec3, f:f32)->metal {
        metal{ albedo:v, fuzz:f }
    }
}

impl dielectric {
    pub fn new(r:f32) -> dielectric {
        dielectric{ ir:r }
    }
}

impl Material for lambertian {
    fn scatter(&self, _r:&ray, h:&hit, attenuation:&mut vec3, scattered:&mut ray) -> bool{
        let mut scatter_direction = h.norm + random_unit_vector();

        if scatter_direction.near_zero() {
            scatter_direction = h.norm;
        }

        *scattered = ray::new(h.pos, scatter_direction, None);
        *attenuation = self.albedo;
        return true;
    }
}

impl Material for metal {
    fn scatter(&self, r:&ray, h:&hit, attenuation:&mut vec3, scattered:&mut ray) -> bool{
        let reflected = reflect(r.dir.normalize(), h.norm);
        *scattered = ray::new(h.pos, reflected + random_unit_sphere()*self.fuzz, None);
        *attenuation = self.albedo;
        return dot(scattered.dir, h.norm) > 0.0;
    }
}

impl Material for dielectric {
    fn scatter(&self, r:&ray, h:&hit, attenuation:&mut vec3, scattered:&mut ray) -> bool{
        *attenuation = vec3(1.0, 1.0, 1.0);
        let refraction_ratio = { if h.front { 1.0/self.ir } else { self.ir } };

        let unit_direction = normalize(r.dir);
        let cos_theta = f32::min(dot(-unit_direction, h.norm), 1.0);
        let sin_theta = f32::sqrt(1.0-cos_theta*cos_theta);

        let cannot_refract = refraction_ratio*sin_theta > 1.0;
        let direction:vec3;

        if cannot_refract || dielectric::reflectance(cos_theta, refraction_ratio) > random_f32() {
            direction = reflect(unit_direction, h.norm);
        }
        else {
            direction = refract(unit_direction, h.norm, refraction_ratio);
        }

        *scattered = ray::new(h.pos, direction, None);
        return true;
    }
}

impl dielectric {
    pub fn reflectance(cosine:f32, ref_idx:f32) -> f32{
        let r0 = (1.0-ref_idx)/(1.0+ref_idx);
        let r1 = r0*r0;
        return r1 + (1.0-r1)*f32::powi(1.0-cosine, 5);
    }
}
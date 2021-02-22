use std::sync::*;

use rxmath::vector::*;
use rxmath::random::*;

use crate::sample::*;
use crate::intersect::ray::*;
use crate::intersect::hit::*;

use crate::texture::*;

/// material
#[allow(non_camel_case_types)]
pub trait Material {
    fn emitted(&self, u:f32, v:f32, p:&vec3) -> vec3 { return vec3(0.0, 0.0, 0.0); } 
    fn scatter(&self, r:&Ray, h:&Hit, attenuation:&mut vec3, scattered:&mut Ray) -> bool;
}

#[allow(non_camel_case_types)]
pub struct lambertian {
    pub albedo:Arc<dyn Texture>,
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

#[allow(non_camel_case_types)]
pub struct diffuse_light {
    emit:Arc<dyn Texture>,
}

impl lambertian {
    pub fn new(v:vec3) -> lambertian {
        lambertian { albedo:Arc::new(SolidColor::new(v)) }
    }
}

impl metal {
    pub fn new(v:vec3, f:f32) -> metal {
        metal { albedo:v, fuzz:f }
    }
}

impl dielectric {
    pub fn new(r:f32) -> dielectric {
        dielectric { ir:r }
    }
    pub fn reflectance(cosine:f32, ref_idx:f32) -> f32{
        let r0 = (1.0-ref_idx)/(1.0+ref_idx);
        let r1 = r0*r0;
        return r1 + (1.0-r1)*f32::powi(1.0-cosine, 5);
    }
}

impl diffuse_light {
    pub fn new(c:vec3) -> diffuse_light {
        diffuse_light { emit:Arc::new(SolidColor::new(c)), }
    }
}

impl Material for lambertian {
    fn scatter(&self, _r:&Ray, h:&Hit, attenuation:&mut vec3, scattered:&mut Ray) -> bool{
        let mut scatter_dection = h.norm + random_unit_vector();

        if scatter_dection.near_zero() {
            scatter_dection = h.norm;
        }

        *scattered = Ray::new(h.pos, scatter_dection, None);
        *attenuation = self.albedo.value(h.u, h.v, &h.pos);
        return true;
    }
}

impl Material for metal {
    fn scatter(&self, r:&Ray, h:&Hit, attenuation:&mut vec3, scattered:&mut Ray) -> bool {
        let reflected = reflect(r.d.normalize(), h.norm);
        *scattered = Ray::new(h.pos, reflected + random_unit_sphere()*self.fuzz, None);
        *attenuation = self.albedo;
        return dot(scattered.d, h.norm) > 0.0;
    }
}

impl Material for dielectric {
    fn scatter(&self, r:&Ray, h:&Hit, attenuation:&mut vec3, scattered:&mut Ray) -> bool {
        *attenuation = vec3(1.0, 1.0, 1.0);
        let refraction_ratio = { if h.front { 1.0/self.ir } else { self.ir } };

        let unit_dection = normalize(r.d);
        let cos_theta = f32::min(dot(-unit_dection, h.norm), 1.0);
        let sin_theta = f32::sqrt(1.0-cos_theta*cos_theta);

        let cannot_refract = refraction_ratio*sin_theta > 1.0;
        let dection:vec3;

        if cannot_refract || dielectric::reflectance(cos_theta, refraction_ratio) > random_f32() {
            dection = reflect(unit_dection, h.norm);
        }
        else {
            dection = refract(unit_dection, h.norm, refraction_ratio);
        }

        *scattered = Ray::new(h.pos, dection, None);
        return true;
    }
}

impl Material for diffuse_light {
    fn emitted(&self, u:f32, v:f32, p:&vec3) -> vec3 {
        return self.emit.value(u, v, p);
    } 
    fn scatter(&self, _r: &Ray, _h: &Hit, _attenuation: &mut vec3, _scattered: &mut Ray) -> bool {
        return false;
    }
}


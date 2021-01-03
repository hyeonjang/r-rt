// External crate
use std::sync::Arc;

// Custom crate
use rxmath::vector::*;
use rxmath::random::*;

use crate::intersect::*;
use crate::sample::*;
use crate::bbox::*;

// ShapeList
pub struct ShapeList<T: Shape>{
    pub list : Vec<T>,
}

impl<T: Shape> ShapeList<T> {
    pub fn new() -> Self {
        ShapeList{ list:Vec::new() }
    }
    pub fn clear(&mut self) { 
        self.list.clear(); 
    }
    pub fn push(&mut self, shape:T) {
        self.list.push(shape)
    }
    pub fn hit(&self, r:&ray, t_min:f32, t_max:f32, h:&mut hit) -> bool {
        let mut i = hit::default();
        let mut hit = false;
        let mut closest = t_max;

        for object in &self.list {
            if object.intersect(r, t_min, closest, &mut i) {
                hit = true;
                closest = i.t.clone();
                *h = i.clone();
            }
        }
        return hit;
    }
}

// traits for all kind of Shapes
pub trait Shape {
    fn intersect( &self, r:&ray, t_min:f32, t_max:f32, i:&mut hit ) -> bool;
    fn bbox( &self, b:BBox ) -> bool;
}

// 1. Sphere
pub struct Sphere {
    pub center : vec3,
    pub radius : f32,
    pub mat_ptr : Arc<dyn material>,
}

impl Sphere {
    pub fn new(center:vec3, radius:f32, mat_ptr:Arc<dyn material>) -> Self {
        Sphere { center:center, radius:radius, mat_ptr:mat_ptr }
    }
}

impl Shape for Sphere {
    fn intersect( &self, r:&ray, t_min:f32, t_max:f32, h:&mut hit ) -> bool {
        // the simplified version
        let oc:vec3 = r.o - self.center;
        let a = r.dir.length2();
        let b = oc.dot(r.dir);
        let c = oc.length2() - self.radius*self.radius;
        
        let discriminant = b*b - a*c;
        if discriminant<0.0 { return false;}  
        let sqrtd = f32::sqrt(discriminant);

        let mut root = (-b-sqrtd)/a;
        if root<t_min || t_max<root {
            root = (-b+sqrtd)/a;
            if root<t_min || t_max<root {
                return false;
            }
        }

        h.t = root;
        h.pos = r.at(h.t);
        let outward_normal = (h.pos - self.center)/self.radius;
        h.set_face_normal(r, outward_normal);
        h.mat_ptr = Arc::clone(&self.mat_ptr);
        return true;
    }
    fn bbox( &self, b:BBox ) -> bool {
        return true;
    }
}


/// material
#[allow(non_camel_case_types)]
pub trait material {
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

impl material for lambertian {
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

impl material for metal {
    fn scatter(&self, r:&ray, h:&hit, attenuation:&mut vec3, scattered:&mut ray) -> bool{
        let reflected = reflect(r.dir.normalize(), h.norm);
        *scattered = ray::new(h.pos, reflected + random_unit_sphere()*self.fuzz, None);
        *attenuation = self.albedo;
        return dot(scattered.dir, h.norm) > 0.0;
    }
}

impl material for dielectric {
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
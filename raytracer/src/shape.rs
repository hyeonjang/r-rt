// External crate
use std::rc::Rc;

// Custom crate
use rxmath::vector::*;

use crate::intersect::*;
use crate::sample::*;

pub trait Shape {
    fn intersect( &self, r:&ray, t_min:f64, t_max:f64, i:&mut hit ) -> bool;
}

pub struct ShapeList<T: Shape>{
    pub list : Vec<T>,
}

pub struct Sphere {
    pub center : vec3,
    pub radius : f64,
    pub mat_ptr : Rc<dyn material>,
}

impl Shape for Sphere {
    fn intersect( &self, r:&ray, t_min:f64, t_max:f64, h:&mut hit ) -> bool {
        // the simplified version
        let oc:vec3 = r.o - self.center;
        let a = r.dir.length2();
        let b = oc.dot(r.dir);
        let c = oc.length2() - self.radius*self.radius;
        
        let discriminant = b*b - a*c;
        if discriminant<0f64 { return false;}  
        let sqrtd = sqrt(discriminant);

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
        h.mat_ptr = Rc::clone(&self.mat_ptr);
        return true;
    }
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
    pub fn hit(&self, r:&ray, t_min:f64, t_max:f64, h:&mut hit) -> bool {
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

/// material
/// 

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
    fuzz:f64,
}

#[allow(non_camel_case_types)]
pub struct dielectric {
    ir:f64,
}

impl lambertian {
    pub fn new(x:f64, y:f64, z:f64)->lambertian {
        lambertian{ albedo:vec3(x, y, z) }
    }
}

impl metal {
    pub fn new(v:vec3, f:f64)->metal {
        metal{ albedo:v, fuzz:f }
    }
}

impl dielectric {
    pub fn new(r:f64) -> dielectric {
        dielectric{ ir:r }
    }
}

impl material for lambertian {
    fn scatter(&self, _r:&ray, h:&hit, attenuation:&mut vec3, scattered:&mut ray) -> bool{
        let mut scatter_direction = h.norm + random_unit_vector();

        if scatter_direction.near_zero() {
            scatter_direction = h.norm;
        }

        *scattered = ray::new(h.pos, scatter_direction);
        *attenuation = self.albedo;
        return true;
    }
}

impl material for metal {
    fn scatter(&self, r:&ray, h:&hit, attenuation:&mut vec3, scattered:&mut ray) -> bool{
        let reflected = reflect(r.dir.normalize(), h.norm);
        *scattered = ray::new(h.pos, reflected + random_unit_sphere()*self.fuzz);
        *attenuation = self.albedo;
        return dot(scattered.dir, h.norm) > 0.0;
    }
}

impl material for dielectric {
    fn scatter(&self, r:&ray, h:&hit, attenuation:&mut vec3, scattered:&mut ray) -> bool{
        *attenuation = vec3(1.0, 1.0, 1.0);
        let refraction_ratio = { if h.front { 1.0/self.ir } else { self.ir } };

        let unit_direction = normalize(r.dir);
        let cos_theta = dot(-unit_direction, h.norm).min(1.0);
        let sin_theta = sqrt(1.0-cos_theta*cos_theta);

        let cannot_refract = refraction_ratio*sin_theta > 1.0;
        let direction:vec3;

        if cannot_refract || dielectric::reflectance(cos_theta, refraction_ratio) > random_f64() {
            direction = reflect(unit_direction, h.norm);
        }
        else {
            direction = refract(unit_direction, h.norm, refraction_ratio);
        }

        *scattered = ray::new(h.pos, direction);
        return true;
    }
}

impl dielectric {
    pub fn reflectance(cosine:f64, ref_idx:f64) -> f64{
        let r0 = (1.0-ref_idx)/(1.0+ref_idx);
        let r1 = r0*r0;
        return r1 + (1.0-r1)*(1.0-cosine).powi(5);
    }
}
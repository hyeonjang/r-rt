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
    pub mat_ptr : Box<material>,
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
        h.norm = (h.pos - self.center)/self.radius;
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
                closest = i.t;
                *h = i;
                //println!("{}", h.t)
            }
        }
        return hit;
    }
}

/// material
/// 

#[allow(non_camel_case_types)]
pub trait material {
    fn scatter( &self, r:&ray, h:&hit, attenuation:&mut vec3, scattered:&mut ray) -> bool;
}

#[allow(non_camel_case_types)]
pub struct lambertian {
    albedo:vec3,
}

impl lambertian {
    pub fn new(v:vec3)->lambertian {
        lambertian{ albedo:v }
    }
}

#[allow(non_camel_case_types)]
pub struct metal {
    albedo:vec3,
}

impl material for lambertian {
    fn scatter( &self, _r:&ray, h:&hit, attenuation:&mut vec3, scattered:&mut ray) -> bool{
        let mut scatter_direction = h.norm + random_unit_vector();
        let temp = ray{o:h.pos, dir:scatter_direction};

        if scatter_direction.near_zero() {
            scatter_direction = h.norm;
        }

        *scattered = temp;
        *attenuation = self.albedo;
        return true;
    }
}

impl material for metal {
    fn scatter( &self, _r:&ray, h:&hit, attenuation:&mut vec3, scattered:&mut ray) -> bool{
        let mut scatter_direction = h.norm + random_unit_sphere();
        let temp = ray{o:h.pos, dir:scatter_direction};

        if scatter_direction.near_zero() {
            scatter_direction = h.norm;
        }

        *scattered = temp;
        *attenuation = self.albedo;
        return true;
    }
}
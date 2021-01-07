use std::sync::Arc;

use rxmath::vector::*;

use crate::shape::material::*;

pub trait Intersect {
    fn intersect(r:&ray) -> bool;
}

#[allow(non_camel_case_types)]
#[derive(Default, Copy, Clone)]
pub struct ray {
    pub o : vec3,
    pub dir : vec3,
    pub tm : f32,
}

impl ray {
    pub fn at(&self, t:f32) -> vec3 {
        return self.o + self.dir*t;
    } 
    pub fn new(o:vec3, d:vec3, t:Option<f32>) -> ray {
        match t {
            Some(p) =>  ray{ o:o, dir:d, tm:t.unwrap() },
            None => ray{ o:o, dir:d , tm:0.0 },
        }
       
    }
}

#[allow(non_camel_case_types)]
#[derive(Clone)]
pub struct hit {
    pub pos   : vec3,
    pub norm  : vec3,
    pub t     : f32,
    pub front : bool,
    pub mat_ptr : Arc<dyn Material>,
}

impl std::default::Default for hit {
    fn default() -> hit {
        hit { pos:vec3(0.0, 0.0, 0.0), norm:vec3(0.0, 0.0, 0.0), t:0.0, front:true, mat_ptr:Arc::new(lambertian::new(vec3(0.0, 0.0, 0.0))) }
    }
}


impl hit {
    #[inline] pub fn set_face_normal(&mut self, r: &ray, on:vec3) {
        if dot(r.dir, on) > 0.0 {
            self.norm = - on;
            self.front = false;
        } else {
            self.norm = on;
            self.front = true;
        }
    }
}






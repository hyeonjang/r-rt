use std::sync::Arc;

use rxmath::vector::*;

use crate::shape::material::*;
use crate::intersect::*;

#[allow(non_camel_case_types)]
#[derive(Clone)]
pub struct Hit {
    pub pos   : vec3,
    pub norm  : vec3,
    pub t_min : f32,
    pub t_max : f32,
    pub u     : f32,
    pub v     : f32,
    pub front : bool,
    pub mat_ptr : Arc<dyn Material>,
}

impl std::default::Default for Hit {
    fn default() -> Hit {
        Hit { 
                pos  :vec3::default(), 
                norm :vec3::default(), 
                t_min:f32::MAX, 
                t_max:f32::MAX,
                u    :f32::MAX,
                v    :f32::MAX,
                front:true, 
                mat_ptr:Arc::new(lambertian::new(vec3(0.0, 0.0, 0.0)))
            }
    }
}

impl Hit {
    #[inline] pub fn set_face_normal(&mut self, r: &Ray, on:vec3) {
        if dot(r.d, on) > 0.0 {
            self.norm = - on;
            self.front = false;
        } else {
            self.norm = on;
            self.front = true;
        }
    }
}






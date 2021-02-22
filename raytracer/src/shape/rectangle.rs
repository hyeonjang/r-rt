use std::sync::*;

use crate::shape::*;
use material::*;

use crate::accelerator::*;
use bounds::*;

pub struct Rectangle {
    x0 : f32, 
    x1 : f32, 
    y0 : f32, 
    y1 : f32, 
    k  : f32, 
    pub mat_ptr :Arc<dyn Material>,
}

impl Rectangle {
    pub fn new(x0:f32, x1:f32, y0:f32, y1:f32, k:f32, mat_ptr:Arc<dyn Material>,) -> Self {
        Rectangle { x0:x0, x1:x1, y0:y0, y1:y1, k:k, mat_ptr:mat_ptr }
    }
}

impl Shape for Rectangle {
    fn bounds(&self) -> Bounds3f {
        Bounds3f::default()
    }
}

impl Intersect for Rectangle {
    fn intersect(&self, r: &Ray, t_min:f32, t_max:f32, i: &mut Hit) -> bool {
        let t = (self.k-r.o.z)/r.d.z;
        if t<t_min || t>t_max {
            return false;
        }
        let x = r.o.x + t*r.d.x;
        let y = r.o.y + t*r.d.y;
        if x<self.x0 || x>self.x1 || y<self.y0 || y>self.y1 {
            return false;
        } 
        println!("{} {} {} \n{} {} {}", x, self.x0, self.x1, y, self.y0, self.y1);
        i.u = (x-self.x0)/(self.x1-self.x0);
        i.v = (y-self.y0)/(self.y1-self.y0);
        i.t_min = t;
        let outward_normal = vec3(0.0, 0.0, 1.0);
        i.set_face_normal(r, outward_normal);
        i.mat_ptr = self.mat_ptr.clone();
        i.pos = r.at(t);
        return true;
    }
    
}
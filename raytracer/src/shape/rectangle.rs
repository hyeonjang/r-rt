use std::sync::*;

use crate::shape::*;
use material::*;

use crate::accelerator::*;
use bounds::*;

pub struct RectangleXY {
    x0 : f32, 
    x1 : f32, 
    y0 : f32, 
    y1 : f32, 
    k  : f32, 
    pub mat_ptr :Arc<dyn Material>,
}

pub struct RectangleYZ {
    y0 : f32, 
    y1 : f32, 
    z0 : f32, 
    z1 : f32, 
    k  : f32, 
    pub mat_ptr :Arc<dyn Material>,
}

pub struct RectangleXZ {
    x0 : f32, 
    x1 : f32, 
    z0 : f32, 
    z1 : f32, 
    k  : f32, 
    pub mat_ptr :Arc<dyn Material>,
}

impl RectangleXY {
    pub fn new(x0:f32, x1:f32, y0:f32, y1:f32, k:f32, mat_ptr:Arc<dyn Material>,) -> Self {
        RectangleXY { x0:x0, x1:x1, y0:y0, y1:y1, k:k, mat_ptr:mat_ptr }
    }
}

impl RectangleYZ {
    pub fn new(y0:f32, y1:f32, z0:f32, z1:f32, k:f32, mat_ptr:Arc<dyn Material>,) -> Self {
        RectangleYZ { y0:y0, y1:y1, z0:z0, z1:z1, k:k, mat_ptr:mat_ptr }
    }
}

impl RectangleXZ {
    pub fn new(x0:f32, x1:f32, z0:f32, z1:f32, k:f32, mat_ptr:Arc<dyn Material>,) -> Self {
        RectangleXZ { x0:x0, x1:x1, z0:z0, z1:z1, k:k, mat_ptr:mat_ptr }
    }
}

impl Shape for RectangleXY {
    fn bounds(&self) -> Bounds3f {
        Bounds3f::default()
    }
}

impl Shape for RectangleYZ {
    fn bounds(&self) -> Bounds3f {
        Bounds3f::default()
    }
}

impl Shape for RectangleXZ {
    fn bounds(&self) -> Bounds3f {
        Bounds3f::default()
    }
}

impl Intersect for RectangleXY {
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
        i.u = (x-self.x0)/(self.x1-self.x0);
        i.v = (y-self.y0)/(self.y1-self.y0);
        i.t_min = t;
        let outward_normal = vec3(0.0, 1.0, 1.0);
        i.set_face_normal(r, outward_normal);
        i.mat_ptr = self.mat_ptr.clone();
        i.pos = r.at(t);
        return true;
    }
}

impl Intersect for RectangleYZ {
    fn intersect(&self, r: &Ray, t_min:f32, t_max:f32, i: &mut Hit) -> bool {
        let t = (self.k-r.o.x)/r.d.x;
        if t<t_min || t>t_max {
            return false;
        }
        let y = r.o.y + t*r.d.y;
        let z = r.o.z + t*r.d.z;
        
        if y<self.y0 || y>self.y1 || z<self.z0 || z>self.z1 {
            return false;
        }
        i.u = (y-self.y0)/(self.y1-self.y0);
        i.v = (z-self.z0)/(self.z1-self.z0);
        i.t_min = t;
        let outward_normal = vec3(1.0, 0.0, 0.0);
        i.set_face_normal(r, outward_normal);
        i.mat_ptr = self.mat_ptr.clone();
        i.pos = r.at(t);
        return true;
    }
}

impl Intersect for RectangleXZ {
    fn intersect(&self, r: &Ray, t_min:f32, t_max:f32, i: &mut Hit) -> bool {
        let t = (self.k-r.o.y)/r.d.y;
        if t<t_min || t>t_max {
            return false;
        }
        let x = r.o.x + t*r.d.x;
        let z = r.o.z + t*r.d.z;
        
        if x<self.x0 || x>self.x1 || z<self.z0 || z>self.z1 {
            return false;
        }
        i.u = (x-self.x0)/(self.x1-self.x0);
        i.v = (z-self.z0)/(self.z1-self.z0);
        i.t_min = t;
        let outward_normal = vec3(0.0, 1.0, 0.0);
        i.set_face_normal(r, outward_normal);
        i.mat_ptr = self.mat_ptr.clone();
        i.pos = r.at(t);
        return true;
    }
}
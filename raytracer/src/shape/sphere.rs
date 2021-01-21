// External crate
use std::sync::Arc;

// Custom crate
use rxmath::vector::*;

// out direc
use crate::intersect::*;

// in direc
use crate::shape::*;
use crate::shape::material::*;

// 1. Sphere
pub struct Sphere {
    pub center :vec3,
    pub radius :f32,
    pub mat_ptr :Arc<dyn Material>,
}

impl Sphere {
    pub fn new(center:vec3, radius:f32, mat_ptr:Arc<dyn Material>) -> Self {
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
    fn bounds(&self) -> Bounds {
        Bounds::new(self.center-vec3(self.radius, self.radius, self.radius), self.center+vec3(self.radius, self.radius, self.radius))
    }
}

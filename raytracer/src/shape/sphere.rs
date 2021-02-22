// External crate
use std::sync::Arc;

// Custom crate
use rxmath::vector::*;

use crate::intersect::*;
use hit::*;
use ray::*;

use crate::shape::*;
use material::*;

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
    pub fn get_uv(&self, p:&vec3, u:&mut f32, v:&mut f32) {
        let theta = f32::acos(-p.y);
        let phi   = f32::atan2(-p.z, p.x) + 3.14;

        *u = phi/(2.0*3.14);
        *v = theta/3.14;
    }
}

impl Intersect for Sphere {
    fn intersect( &self, r:&Ray, h:&mut Hit ) -> bool {
        // the simplified version
        let oc:vec3 = r.o - self.center;
        let a = r.d.length2();
        let b = oc.dot(r.d);
        let c = oc.length2() - self.radius*self.radius;
        
        let discriminant = b*b - a*c; if discriminant<0.0 { return false; }  
        let sqrtd = f32::sqrt(discriminant);

        let mut root = (-b-sqrtd)/a;
        if root<0.001 || h.t_max<root {
            root = (-b+sqrtd)/a;
            if root<0.001 || h.t_max<root {
                return false;
            }
        }

        h.t_min = root;
        h.pos = r.at(h.t_min);
        let outward_normal = (h.pos - self.center)/self.radius;
        h.set_face_normal(r, outward_normal);
        self.get_uv(&outward_normal, &mut h.u, &mut h.v);
        h.mat_ptr = Arc::clone(&self.mat_ptr);
        return true;
    }
}

impl Shape for Sphere {
    fn bounds(&self) -> Bounds3f {
        Bounds3f::new(self.center-vec3(self.radius, self.radius, self.radius), self.center+vec3(self.radius, self.radius, self.radius))
    }
}

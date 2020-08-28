use rxmath::vector::*;
use rxmath::matrix::*;
/////////////////////////
use crate::intersect::*;
/////////////////////////
/// 
pub trait Shape {
   fn intersect( &self, r : &Ray ) -> bool;
}

pub struct Sphere {
    pub center : Fpoint3,
    pub radius : f32,
}

impl Shape for Sphere {
    fn intersect( &self, r: &Ray ) -> bool {
        let oc = r.o - self.center;
        let a = dot(r.d, r.d);
        let b = 2.0 * dot(oc, r.d);
        let c = dot(oc, oc) - self.radius*self.radius;
        let discriminat = b*b - 4f32*a*c;
        
        if discriminat < 0.0 {
            return false;
        } 
        else {
            return true; 
        }
    }
}

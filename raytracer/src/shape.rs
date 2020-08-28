use rxmath::vector::*;
use rxmath::matrix::*;
/////////////////////////
use crate::intersect::*;
/////////////////////////
/// 
pub trait Shape {
   fn intersect( &self, r:&Ray, i:&mut Iact ) -> bool;
}

pub struct Sphere {
    pub center : Fpoint3,
    pub radius : f32,
}

impl Shape for Sphere {
    fn intersect( &self, r:&Ray, i:&mut Iact ) -> bool {
        let oc = r.o - self.center;
        let a = dot(r.d, r.d);
        let b = 2.0 * dot(oc, r.d);
        let c = dot(oc, oc) - self.radius*self.radius;
        let discriminant = b*b - 4f32*a*c;
        
        if discriminant < 0.0 { return false; } 
      
        i.t =  -b - sqrt(discriminant)/(2.0*a);
        i.norm = normalize(i.pos - self.center);
        i.norm *= -dot(i.norm, r.d).sin();

        return true;
    }
}

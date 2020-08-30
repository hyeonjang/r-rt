use rxmath::vector::*;
use rxmath::matrix::*;
/////////////////////////
use crate::intersect::*;
/////////////////////////
/// 
pub trait Shape {
    fn intersect( &self, r:&Ray, i:&mut Iact ) -> bool;
}

pub struct ShapeList<T: Shape>{
    pub list : Vec<T>,
}

pub struct Sphere {
    pub center : Fpoint3,
    pub radius : f32,
}

impl Shape for Sphere {
    fn intersect( &self, r:&Ray, i:&mut Iact ) -> bool {
        // the simplified version
        let oc = r.o - self.center;
        let a = dot(r.dir, r.dir);
        let b = dot(oc, r.dir);
        let c = dot(oc, oc) - self.radius*self.radius;
        
        let discriminant = b*b - a*c;
        if discriminant < 0.0 { return false; } 
      
        i.t     = -b - sqrt(discriminant)/a;
        i.tfar  = i.t;
        i.pos   = r.at(i.t);
        let on  = (i.pos - self.center)/self.radius;
        i.set_face_normal(r, &on);

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
    pub fn hit(&self, r:&Ray, i:&mut Iact) -> bool {
        let mut hit = false;
        for object in &self.list {
            if object.intersect(r, i) {
                hit = true;
            }
        }
        return hit;
    }
}
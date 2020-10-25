use rxmath::vector::*;
//use rxmath::matrix::*;
/////////////////////////
use crate::intersect::*;
/////////////////////////
/// 
pub trait Shape {
    fn intersect( &self, r:&Ray, t_min:f32, t_max:f32, i:&mut hit ) -> bool;
}

pub struct ShapeList<T: Shape>{
    pub list : Vec<T>,
}

pub struct Sphere {
    pub center : vec3,
    pub radius : f32,
}

impl Shape for Sphere {
    fn intersect( &self, r:&Ray, t_min:f32, t_max:f32, h:&mut hit ) -> bool {
        // the simplified version
        let oc:vec3 = r.o - self.center;
        let a = dot(r.dir, r.dir).sqrt();
        let b = dot(oc, r.dir);
        let c = dot(oc, oc).sqrt() - self.radius*self.radius;
        let discriminant = b*b - a*c;

        if discriminant > 0f32 {
            let root = sqrt(discriminant);
            let mut temp = (-b-root)/a;

            if temp<t_max && temp>t_min {
                h.t = temp;
                h.pos = r.at(h.t);
                h.norm = (h.pos - self.center)/self.radius;
                return true;
            }

            temp = (-b+root)/a;
            if temp<t_max && temp>t_min {
                h.t = temp;
                h.pos = r.at(h.t);
                h.norm = (h.pos - self.center)/self.radius;
                return true;
            }
        }
        return false;
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
    pub fn hit(&self, r:&Ray, t_min:f32, t_max:f32, h:&mut hit) -> bool {
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
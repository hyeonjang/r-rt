pub mod material;
pub mod sphere;

use crate::intersect::*;
use crate::sample::*;

use crate::accelerator::*;
use bbox::*;

// traits for all kind of Shapes
pub trait Shape
{
    fn intersect( &self, r:&ray, t_min:f32, t_max:f32, i:&mut hit ) -> bool;
    fn bbox( &self, b:BBox ) -> bool;
}

pub struct Rectangle
{

}

impl Shape for Rectangle 
{
    fn intersect( &self, r:&ray, t_min:f32, t_max:f32, i:&mut hit ) -> bool
    {
        return true;
    }
    fn bbox( &self, b:BBox ) -> bool 
    {
        return true;
    }
}

// ShapeList
pub struct ShapeList {
    pub list : Vec<Box<dyn Shape>>,
    pub acc : Option<bvh::BVH>,
}

impl ShapeList
{
    pub fn new() -> Self {
        ShapeList{ list:vec![], acc:None }
    }
    pub fn clear(&mut self) { 
        self.list.clear();
    }
    pub fn push<T>(&mut self, shape:T) where T:Shape +'static {
        self.list.push(Box::new(shape))
    }
    pub fn hit(&self, r:&ray, t_min:f32, t_max:f32, h:&mut hit) -> bool {
        
        let mut i = hit::default();
        let mut hit = false;
        let mut closest = t_max;

        for object in &self.list {
            if object.intersect(r, t_min, closest, &mut i) {
                hit = true;
                closest = i.t.clone();
                *h = i.clone();
            }
        }
        return hit;
    }
    pub fn acc_build(&self)
    {
        //self.acc.unwrap().build(self.list);
    }
}




pub mod bounds;
pub mod bvh;
pub mod kdtree;

use crate::intersect::*;
use crate::shape::*;

#[allow(non_camel_case_types)]
pub enum AcceleratorType {
    //kDTree,
    BVH,
}

pub trait Accelerator : Intersect {
    fn build(&mut self, primitives:&Vec<Box<dyn Shape>>);
}

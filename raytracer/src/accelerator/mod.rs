pub mod bounds;
pub mod bvh;

use crate::shape::*;

#[allow(non_camel_case_types)]
pub enum AcceleratorType {
    kDTree,
    BVH,
}

pub trait Accelerator {
    fn build<'a>(&self, primitives:&Vec<Box<dyn Shape + 'a>>);
    fn hit(&self) -> bool;
}
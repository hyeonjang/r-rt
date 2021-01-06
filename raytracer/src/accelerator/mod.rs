pub mod bbox;
pub mod bvh;

use crate::shape::*;

pub trait Accelerator {
    type Output;
    fn build(&self, primitives:&Vec<Box<dyn Shape>>) -> Self::Output;
    fn hit() -> bool;
}
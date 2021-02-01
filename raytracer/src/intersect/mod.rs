pub mod hit;
pub mod ray;

use ray::*;
use hit::*;

pub trait Intersect {
    fn intersect(&self, ray:&Ray, isect:&mut Hit) -> bool;
}
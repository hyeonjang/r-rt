pub mod hit;
pub mod ray;

use ray::*;
use hit::*;

pub trait Intersect {
    fn intersect(&self, ray:&Ray, t_min:f32, t_max:f32, h:&mut Hit) -> bool;
}
use std::sync::*;

use rxmath::vector::*;

use accelerator::*;
use bounds::*;

use crate::*;
use material::*;
use rectangle::*;

pub struct Prism {
    min   : vec3,
    max   : vec3,
    sides : ShapeList,
}

impl Prism {
    pub fn new(p0:vec3, p1:vec3, ptr:Arc<dyn Material>) -> Self {
        let mut sides = ShapeList::new();

        sides.push(RectangleXY::new(p0.x, p1.x, p0.y, p1.y, p1.z, ptr.clone()));
        sides.push(RectangleXY::new(p0.x, p1.x, p0.y, p1.y, p1.z, ptr.clone()));

        sides.push(RectangleXZ::new(p0.x, p1.x, p0.z, p1.z, p1.y, ptr.clone()));
        sides.push(RectangleXZ::new(p0.x, p1.x, p0.z, p1.z, p1.y, ptr.clone()));

        sides.push(RectangleYZ::new(p0.y, p1.y, p0.z, p1.z, p1.x, ptr.clone()));
        sides.push(RectangleYZ::new(p0.y, p1.y, p0.z, p1.z, p1.x, ptr.clone()));

        Prism { min:p0, max:p1, sides:sides }
    }
}

impl Shape for Prism {
    fn bounds(&self) -> Bounds3f {
        Bounds3f{ min:self.min, max:self.max }
    }
}

impl Intersect for Prism {
    fn intersect(&self, ray: &Ray, t_min: f32, t_max: f32, isect: &mut Hit) -> bool {
        return self.sides.intersect(ray, t_min, t_max, isect);
    }
}
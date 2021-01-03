use rxmath::vector::*;

use crate::intersect::*;

pub trait Union {
    type Output;
    fn bbox(b0:BBox, b1:BBox) -> Self::Output;
    fn vec(v0:vec3, v1:vec3) -> Self::Output;
    fn expand(&mut self, b:BBox);
}

impl Union for BBox {
    type Output = BBox;

    fn bbox(c0:BBox, c1:BBox) -> Self::Output {
        let min = vec3(f32::min(c0.min.x, c1.min.x), f32::min(c0.min.y, c1.min.y), f32::min(c0.min.z, c1.min.z));
        let max = vec3(f32::max(c0.max.x, c1.max.x), f32::max(c0.max.y, c1.max.y), f32::max(c0.max.z, c1.max.z));
        return BBox::new(min, max);
    }
    fn vec(c0:vec3, c1:vec3) -> Self::Output {
        let min = vec3(f32::min(c0.x, c1.x), f32::min(c0.y, c1.y), f32::min(c0.z, c1.z));
        let max = vec3(f32::max(c0.x, c1.x), f32::max(c0.y, c1.y), f32::max(c0.z, c1.z));
        return BBox::new(min, max);
    }
    fn expand(&mut self, b:BBox){
        self.min = vec3(f32::min(self.min.x, b.min.x), f32::min(self.min.x, b.min.x), f32::min(self.min.x, b.min.x));
        self.max = vec3(f32::max(self.max.x, b.max.x), f32::max(self.min.x, b.min.x), f32::max(self.min.x, b.min.x));
    }
}

#[derive(Copy, Clone)]
pub struct BBox {
    pub min:vec3,
    pub max:vec3,
}

impl BBox {
    pub fn new(min:vec3, max:vec3) -> Self {
        BBox { min:min, max:max }
    }
    // optimized code by Andrew Kensler at Pixar
    pub fn hit(r:ray, mut t_min:f32, mut t_max:f32) -> bool {
        for i in 0..3 {
            let inv_d = 1.0 / r.dir[i];
            let mut t0 = (vec3::min()[i] - r.o[i]) * inv_d;
            let mut t1 = (vec3::max()[i] - r.o[i]) * inv_d;
            if inv_d<0.0 { std::mem::swap(&mut t0, &mut t1); }
            t_min = if t0>t_min { t0 } else { t_min };
            t_max = if t1<t_max { t1 } else { t_max };
            if t_max<=t_min { return false }
        }
        return true;
    }
}

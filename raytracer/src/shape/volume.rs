// External crate
use std::sync::Arc;

// Custom crate
use rxmath::vector::*;

use crate::intersect::*;
use hit::*;
use ray::*;

use crate::shape::*;
use material::*;

use crate::texture::*;

pub struct ConstantMedium {
    boundary:Arc<dyn Shape>,
    phase_function:Arc<dyn Material>,
    neg_inv_density:f32,     
}

impl ConstantMedium {
   pub fn new(b:Arc<dyn Shape>, d:f32, a:vec3) -> Self {
        ConstantMedium { boundary:b, neg_inv_density:-1.0/d, phase_function:Arc::new(isotropic::new(a)) }
   }
}

impl Intersect for ConstantMedium {
    fn intersect(&self, ray: &Ray, t_min: f32, t_max: f32, hit0:&mut Hit) -> bool {
        // from rt oneweekend, print occasional samples when debugging.
        let b_debug = false;
        let debugging = b_debug && random_f32()<0.00001;

        let mut hit1 : Hit = Hit::default();
        let mut hit2 : Hit = Hit::default();

        if !self.boundary.intersect(ray, -f32::MAX, f32::MAX, &mut hit1) {
            return false;
        }

        if !self.boundary.intersect(ray, hit1.t_min+0.0001, f32::MAX, &mut hit2) {
            return false;
        }

        if debugging { println!("{}{}", hit1.t_min, hit2.t_min); }

        if hit1.t_min<t_min { hit1.t_min=t_min }
        if hit2.t_min<t_min { hit2.t_min=t_min }

        if hit1.t_min>=hit2.t_min { return false }

        let ray_length = ray.d.length();
        let distance_inside_boundary = (hit2.t_min - hit1.t_min)*ray_length;
        let hit_distance = self.neg_inv_density * f32::log10(random_f32()); //@@todo check log exp

        if hit_distance>distance_inside_boundary { return false; }

        hit0.t_min = hit1.t_min + hit_distance/ray_length;
        hit0.pos   = ray.at(hit0.t_min);

        if debugging {

        }

        hit0.norm = vec3(1.0, 0.0, 0.0);
        hit0.front = true;
        hit0.mat_ptr = self.phase_function.clone();

        return true;
    }
}

impl Shape for ConstantMedium {
    fn bounds(&self) -> Bounds3f {
        Bounds3f::default()
    }
}
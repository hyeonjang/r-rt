extern crate rxmath;
use rxmath::*;

struct Ray {
    o : Fvec3,
    d : Fvec3,
}

impl Ray {
    fn new(_o:Fvec3, _d:Fvec3) -> Ray {
        Ray{ o:_o, d:_d }
    }
}

// pub struct Isect {

// }

fn main() {
    let v0 = vec3(1_f32, 1_f32, 1_f32);
    let v1 = v0.length();
    let r : Ray = Ray::new(vec3(0_f32,0_f32,0_f32), vec3(1_f32,1_f32,1_f32));
    println!("main raytracer {}", v1);
}

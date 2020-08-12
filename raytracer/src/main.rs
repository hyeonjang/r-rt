extern crate rxmath;
use rxmath::*;

struct Ray {
    pub o : Fvec3,
    pub d : Fvec3,
}

impl Ray {
    fn new(_o:Fvec3, _d:Fvec3) -> Ray {
        Ray{ o:_o, d:_d }
    }
}

pub struct Isect {

}

fn main() {
    let v0 = vec3(1_f32, 1_f32, 1_f32);
    let mut v1 = v0;
    let v2 = v1 - v0;

    let l0 = v0.length();
    let l2 = v2.length();

    println!("main raytracer {} {}", v0.length(), v2.length());
    println!("print length {} {}", l0, l2);
}

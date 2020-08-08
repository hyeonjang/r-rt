extern crate rxmath;
use rxmath::*;

struct Ray {
    o : rxmath::Vec2,
    d : Vec2,
}

impl Ray {
    fn new(_o:Vec2, _d:Vec2) -> Ray {
        Ray{ o:_o, d:_d }
    }
}

// pub struct Isect {

// }

fn main() {
    let r : Ray = Ray::new(vec2(0,0), vec2(1,1));
    println!("main raytracer{}", r.o.x);
}

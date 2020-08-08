extern crate rxmath;
use rxmath::*;

pub struct Ray<T> {
    o : Vector3<T>,
    d : Vector3<T>,
}

impl Ray {
    fn new(_o:Vector3, _d:Vector3) -> Ray {
        Ray{ o:_o, d:_d }
    }
}

// pub struct Isect {

// }

fn main() {
    let r : Ray = Ray::new(vec3(0,0,0), vec3(1,1,1));
    println!("main raytracer{}", r.o.x);
}

use rxmath::vector::*;
use rxmath::matrix::*;

#[allow(dead_code)]
struct Ray {
    pub o : Fvec3,
    pub d : Fvec3,
}

#[allow(dead_code)]
impl Ray {
    fn new(_o:Fvec3, _d:Fvec3) -> Ray {
        Ray{ o:_o, d:_d }
    }
}

pub struct Isect {

}

fn main() {
    let v0 = vec3(1_f32, 1_f32, 1_f32);
    let m0 : Gmat2<f32> = Gmat2::new(1_f32, 1_f32, 1_f32, 1_f32);

    println!("{:?}", v0);
    println!("{}", m0);
}

use rxmath::vector::*;
use rxmath::matrix::*;

#[derive(Copy, Clone)]
pub struct Ray {
    pub o : Fpoint3,
    pub d : Fvec3,
}

impl Ray {
    pub fn at(self, t:f32) -> Fvec3 {
        return self.o + self.d*t;
    } 
}

pub struct Iact {
    pub t : f32,
    pub pos : Fvec3,
    pub norm : Fvec3,
}

#[allow(dead_code)]
impl Ray {
    fn new(_o:Fvec3, _d:Fvec3) -> Ray {
        Ray{ o:_o, d:_d }
    }
}




use rxmath::vector::*;
use rxmath::matrix::*;

#[derive(Default, Copy, Clone)]
pub struct Ray {
    pub o : Fpoint3,
    pub dir : Fvec3,
}

impl Ray {
    pub fn at(self, t:f32) -> Fvec3 {
        return self.o + self.dir*t;
    } 
}

#[derive(Default)]
pub struct Iact {
    pub pos   : Fvec3,
    pub norm  : Fvec3,
    pub t     : f32,
    pub tfar  : f32,
    pub front : bool,
}

#[allow(dead_code)]
impl Ray {
    fn new(_o:Fvec3, _d:Fvec3) -> Ray {
        Ray{ o:_o, dir:_d }
    }
}

impl Iact {
    #[inline] pub fn set_face_normal(&mut self, r: &Ray, on:&Fvec3) {
        self.front = dot(r.dir, *on)<0f32;
        self.norm  = if self.front { *on } else { -*on }
    }
}






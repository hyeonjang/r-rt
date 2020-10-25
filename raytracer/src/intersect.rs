use rxmath::vector::*;
//use rxmath::matrix::*;

#[derive(Default, Copy, Clone)]
pub struct Ray {
    pub o : vec3,
    pub dir : vec3,
}

impl Ray {
    pub fn at(self, t:f32) -> vec3 {
        return self.o + self.dir*t;
    } 
}

#[derive(Default, Copy, Clone)]
#[allow(non_camel_case_types)]
pub struct hit {
    pub pos   : vec3,
    pub norm  : vec3,
    pub t     : f32,
    pub front : bool,
}

#[allow(dead_code)]
impl Ray {
    fn new(_o:vec3, _d:vec3) -> Ray {
        Ray{ o:_o, dir:_d }
    }
}

impl hit {
    #[inline] pub fn set_face_normal(&mut self, r: &Ray, on:vec3) {
        if dot(r.dir, on) > 0.0 {
            self.norm = - on;
            self.front = false;
        } else {
            self.norm = on;
            self.front = true;
        }
    }
}






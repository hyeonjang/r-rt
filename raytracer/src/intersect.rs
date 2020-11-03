use rxmath::vector::*;
//use rxmath::matrix::*;

#[derive(Default, Copy, Clone)]
pub struct ray {
    pub o : vec3,
    pub dir : vec3,
}

impl ray {
    pub fn at(self, t:f64) -> vec3 {
        return self.o + self.dir*t;
    } 
}

#[derive(Default, Copy, Clone)]
#[allow(non_camel_case_types)]
pub struct hit {
    pub pos   : vec3,
    pub norm  : vec3,
    pub t     : f64,
    pub front : bool,
}

#[allow(dead_code)]
impl ray {
    fn new(_o:vec3, _d:vec3) -> ray {
        ray{ o:_o, dir:_d }
    }
}

impl hit {
    #[inline] pub fn set_face_normal(&mut self, r: &ray, on:vec3) {
        if dot(r.dir, on) > 0.0 {
            self.norm = - on;
            self.front = false;
        } else {
            self.norm = on;
            self.front = true;
        }
    }
}






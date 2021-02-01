use rxmath::vector::*;

#[derive(Default, Copy, Clone)]
pub struct Ray {
    pub o : vec3,
    pub dir : vec3,
    pub tm : f32,
}

impl Ray {
    pub fn at(&self, t:f32) -> vec3 {
        return self.o + self.dir*t;
    } 
    pub fn new(o:vec3, d:vec3, t:Option<f32>) -> Ray {
        match t {
            Some(p) =>  Ray{ o:o, dir:d, tm:t.unwrap() },
            None => Ray{ o:o, dir:d , tm:0.0 },
        }
    }
}
use rxmath::vector::*;

#[derive(Default, Copy, Clone)]
pub struct Ray {
    pub o : vec3,
    pub d : vec3,
    pub t_max : f32,
    pub tm : f32,
}

impl Ray {
    pub fn at(&self, t:f32) -> vec3 {
        return self.o + self.d*t;
    } 
    pub fn new(o:vec3, d:vec3, t:Option<f32>) -> Ray {
        match t {
            Some(_) =>  Ray{ o:o, d:d, t_max:f32::MAX, tm:t.unwrap() },
            None => Ray{ o:o, d:d, t_max:f32::MAX, tm:0.0 },
        }
    }
}
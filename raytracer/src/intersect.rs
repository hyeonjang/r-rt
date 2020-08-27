use rxmath::vector::*;
use rxmath::matrix::*;

#[derive(Copy, Clone)]
pub struct Ray {
    pub o : Fvec3,
    pub d : Fvec3,
}

impl Ray {
    pub fn at(self, t:f32) -> Fvec3 {
        return self.o + self.d*t;
    } 
}

#[allow(dead_code)]
impl Ray {
    fn new(_o:Fvec3, _d:Fvec3) -> Ray {
        Ray{ o:_o, d:_d }
    }
}

pub fn ray_color(r:Ray) -> Fvec3 {
    let t = hit_sphere(vec3(0f32,0f32,-1f32), 0.5, r.clone());

    if t>0f32 {
        let N = r.at(t).normalize() - vec3(0f32, 0f32, -1f32);
        return vec3(N.x+1f32, N.y+1f32, N.z+1f32)*0.5;
    }

    let unit_direction = r.d.normalize();
    let t = 0.5*(unit_direction.y + 1.0);
    return vec3(1.0, 1.0, 1.0)*(1.0-t) + vec3(0.5, 0.7, 1.0)*t;
}

pub fn hit_sphere(center:Fvec3, radius:f32, r:Ray) -> f32 {
    let oc = r.o - center;
    let a = dot(r.d, r.d);
    let b = 2.0 * dot(oc, r.d);
    let c = dot(oc, oc) - radius*radius;
    let discriminat = b*b - 4f32*a*c;
    
    if discriminat < 0f32 {
        return -1.0;
    }
    else{
        return (-b - discriminat.sqrt())/(2.0*a);
    }
}

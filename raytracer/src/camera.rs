use rxmath::vector::*;
use crate::intersect::*;

#[derive(Default, Copy, Clone)]
pub struct Camera {
    origin : vec3,
    lower_left_corner : vec3,
    horizontal : vec3,
    vertical : vec3,
}

impl Camera {
    pub fn new(lookfrom:vec3, lookat:vec3, vup:vec3, vfov:f64, aspect_ratio:f64) -> Self {
        let theta = vfov * 3.14 / 180.0;
        let h = (theta/2.0).tan();
        let viewport_height = 2.0 * h;
        let viewport_width = aspect_ratio * viewport_height;

        let w = normalize(lookfrom-lookat);
        let u = normalize(cross(vup, w));
        let v = cross(w, u);

        let _origin = lookfrom;
        let _horizontal = u*viewport_width;
        let _vertical = v*viewport_height;
        let _lower_left_corner = _origin.clone() - _horizontal.clone()/2.0 - _vertical.clone()/2.0 - w.clone().clone();
   
        Camera{ origin:_origin, horizontal:_horizontal, vertical:_vertical, lower_left_corner:_lower_left_corner, }
    }

    pub fn get_ray(self, s:f64, t:f64) -> ray{
        return ray{o:self.origin, dir:(self.lower_left_corner+self.horizontal*s+self.vertical*t-self.origin)}
    }
}
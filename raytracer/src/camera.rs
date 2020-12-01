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
        let theta = rxmath::degrees_to_radians(vfov);
        let h = f64::tan(theta/2.0);
        let viewport_height = 2.0 * h;
        let viewport_width = aspect_ratio * viewport_height;

        let w = normalize(lookfrom-lookat);
        let u = normalize(cross(w, vup));
        let v = cross(u, w);

        let _origin = lookfrom;
        let _horizontal = u*viewport_width;//u*viewport_width;
        let _vertical = v*viewport_height;//v*viewport_height;
        let _lower_left_corner = _origin - _horizontal/2.0 - _vertical/2.0 - w;
        
        return Camera{ origin:_origin, horizontal:_horizontal, vertical:_vertical, lower_left_corner:_lower_left_corner, }
    }

    pub fn get_ray(self, s:f64, t:f64) -> ray{
        return ray{o:self.origin, dir:(self.lower_left_corner+self.horizontal*s+self.vertical*t-self.origin)}
    }
} 
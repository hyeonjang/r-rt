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
    pub fn new() -> Self {
        let aspect_ratio = 16.0/9.0;
        let viewport_height = 2.0;
        let viewport_width = aspect_ratio * viewport_height;
        let focal_length = 1.0;

        let _origin = vec3(0.0, 0.0, 0.0);
        let _horizontal = vec3(viewport_width, 0.0, 0.0);
        let _vertical = vec3(0.0, viewport_height, 0.0);
        let _lower_left_corner = _origin - _horizontal/2.0 - _vertical/2.0 - vec3(0.0, 0.0, focal_length);
   
        Camera{ origin:_origin, horizontal:_horizontal, vertical:_vertical, lower_left_corner:_lower_left_corner, }
    }

    pub fn get_ray(self, u:f64, v:f64) -> ray{
        return ray{o:self.origin, dir:(self.lower_left_corner+self.horizontal*u+self.vertical*v-self.origin)}
    }
}
use rxmath::vector::*;

use crate::intersect::*;
use crate::sample::*;

#[derive(Default, Copy, Clone)]
pub struct Camera {
    origin : vec3,
    lower_left_corner : vec3,
    horizontal : vec3,
    vertical : vec3,
    u:vec3, v:vec3, w:vec3,
    lens_radius:f64,
}

impl Camera {
    pub fn new(lookfrom:vec3, lookat:vec3, vup:vec3, vfov:f64, aspect_ratio:f64, apeture:f64, focus_dist:f64) -> Self {
        let theta = rxmath::degrees_to_radians(vfov);
        let h = f64::tan(theta/2.0);
        let viewport_height = 2.0 * h;
        let viewport_width = aspect_ratio * viewport_height;

        let w = normalize(lookfrom-lookat);
        let u = normalize(cross(w, vup));
        let v = cross(u, w);

        let origin = lookfrom;
        let horizontal = u*viewport_width*focus_dist;//u*viewport_width;
        let vertical = v*viewport_height*focus_dist;//v*viewport_height;
        let lower_left_corner = origin - horizontal/2.0 - vertical/2.0 - w*focus_dist;
        
        let lens_radius = apeture/2.0;

        return Camera{ origin:origin, horizontal:horizontal, vertical:vertical, lower_left_corner:lower_left_corner, w:w, u:u, v:v, lens_radius:lens_radius}
    }

    pub fn get_ray(&self, s:f64, t:f64) -> ray{
        let rd = random_unit_disk()*self.lens_radius;
        let offset = self.u*rd.x + self.v*rd.y;

        return ray{o:self.origin+offset, dir:(self.lower_left_corner+self.horizontal*s+self.vertical*t-self.origin-offset)}
    }
} 
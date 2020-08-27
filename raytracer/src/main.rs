extern crate image;
extern crate num_complex;

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

fn main() {
    
    const aspect_ratio:f32 = 16.0/9.0;
    let imgx = 400;
    let imgy = (imgx as f32/aspect_ratio) as u32;

    // Create a new ImgBuf with width: imgx and height: imgy
    let mut imgbuf = image::ImageBuffer::new(imgx, imgy);


    let viewport_height = 2.0;
    let viewport_width  = aspect_ratio * viewport_height;
    let focal_length    = 1.0;
    
    let origin      = vec3(0.0, 0.0, 0.0);
    let horizontal  = vec3(viewport_width, 0.0, 0.0);
    let vertical    = vec3(0.0, viewport_height, 0.0);

    let lower_left_corner = origin - horizontal/2f32 - vertical/2f32 -vec3(0.0, 0.0, focal_length);

    // A redundant loop to demonstrate reading image data
    for y in (0..imgy).rev() {
        for x in 0..imgx {
            let u  = x as f32 / imgx as f32;
            let v  = y as f32 / imgy as f32;

            let r : Ray = Ray{ o:origin, d:lower_left_corner + horizontal*u + vertical*v - origin }; 
            let pixel_color = ray_color(r)*255f32;

            let pixel = imgbuf.get_pixel_mut(x, y);
            *pixel = image::Rgb([pixel_color.x as u8, pixel_color.y as u8, pixel_color.z as u8]);
        }
    }

    // Save the image as “fractal.png”, the format is deduced from the path
    imgbuf.save("result.png").unwrap();
}

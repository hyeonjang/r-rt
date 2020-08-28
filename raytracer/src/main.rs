extern crate image;
extern crate num_complex;

pub mod intersect;
pub mod shape;

use intersect::*;
use shape::*;
use rxmath::vector::*;

pub fn ray_color(r:Ray) -> Fvec3 {
    let o = vec3(0f32, 0f32, -1f32);
    let s : Sphere = Sphere{ center:o, radius:0.5 };
    if s.intersect(&r){ return vec3(1.0, 0.0, 0.0); }
    // if t>0f32 {
    //     let N = r.at(t).normalize() - vec3(0f32, 0f32, -1f32);
    //     return vec3(N.x+1f32, N.y+1f32, N.z+1f32)*0.5;
    // }

    let unit_direction = r.d.normalize();
    let t = 0.5*(unit_direction.y + 1.0);
    return vec3(1.0, 1.0, 1.0)*(1.0-t) + vec3(0.5, 0.7, 1.0)*t;
}

fn main() {

    const aspect_ratio:f32 = 16.0/9.0;
    let imgx = 800;
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
    for y in 0..imgy {
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

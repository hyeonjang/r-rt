extern crate image;

pub mod intersect;
pub mod shape;
pub mod camera;
pub mod sample;

use rxmath::*;
use rxmath::vector::*;
use intersect::*;
use shape::*;
use camera::*;
use sample::*;

pub fn ray_color(r:Ray, objects:&ShapeList<Sphere>) -> vec3 {
    let mut i:hit = hit::default();
    let t_min = -1.0f32;
    let t_max = f32::MAX;

    if objects.hit(&r, t_min, t_max, &mut i) { 
        return (i.norm+vec3(1f32, 1f32, 1f32))*0.5;
    }

    let unit_direction = r.dir.normalize();
    let t = 0.5*(unit_direction.y + 1.0);
    return vec3(1.0, 1.0, 1.0)*(1.0-t) + vec3(0.5, 0.7, 1.0)*t;
}

pub fn write_color(pixel_color:vec3, sample_count:i32) -> vec3 {
    let scale = 1.0 / sample_count as f32;
   
    let r = pixel_color.x * scale;
    let g = pixel_color.y * scale;
    let b = pixel_color.z * scale;

    return vec3(clamp(r, 0.0, 1.0), clamp(g, 0.0, 1.0), clamp(b, 0.0, 1.0)) * 256f32;
}


fn main() {

    // Image
    const ASPECT:f32 = 16.0/9.0;
    let imgx = 800;
    let imgy = (imgx as f32/ASPECT) as u32;
    let sample_count = 64;

    // Create a new ImgBuf with width: imgx and height: imgy
    let mut imgbuf = image::ImageBuffer::new(imgx, imgy);

    // World
    let mut world : ShapeList<Sphere> = ShapeList::new();
    world.push( Sphere{center:vec3(0f32, 0f32, -1f32), radius:0.5f32} );
    //world.push( Sphere{center:vec3(0f32, 0f32, -1f32), radius:0.25f32} );

    // Camera
    let cam = Camera::new();
 
    // A redundant loop to demonstrate reading image data
    for y in (0..imgy).rev() {
        for x in 0..imgx {
            let mut pixel_color = vec3(0.0, 0.0, 0.0);
            for i in 0..sample_count {
                let u  = (x as f32 + random_f32())/(imgx-1) as f32;
                let v  = (y as f32 + random_f32())/(imgy-1) as f32;        
                let r : Ray = cam.get_ray(u, v);
                pixel_color += ray_color(r, &mut world);
            }
            //let rgb = pixel_color/64.0;
            let rgb = write_color(pixel_color, sample_count);
            let pixel = imgbuf.get_pixel_mut(x, y);
            *pixel = image::Rgb([rgb.x as u8, rgb.y as u8, rgb.z as u8]);
        }
    }

    // Save the image as “fractal.png”, the format is deduced from the path
    imgbuf.save("result.png").unwrap();
}

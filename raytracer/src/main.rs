extern crate image;

use std::sync::Arc;
use std::thread;
use std::borrow::Borrow;

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

pub fn ray_color(r:ray, objects:&ShapeList<Sphere>, depth:u32) -> vec3 {
    let mut i:hit = hit::default();
    let t_min = 0.001f64;
    let t_max = f64::MAX;

    if depth <= 0 {
        return vec3(0f64, 0f64, 0f64);
    }

    if objects.hit(&r, t_min, t_max, &mut i) { 
        let target:vec3 = i.pos + i.norm + random_unit_sphere().normalize();
        return ray_color(ray{o:i.pos, dir:target-i.pos}, objects, depth-1)*0.5;
    }

    let unit_direction = r.dir.normalize();
    let t = 0.5*(unit_direction.y + 1.0);
    return vec3(1.0, 1.0, 1.0)*(1.0-t) + vec3(0.5, 0.7, 1.0)*t;
}

pub fn write_color(pixel_color:vec3, sample_count:i64) -> vec3 {
    let scale = 1.0 / sample_count as f64;
   
    let r = (pixel_color.x * scale).sqrt();
    let g = (pixel_color.y * scale).sqrt();
    let b = (pixel_color.z * scale).sqrt();

    return vec3(clamp(r, 0.0, 1.0)*256f64, clamp(g, 0.0, 1.0)*256f64, clamp(b, 0.0, 1.0)*256f64);
}


fn main() {

    // Image
    const ASPECT:f64 = 16.0/9.0;
    let imgx = 400;
    let imgy = (imgx as f64/ASPECT) as u32;
    let sample_count = 16;
    const MAX_DEPTH:u32 = 50;

    // Create a new ImgBuf with width: imgx and height: imgy
    let mut imgbuf = image::ImageBuffer::new(imgx, imgy);

    // World
    let mut world = ShapeList::new();
    world.push( Sphere{center:vec3(0f64, 0f64, -1f64), radius:0.5f64} );
    world.push( Sphere{center:vec3(-1.0f64, 0f64, -1f64), radius:0.5f64} );
    world.push( Sphere{center:vec3(0f64, 100.5f64, -1f64), radius:100.0f64} );

    let mut ArcWorld = Arc::new(world);
    let ArcImgbuf = Arc::new(imgbuf);

    // Camera
    let cam = Camera::new();
 
    // A redundant loop to demonstrate reading image data
    let handle = thread::spawn(move || {
        for y in (0..imgy).rev() {
            for x in 0..imgx {
                let mut pixel_color = vec3(0.0, 0.0, 0.0);
                for _i in 0..sample_count {
                    let u  = (x as f64 + random_f64())/(imgx-1) as f64;
                    let v  = (y as f64 + random_f64())/(imgy-1) as f64;        
                    let r : ray = cam.get_ray(u, v);
                    pixel_color += ray_color(r, &mut ArcWorld, MAX_DEPTH);
                }
                let rgb = write_color(pixel_color, sample_count);
                let pixel = ArcImgbuf.get_pixel_mut(x, y);
                *pixel = image::Rgb([rgb.x as u8, rgb.y as u8, rgb.z as u8]);
            }
        }
    });

    // Save the image as “fractal.png”, the format is deduced from the path
    //imgbuf.save("result.png").unwrap();
}

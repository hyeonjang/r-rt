extern crate image;

// std
use std::time::{Instant};
use std::rc::Rc;

// External create
use chrono::{Utc};

// Custom crate
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

// static variables
static NAME:&'static str = "ray-tracer";
pub fn get_date()->String{ return Utc::now().format("%Y-%m-%d").to_string(); }

pub fn ray_color(r:ray, objects:&ShapeList<Sphere>, depth:u32) -> vec3 {
    let mut i:hit = hit::default();
    let t_min = 0.001f64;
    let t_max = f64::MAX;

    if depth <= 0 {
        return vec3(0f64, 0f64, 0f64);
    }

    if objects.hit(&r, t_min, t_max, &mut i) { 
        let mut scattered:ray = ray::default();
        let mut attenuation = vec3(0.0, 0.0, 0.0);
        if i.mat_ptr.scatter(&r, &i, &mut attenuation, &mut scattered) {
            return ray_color(scattered, objects, depth-1)*0.5;
        }
        else {
            return vec3(0f64, 0f64, 0f64);
        }
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

    return vec3(clamp(r, 0.0, 0.999)*256f64, clamp(g, 0.0,0.999)*256f64, clamp(b, 0.0, 0.999)*256f64);
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

    // Material List
    let material_ground = Rc::new(lambertian::new(0.8, 0.8, 0.0)); // why this b r g
    let material_center = Rc::new(lambertian::new(0.7, 0.3, 0.3));
    let material_left   = Rc::new(metal::new(0.8,0.8,0.8));
    let material_right  = Rc::new(metal::new(0.2,0.8,0.6));

    // World
    let mut world = ShapeList::new();
    world.push( Sphere{center:vec3(0f64, 100.5f64, -1f64), radius:100.0f64, mat_ptr:material_ground.clone()} );
    world.push( Sphere{center:vec3(0f64, 0f64, -1f64), radius:0.5f64, mat_ptr:material_center.clone()} );
    world.push( Sphere{center:vec3(-1.0f64, 0f64, -1f64), radius:0.5f64, mat_ptr:material_left.clone()} );
    world.push( Sphere{center:vec3( 1.0f64, 0f64, -1f64), radius:0.5f64, mat_ptr:material_right.clone()} );

    // Camera
    let cam = Camera::new();
 
    // Ray Trace!
    let start = Instant::now();
    for y in (0..imgy).rev() { 
        for x in 0..imgx {
            let mut pixel_color = vec3(0.0, 0.0, 0.0);
            for _i in 0..sample_count {
                let u  = (x as f64 + random_f64())/(imgx-1) as f64;
                let v  = (y as f64 + random_f64())/(imgy-1) as f64;        
                let r : ray = cam.get_ray(u, v);
                pixel_color += ray_color(r, &mut world, MAX_DEPTH);
            }
            let rgb = write_color(pixel_color, sample_count);
            let pixel = imgbuf.get_pixel_mut(x, y);
            *pixel = image::Rgb([rgb.x as u8, rgb.y as u8, rgb.z as u8]);
        }
    }
    let duration = start.elapsed();
    println!("[{}]time duration:{:?}", NAME, duration);

    let current_dir = std::env::current_dir().unwrap().to_str().unwrap().to_owned();
    let path = format!("{}/result/{}.png", current_dir, get_date());
    imgbuf.save(path).unwrap();
}

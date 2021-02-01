#[macro_use] 
extern crate rxmath;
extern crate image;

// std
use std::time::{Instant};
use std::sync::Arc;

// External create
use chrono::{Utc};
use indicatif::{ProgressBar, ProgressStyle};

// Custom crate
use rxmath::vector::*;
use rxmath::random::*;
use rxmath::macros::*;

// Current crate
pub mod intersect;
pub mod camera;
pub mod sample;
//pub mod thread;

mod accelerator;
mod shape;

use self::intersect::*;
use hit::*;
use ray::*;
use self::shape::*;
use self::camera::*;

// static variables
static NAME:&'static str = "ray-tracer";
static STYLE:&'static str = "[{elapsed_precise}] [{bar:40.cyan/blue}] ({eta})";
pub fn get_date()->String{ return Utc::now().format("%Y-%m-%d").to_string(); }

pub fn ray_color(r:&Ray, objects:&ShapeList, depth:u32) -> vec3 {
    let mut i:Hit = Hit::default();

    if depth <= 0 {
        return vec3(0f32, 0f32, 0f32);
    }

    if objects.intersect(&r, &mut i) { 
        let mut scattered:Ray = Ray::default();
        let mut attenuation = vec3(0f32, 0f32, 0f32);
        if i.mat_ptr.scatter(&r, &i, &mut attenuation, &mut scattered) {
            //println!("{}", attenuation);
            return attenuation*ray_color(&scattered, objects, depth-1);
        }
        return vec3(0f32, 0f32, 0f32);
    }

    let unit_direction = normalize(r.dir);
    let t = 0.5*(unit_direction.y + 1.0);
    return vec3(1.0, 1.0, 1.0)*(1.0-t) + vec3(0.5, 0.7, 1.0)*t;
}

pub fn write_color(pixel_color:vec3, sample_count:u64) -> vec3 {
    let scale = 1.0 / sample_count as f32;

    let xyz = pixel_color*scale;
    let rgb = vec3(f32::sqrt(xyz.x), f32::sqrt(xyz.y), f32::sqrt(xyz.z));

    return saturate_vec3(rgb)*256.0;
}

fn main() {

    // Image
    let aspect_ratio:f32 = 3.0/2.0;
    let imgx:u32 = 400;
    let imgy:u32 = (imgx as f32/aspect_ratio) as u32;
    let sample_count:u64 = 4;
    let max_depth:u64= 1;

    // Create a new ImgBuf with width: imgx and height: imgy
    let mut imgbuf = image::ImageBuffer::new(imgx as u32, imgy as u32);

    // Material List

    // World
    let mut world = random_scene(0);
    let acc_start = Instant::now();
    //world.acc_build(accelerator::AcceleratorType::BVH);
    let acc_end = acc_start.elapsed();
    println!("[{}] accelerator building duration:{:?}", NAME, acc_end);

    // Camera
    let lookfrom = vec3(13.0, -2.0, 3.0);
    let lookat = vec3(0.0, 0.0, 0.0);
    let vup = vec3(0.0, 1.0, 0.0);
    let dist_to_focus = 10.0;
    let aperture = 0.1;

    let cam = Camera::new(lookfrom, lookat, vup, 20.0, aspect_ratio, aperture, dist_to_focus, 0.0, 0.1);
    // ** upside downed why

    // Ray Trace!
    let start = Instant::now();
    let pb = ProgressBar::new(((imgx*imgy) as u64)*sample_count);
    pb.set_style(ProgressStyle::default_bar().template(STYLE));
    for y in 0..imgy {
        for x in 0..imgx {
            let mut pixel_color = vec3(0.0, 0.0, 0.0);
            for _ in 0..sample_count {
                pb.inc(1);
                let u  = (x as f32 + random_f32())/(imgx-1) as f32;
                let v  = (y as f32 + random_f32())/(imgy-1) as f32;        
                let r : Ray = cam.get_ray(u, v);
                pixel_color += ray_color(&r, &mut world, max_depth as u32);
            }
            let rgb = write_color(pixel_color, sample_count);
            let pixel = imgbuf.get_pixel_mut(x as u32, y as u32);
            *pixel = image::Rgb([rgb.x as u8, rgb.y as u8, rgb.z as u8]);
        }
    }
    pb.finish();
    let duration = start.elapsed();
    println!("[{}] time duration:{:?}", NAME, duration);

    let current_dir = std::env::current_dir().unwrap().to_str().unwrap().to_owned();
    let path = format!("{}/result/{}.png", current_dir, get_date());
    imgbuf.save(path).unwrap();
}

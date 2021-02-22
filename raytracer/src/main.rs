#[macro_use] 
extern crate rxmath;
extern crate image;

// std
use std::time::{Instant};

// External create
use chrono::{Utc};
use indicatif::{ProgressBar, ProgressStyle};

// Custom crate
use rxmath::vector::*;
use rxmath::random::*;

// Current crate
pub mod intersect;
pub mod camera;
pub mod sample;
pub mod intergrate;
pub mod texture;
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

pub fn ray_color(r:&Ray, background:&vec3, objects:&ShapeList, depth:u32) -> vec3 {
    let mut i:Hit = Hit::default();

    if depth <= 0 {
        return vec3(0f32, 0f32, 0f32);
    }

    if !objects.intersect(r, 0.001, f32::MAX, &mut i) {
        return *background;
    }

    let mut scattered:Ray = Ray::default();
    let mut attenuation = vec3(0f32, 0f32, 0f32);
    let emitted = i.mat_ptr.emitted(i.u, i.v, &i.pos);

    if !i.mat_ptr.scatter(&r, &i, &mut attenuation, &mut scattered) {
        return emitted;
    }

    return emitted + attenuation * ray_color(&mut scattered, background, objects, depth-1);
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
    let mut sample_count:u64 = 4;
    let max_depth:u64= 3;

    // Create a new ImgBuf with width: imgx and height: imgy
    let mut imgbuf = image::ImageBuffer::new(imgx as u32, imgy as u32);

    // Camera
    let mut lookfrom = vec3(13.0, -2.0, 3.0);
    let mut lookat = vec3(0.0, 0.0, 0.0);
    let mut vup = vec3(0.0, 1.0, 0.0);
    let mut vfov = 40.0;
    let mut dist_to_focus = 10.0;
    let mut aperture = 0.1;

    // World
    enum case {
        random, 
        spheres, 
        perlin, 
        light,
    };

    let c : case = case::light;
    let mut background = vec3(0.3, 0.3, 0.3);
    let mut world : ShapeList;

    match c {
        case::random => {
            world = random_scene(0);
        },
        case::spheres => {
            world = two_spheres();
        }
        case::perlin => {
            world = two_perlin_spheres()
        }
        case::light => {
            world = simple_light();
            sample_count = 1;
            background = vec3(0.30, 0.30, 0.30);
            lookfrom = vec3(26.0, -3.0, 6.0);
            lookat = vec3(0.0, -2.0, 0.0);
            vfov = 20.0;
        }
    };

    let cam = Camera::new(lookfrom, lookat, vup, vfov, aspect_ratio, aperture, dist_to_focus, 0.0, 0.1);

    // Ray Trace!
    let start = Instant::now();
    let pb = ProgressBar::new(((imgx*imgy) as u64)*sample_count);
    pb.set_style(ProgressStyle::default_bar().template(STYLE));
    for y in (0..imgy).rev() {
        for x in 0..imgx {
            let mut pixel_color = vec3(0.0, 0.0, 0.0);
            for _ in 0..sample_count {
                pb.inc(1);
                let u  = (x as f32 + random_f32())/(imgx-1) as f32;
                let v  = (y as f32 + random_f32())/(imgy-1) as f32;        
                let r : Ray = cam.get_ray(u, v);
                pixel_color += ray_color(&r, &background, &mut world, max_depth as u32);
            }
            let rgb = write_color(pixel_color, sample_count);
            let pixel = imgbuf.get_pixel_mut(x as u32, y as u32);
            *pixel = image::Rgb([rgb.x as u8, rgb.y as u8, rgb.z as u8]);
        }
    }
    pb.finish();
    let duration = start.elapsed();
    println!("[{}] time duration:{:?}", NAME, duration);

    let current_d = std::env::current_dir().unwrap().to_str().unwrap().to_owned();
    let path = format!("{}/result/{}.png", current_d, get_date());
    imgbuf.save(path).unwrap();
}

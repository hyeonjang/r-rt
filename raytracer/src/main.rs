extern crate image;

// std
use std::time::{Instant};
use std::rc::Rc;

// External create
use chrono::{Utc};
use indicatif::{ProgressBar, ProgressStyle};

// Custom crate
use rxmath::vector::*;

// Current crate
pub mod intersect;
pub mod shape;
pub mod camera;
pub mod sample;

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
        let mut attenuation = vec3(0f64, 0f64, 0f64);
        if i.mat_ptr.scatter(&r, &i, &mut attenuation, &mut scattered) {
            return attenuation*ray_color(scattered, objects, depth-1);
        }
        else {
            return vec3(0f64, 0f64, 0f64);
        }
    }

    let unit_direction = normalize(r.dir);
    let t = 0.5*(unit_direction.y + 1.0);
    return vec3(1.0, 1.0, 1.0)*(1.0-t) + vec3(0.5, 0.7, 1.0)*t;
}

pub fn write_color(pixel_color:vec3, sample_count:i64) -> vec3 {
    let scale = 1.0 / sample_count as f64;

    let xyz = pixel_color*scale;
    let rgb = vec3(f64::sqrt(xyz.x), f64::sqrt(xyz.y), f64::sqrt(xyz.z));

    return saturate_vec3(rgb)*256.0;
}


fn main() {

    // Image
    const ASPECT:f64 = 16.0/9.0;
    let imgx:u64 = 400;
    let imgy:u64 = (imgx as f64/ASPECT) as u64;
    let sample_count = 1;
    const MAX_DEPTH:u64 = 5;

    // Create a new ImgBuf with width: imgx and height: imgy
    let mut imgbuf = image::ImageBuffer::new(imgx as u32, imgy as u32);

    // Material List
    let material_ground = Rc::new(lambertian::new(0.8, 0.8, 0.0));
    let material_center = Rc::new(lambertian::new(0.1, 0.2, 0.5));
    let material_left   = Rc::new(dielectric::new(1.5));
    let material_right  = Rc::new(metal::new(vec3(0.8,0.6,0.2), 0.0));

    // World
    let mut world = ShapeList::new();
    world.push( Sphere{center:vec3(0f64, 100.5f64, -1f64), radius:100.0f64, mat_ptr:material_ground.clone()} );
    world.push( Sphere{center:vec3(0f64, 0f64, -1f64), radius:0.5f64, mat_ptr:material_center.clone()} );
    world.push( Sphere{center:vec3(-1.0f64, 0f64, -1f64), radius:0.5f64, mat_ptr:material_left.clone()} );
    world.push( Sphere{center:vec3(-1.0f64, 0f64, -1f64), radius:-0.4f64, mat_ptr:material_left.clone()} );
    world.push( Sphere{center:vec3( 1.0f64, 0f64, -1f64), radius:0.5f64, mat_ptr:material_right.clone()} );

    // Camera
    let cam = Camera::new(vec3(-2.0, 2.0, 1.0), vec3(0.0, 0.0, -1.0), vec3(0.0, 1.0, 0.0), 90.0, ASPECT);
 
    // Ray Trace!
    let start = Instant::now();
    let pb = ProgressBar::new(imgy);
    pb.set_style(ProgressStyle::default_bar()
    .template("{spinner:.green} [{elapsed_precise}] [{bar:40.cyan/blue}] ({eta})")
    .progress_chars("#>-"));
    for y in 0..imgy {
        pb.inc(1);
        for x in 0..imgx {
            let mut pixel_color = vec3(0.0, 0.0, 0.0);
            for _ in 0..sample_count {
                let u  = (x as f64 + random_f64())/(imgx-1) as f64;
                let v  = (y as f64 + random_f64())/(imgy-1) as f64;        
                let r : ray = cam.get_ray(u, v);
                pixel_color += ray_color(r, &mut world, MAX_DEPTH as u32);
            }
            let rgb = write_color(pixel_color, sample_count);
            let pixel = imgbuf.get_pixel_mut(x as u32, y as u32);
            *pixel = image::Rgb([rgb.x as u8, rgb.y as u8, rgb.z as u8]);
        }
    }
    pb.finish_and_clear();
    let duration = start.elapsed();
    println!("[{}]Time duration:{:?}", NAME, duration);

    let current_dir = std::env::current_dir().unwrap().to_str().unwrap().to_owned();
    let path = format!("{}/result/{}.png", current_dir, get_date());
    imgbuf.save(path).unwrap();
}

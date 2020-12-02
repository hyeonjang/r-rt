extern crate image;

// std
use std::time::{Instant};
use std::rc::Rc;

// External create
use chrono::{Utc};
use indicatif::{ProgressBar, ProgressStyle};

// Custom crate
use rxmath::vector::*;
use rxmath::random::*;

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
static STYLE:&'static str = "{spinner:.green} [{elapsed_precise}] [{bar:40.cyan/blue}] ({eta})";
pub fn get_date()->String{ return Utc::now().format("%Y-%m-%d").to_string(); }

pub fn ray_color(r:&ray, objects:&ShapeList<Sphere>, depth:u32) -> vec3 {
    let mut i:hit = hit::default();

    if depth <= 0 {
        return vec3(0f64, 0f64, 0f64);
    }

    if objects.hit(&r, 0.001, f64::MAX, &mut i) { 
        let mut scattered:ray = ray::default();
        let mut attenuation = vec3(0f64, 0f64, 0f64);
        if i.mat_ptr.scatter(&r, &i, &mut attenuation, &mut scattered) {
            //println!("{}", attenuation);
            return attenuation*ray_color(&scattered, objects, depth-1);
        }
        return vec3(0f64, 0f64, 0f64);
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

pub fn random_scene() -> ShapeList<Sphere> {
    let mut world = ShapeList::new();

    let ground_material = Rc::new(lambertian::new(0.5, 0.5, 0.5));
    world.push( Sphere{center:vec3(0f64, 1000f64, 0f64), radius:1000.0f64, mat_ptr:ground_material} );

    for a in -11..11 {
        for b in -11..11 {
            let choose_mat = random_f64();
            let center = vec3(a as f64 + 0.9*random_f64(), 0.2, b as f64 + 0.9*random_f64());
        
            if (center - vec3(4.0, 0.2, 0.0)).length() > 0.9 {
                let sphere_material:Rc<dyn material>;

                if choose_mat < 0.8 {
                    // diffuse
                    let albedo = vec3::random() * vec3::random();
                }

            }
        }
    }

    return world;
}

fn main() {

    // Image
    const ASPECT:f64 = 16.0/9.0;
    let imgx = 400;
    let imgy = (imgx as f64/ASPECT) as u32;
    let sample_count = 64;
    const MAX_DEPTH:u32= 32;

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
    let lookfrom = vec3(3.0, -3.0, 2.0);
    let lookat = vec3(0.0, 0.0, -1.0);
    let vup = vec3(0.0, 1.0, 0.0);
    let dist_to_focus = (lookfrom-lookat).length();
    let aperture = 2.0;

    let cam = Camera::new(lookfrom, lookat, vup, 20.0, ASPECT, aperture, dist_to_focus);
    // ** upside downed why

    // Ray Trace!
    let start = Instant::now();
    let pb = ProgressBar::new(imgy as u64);
    pb.set_style(ProgressStyle::default_bar().template(STYLE));
    for y in 0..imgy {
        pb.inc(1);
        for x in 0..imgx {
            let mut pixel_color = vec3(0.0, 0.0, 0.0);
            for _ in 0..sample_count {
                let u  = (x as f64 + random_f64())/(imgx-1) as f64;
                let v  = (y as f64 + random_f64())/(imgy-1) as f64;        
                let r : ray = cam.get_ray(u, v);
                pixel_color += ray_color(&r, &mut world, MAX_DEPTH as u32);
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

extern crate image;

pub mod intersect;
pub mod shape;
pub mod camera;

use rxmath::vector::*;
use intersect::*;
use shape::*;
use camera::*;

pub fn hit_sphere(center:vec3, radius:f32, r:Ray) -> f32{
    let oc = r.o - center;
    let a = dot(r.dir, r.dir);
    let b = 2.0 * dot(oc, r.dir);
    let c = dot(oc, oc) - radius*radius;
    let disc = b*b - 4.0*a*c;
    if disc < 0.0 {
        return -1.0;
    } else {
        return (-b-sqrt(disc))/(2.0*a);
    }
}


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

fn main() {

    // Image
    const ASPECT:f32 = 16.0/9.0;
    let imgx = 800;
    let imgy = (imgx as f32/ASPECT) as u32;

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
            let u  = x as f32 / (imgx-1) as f32;
            let v  = y as f32 / (imgy-1) as f32;

            let r : Ray = cam.get_ray(u, v);
            let pixel_color = ray_color(r, &mut world)*255f32;

            let pixel = imgbuf.get_pixel_mut(x, y);
            *pixel = image::Rgb([pixel_color.x as u8, pixel_color.y as u8, pixel_color.z as u8]);
        }
    }

    // Save the image as “fractal.png”, the format is deduced from the path
    imgbuf.save("result.png").unwrap();
}

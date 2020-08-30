extern crate image;

pub mod intersect;
pub mod shape;

use intersect::*;
use shape::*;
use rxmath::vector::*;

pub fn ray_color(r:Ray, objects:&ShapeList<Sphere>) -> Fvec3 {
    let o = vec3(0f32, 0f32, -1f32);
    let mut i : Iact = Iact::default();
    
    if objects.hit(&r, &mut i) { 
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
    world.push( Sphere{center:vec3(0f32, 0f32, -1f32), radius:0.5} );
    world.push( Sphere{center:vec3(0f32, 1f32, -1f32), radius:0.5} );

    // Camera

    let viewport_height = 2.0;
    let viewport_width  = ASPECT * viewport_height;
    let focal_length    = 1.0;
    
    let origin      = vec3(0.0, 0.0, 0.0);
    let horizontal  = vec3(viewport_width, 0.0, 0.0);
    let vertical    = vec3(0.0, viewport_height, 0.0);

    let lower_left_corner = origin - horizontal/2f32 - vertical/2f32 -vec3(0.0, 0.0, focal_length);

    // A redundant loop to demonstrate reading image data
    for y in (0..imgy).rev() {
        for x in 0..imgx {
            let u  = x as f32 / (imgx-1) as f32;
            let v  = y as f32 / (imgy-1) as f32;

            let r : Ray = Ray{ o:origin, dir:lower_left_corner + horizontal*u + vertical*v - origin }; 
            let pixel_color = ray_color(r, &mut world)*256f32;

            let pixel = imgbuf.get_pixel_mut(x, y);
            *pixel = image::Rgb([pixel_color.x as u8, pixel_color.y as u8, pixel_color.z as u8]);
        }
    }

    // Save the image as “fractal.png”, the format is deduced from the path
    imgbuf.save("result.png").unwrap();
}

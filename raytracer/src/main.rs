extern crate image;
extern crate num_complex;

pub mod intersect;

fn main() {

    use intersect::*;
    use rxmath::vector::*;
    
    const aspect_ratio:f32 = 16.0/9.0;
    let imgx = 400;
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
    for y in (0..imgy).rev() {
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

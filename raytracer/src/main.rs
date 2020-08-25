use rxmath::vector::*;
use rxmath::matrix::*;

#[allow(dead_code)]
struct Ray {
    pub o : Fvec3,
    pub d : Fvec3,
}

#[allow(dead_code)]
impl Ray {
    fn new(_o:Fvec3, _d:Fvec3) -> Ray {
        Ray{ o:_o, d:_d }
    }
}

pub struct Isect {

}

fn main() {

    // Image
    const IMAGE_WIDTH : u32 = 256;
    const IMAGE_HEIGHT : u32 = 256;


    println!("{}x{}", IMAGE_HEIGHT, IMAGE_WIDTH);

    // Render
    for i in 0..IMAGE_HEIGHT {
        for j in 0..IMAGE_WIDTH {
            let r  = (i as f32)/((IMAGE_WIDTH-1) as f32);
            let g  = (j as f32)/((IMAGE_HEIGHT-1) as f32); 
            let b  = 0.25; 

            let ir : u32 = (255.999*r) as u32;
            let ig : u32 = (255.999*g) as u32;
            let ib : u32 = (255.999*b) as u32;

            println!("{} {} {}", ir, ig, ib);
        }
    }
}

use std::sync::*;

use rxmath::vector::*;
use rxmath::random::*;

pub trait Texture {
    fn value(&self, u:f32, v:f32, p:&vec3) -> vec3;
}

pub struct SolidColor {
    color_value:vec3,
}

impl SolidColor {
    pub fn new(v:vec3) -> Self {
        SolidColor { color_value:v }
    }
}

pub struct Checker {
    odd : Arc<dyn Texture>,
    even: Arc<dyn Texture>,
}

impl Checker {
    pub fn new(even:Arc<dyn Texture>, odd:Arc<dyn Texture>) -> Self {
        Checker { even:even, odd:odd }
    }
}

pub struct Noise {
    noise : Perlin,
    scale : f32,
}

impl Noise {
    pub fn new(scale:f32) -> Self {
        Noise { noise:Perlin::new(), scale:scale }
    }
}

impl Texture for Noise {
    fn value(&self, _u: f32, _v: f32, p:&vec3) -> vec3 {
        return vec3(1.0,1.0,1.0)*self.noise.noise(p);
    }
}

impl Texture for SolidColor {
    fn value(&self, _u: f32, _v: f32, _p:&vec3) -> vec3 {
        return self.color_value;
    }
}

impl Texture for Checker {
    fn value(&self, u: f32, v: f32, p:&vec3) -> vec3 {
        let sines = f32::sin(10.0*p.x)*f32::sin(10.0*p.y)*f32::sin(10.0*p.z);
        if sines < 0.0 { return self.odd.value(u, v, p) } 
        else           { return self.even.value(u, v, p) }
    }
}

struct Perlin {
    ranfloat : Vec<f32>,
    perm_x   : Vec<i32>,
    perm_y   : Vec<i32>,
    perm_z   : Vec<i32>,
}

impl Perlin {
    fn point_count() -> usize {
        256
    }

    fn perlin_generate_perm() -> Vec<i32> {
        let mut p = vec![0 as i32; Perlin::point_count()];
        for i in 0..Perlin::point_count() {
            p[i] = i as i32;
        }
        Perlin::permute(&mut p, Perlin::point_count());
        return p;
    }

    fn permute(p:&mut Vec::<i32>, n:usize) {
        for i in (1..(n-1)).rev() {
            let target = random_i32(0, i as i32);
            let tmp    = p[i as usize];
            p[i as usize] = p[target as usize];
            p[target as usize] = tmp;
        }
    }

    fn trilinear_interp(c:[[[f32;2];2];2], u:f32, v:f32, w:f32) -> f32 {
        let mut accum = 0.0;
        for i in 0..2 {
            for j in 0..2 {
                for k in 0..2 {
                    accum += ((i as f32*u)+((1-i) as f32)*(1 as f32-u))*
                             ((j as f32*v)+((1-j) as f32)*(1 as f32-v))*
                             ((k as f32*w)+((1-k) as f32)*(1 as f32-w))*
                             c[i][j][k]
                }
            }
        }

        return accum;
    }

    pub fn new() -> Self {
        let point_count = Perlin::point_count();
        let mut ranfloat = vec![0.0; Perlin::point_count()];
        for i in 0..point_count {
            ranfloat[i as usize] = random_f32();
        }
        let perm_x = Perlin::perlin_generate_perm();
        let perm_y = Perlin::perlin_generate_perm();
        let perm_z = Perlin::perlin_generate_perm();

        Perlin { ranfloat:ranfloat, perm_x:perm_x, perm_y:perm_y, perm_z:perm_z }
    }

    pub fn noise(&self, p:&vec3) -> f32 {
        // let mut u = p.x - f32::floor(p.x);
        // let mut v = p.y - f32::floor(p.y);
        // let mut w = p.z - f32::floor(p.z);

        // // u = u*u*(3.0-2.0*u);
        // // v = v*v*(3.0-2.0*v);
        // // w = w*w*(3.0-2.0*w);

        // let i = f32::floor(p.x) as usize;
        // let j = f32::floor(p.y) as usize;
        // let k = f32::floor(p.z) as usize;
        // let mut c = [[[0.0;2];2];2];

        // for di in 0..2 {
        //     for dj in 0..2 {
        //         for dk in 0..2 {
        //             c[di][dj][dk] = self.ranfloat[
        //                (self.perm_x[(i+di)&255]^
        //                 self.perm_y[(j+dj)&255]^
        //                 self.perm_z[(k+dk)&255]) as usize
        //             ]
        //         }
        //     }
        // }

        // return Perlin::trilinear_interp(c, u, v, w);
    
        let i = ((4.0*p.x) as i32&255) as usize;
        let j = ((4.0*p.y) as i32&255) as usize;
        let k = ((4.0*p.z) as i32&255) as usize;

        return self.ranfloat[(self.perm_x[i]^self.perm_y[j]^self.perm_z[k]) as usize];

    
    } 


}
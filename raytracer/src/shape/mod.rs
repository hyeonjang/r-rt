pub mod material;
pub mod sphere;
pub mod triangle;

use crate::intersect::*;
use ray::*;
use hit::*;

use crate::accelerator::*;
use bounds::*;

// traits for all kind of Shapes
pub trait Shape : Intersect {
    fn bounds( &self ) -> Bounds3f;
}

// ShapeList
pub struct ShapeList {
    pub list : Vec<Box<dyn Shape>>,
    pub acc : Option<Box<dyn Accelerator>>,
}

impl ShapeList {
    pub fn new() -> Self {
        ShapeList{ list:vec![], acc:None }
    }
    pub fn clear(&mut self) { 
        self.list.clear();
    }
    pub fn push<T>(&mut self, shape:T) where T:Shape +'static {
        self.list.push(Box::new(shape))
    }
    pub fn acc_build(&mut self, ty:AcceleratorType) {
        let acc = match ty {
            AcceleratorType::BVH => bvh::Create(self, &self.list),
        };
        self.acc = Some(acc);
    }
}

impl Intersect for ShapeList {
    fn intersect(&self, r:&Ray, h:&mut Hit) -> bool {
        match &self.acc {
            Some(s) => { self.acc.as_ref().unwrap().intersect(r, h) },
            None => {
                let mut hit = false;
                for object in &self.list {
                    let mut i = Hit::default();
                    if object.intersect(r, &mut i) && i.t_min<h.t_min {
                        hit = true;
                        *h = i.clone();
                    }
                }
                return hit;
            }
        }
    }
}


////////////////
/// example scene

use std::sync::*;
use rxmath::vector::*;
use rxmath::random::*;
use sphere::*;
use material::*;

use crate::texture::*;

pub fn random_scene(count:i8) -> ShapeList {
    let mut world = ShapeList::new();

    let checker = Arc::new(
        Checker::new(
            Arc::new(SolidColor::new(vec3(0.2,0.3,0.1))),
            Arc::new(SolidColor::new(vec3(0.9,0.9,0.9))),
        ));
    world.push(Sphere{center:vec3(0f32, 1000f32, 0f32), radius:1000.0f32, mat_ptr:Arc::new(lambertian{ albedo:checker })});
    //let ground_material = Arc::new(lambertian::new(vec3(0.5, 0.5, 0.5)));

    let material1 = Arc::new(dielectric::new(1.5));
    world.push(Sphere::new(vec3(0.0, -1.0, 0.0), 1.0, material1));

    let material2 = Arc::new(lambertian::new(vec3(0.4, 0.2, 0.1)));
    world.push(Sphere::new(vec3(-4.0, -1.0, 0.1), 1.0, material2));

    let material3 = Arc::new(metal::new(vec3(0.7, 0.6, 0.5), 0.0));
    world.push(Sphere::new(vec3( 4.0, -1.0, 0.0), 1.0, material3));

    for a in -count..count {
        for b in -count..count {
            let choose_mat = random_f32();
            let center = vec3(a as f32 + 0.9*random_f32(), -0.2, b as f32 + 0.9*random_f32());
        
            if (center - vec3(4.0, 0.2, 0.0)).length() > 0.9 {
                let sphere_material:Arc<dyn Material>;

                if choose_mat < 0.8 { 
                    // diffuse
                    let albedo = vec3::random() * vec3::random();
                    sphere_material = Arc::new(lambertian::new(albedo));
                    world.push(Sphere::new(center, 0.2, sphere_material));
                }
                else if choose_mat < 0.95 {
                    let albedo = vec3::random_range(0.5, 1.0);
                    let fuzz = random_range_f32(0.0, 0.5);
                    sphere_material = Arc::new(metal::new(albedo, fuzz));
                    world.push(Sphere::new(center, 0.2, sphere_material));
                }
                else {
                    sphere_material = Arc::new(dielectric::new(1.5));
                    world.push(Sphere::new(center, 0.2, sphere_material));
                }

            }
        }
    }
    return world;
}

pub fn two_spheres() -> ShapeList {
    let mut world = ShapeList::new();
    let checker = Arc::new(Checker::new(
        Arc::new(SolidColor::new(vec3(0.2, 0.3, 0.1))), 
        Arc::new(SolidColor::new(vec3(0.9, 0.9, 0.9))),
            ));
    
    world.push(Sphere::new(vec3(0.0,-10.0,0.0), 10.0, Arc::new(lambertian { albedo:checker.clone() })));
    world.push(Sphere::new(vec3(0.0, 10.0,0.0), 10.0, Arc::new(lambertian { albedo:checker.clone() })));

    return world;
}

pub fn two_perlin_spheres() -> ShapeList {
    let mut world = ShapeList::new();
    
    let pertext = Arc::new(Noise::new(4.0));
    world.push(Sphere::new(vec3(0.0,1000.0,0.0), 1000.0, Arc::new(lambertian { albedo:pertext.clone() })));
    world.push(Sphere::new(vec3(0.0,-2.0,0.0), 2.0, Arc::new(lambertian { albedo:pertext.clone() })));

    return world;
}


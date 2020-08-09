#![crate_type="lib"]

extern crate num_traits;
use std::ops;

///////////////////////////////
// 0. Vector Structures 
pub struct Tvec2<T> { 
    pub x : T, 
    pub y : T,
}
pub struct Tvec3<T> {
    pub x : T,
    pub y : T, 
    pub z : T,
}
pub struct Tvec4<T> {
    pub x : T, 
    pub y : T,
    pub z : T,
    pub w : T,
}

// 0.1.0 Contructors & Copy & Clone
impl<T> Tvec2<T> {
    pub fn new(_x:T, _y:T) -> Tvec2<T> {
        Tvec2{ x:_x, y:_y } 
    }
}
impl<T> Tvec3<T> {
    pub fn new(_x:T, _y:T, _z:T) -> Tvec3<T> {
        Tvec3{ x:_x, y:_y, z:_z } 
    }
}
impl<T> Tvec4<T> {
    pub fn new(_x:T, _y:T, _z:T, _w:T) -> Tvec4<T> {
        Tvec4{ x:_x, y:_y, z:_z, w:_w }
    }
}

// copy, clone features
impl<T: Copy> Copy for Tvec2<T> {}
impl<T: Clone> Clone for Tvec2<T>{ 
    fn clone(&self)->Self { 
       Tvec2 { x:self.x.clone(), y:self.y.clone() }
    } 
}
impl<T: Copy> Copy for Tvec3<T> {}
impl<T: Clone> Clone for Tvec3<T>{ 
    fn clone(&self)->Self { 
       Tvec3 { x:self.x.clone(), y:self.y.clone(), z:self.z.clone() }
    } 
}
impl<T: Copy> Copy for Tvec4<T> {}
impl<T: Clone> Clone for Tvec4<T>{ 
    fn clone(&self)->Self { 
       Tvec4 { x:self.x.clone(), y:self.y.clone(), z:self.z.clone(), w:self.w.clone() }
    } 
}

//0.1.1 Function Contructors for Convenience
pub fn vec2<T>(_x:T, _y:T) -> Tvec2<T> {
    Tvec2{ x:_x, y:_y }
}
pub fn vec3<T>(_x:T, _y:T, _z:T) -> Tvec3<T> {
    Tvec3{ x:_x, y:_y, z:_z }
}
pub fn vec4<T>(_x:T, _y:T, _z:T, _w:T) -> Tvec4<T> {
    Tvec4{ x:_x, y:_y, z:_z, w:_w }
}

// 1. Operator overloading
// 1.1 tvec2
impl<T: ops::Add<Output= T>> ops::Add<Tvec2<T>> for Tvec2<T> {
    type Output = Tvec2<T>;
    fn add(self, _rhs: Tvec2<T>) -> Tvec2<T> {
        Tvec2{ x:self.x+_rhs.x, y:self.y+_rhs.y }
    }
}
impl<T: ops::Sub<Output=T>> ops::Sub<Tvec2<T>> for Tvec2<T> {
    type Output = Tvec2<T>;
    fn sub(self, _rhs: Tvec2<T>) -> Tvec2<T> {
        Tvec2{ x:self.x-_rhs.x, y:self.y-_rhs.y }
    }
}
impl<T: ops::Neg<Output=T>> ops::Neg for Tvec2<T> {
    type Output = Tvec2<T>;
    fn neg(self) -> Tvec2<T> {
        Tvec2{ x:-self.x, y:-self.y }
    }
}
impl<T: Copy+ops::Add<Output=T>> ops::AddAssign for Tvec2<T> {
    fn add_assign(&mut self, other:Self) {
        *self = Self { x:self.x+other.x, y:self.y+other.y }
    }
}
impl<T: Copy+ops::Sub<Output=T>> ops::SubAssign for Tvec2<T> {
    fn sub_assign(&mut self, other:Self) {
        *self = Self { x:self.x-other.x, y:self.y-other.y }
    }
}

// 1.2 Tvec3
impl<T: ops::Add<Output= T>> ops::Add<Tvec3<T>> for Tvec3<T> {
    type Output = Tvec3<T>;
    fn add(self, _rhs: Tvec3<T>) -> Tvec3<T> {
        Tvec3{ x:self.x+_rhs.x, y:self.y+_rhs.y, z:self.z+_rhs.z }
    }
}
impl<T: ops::Sub<Output=T>> ops::Sub<Tvec3<T>> for Tvec3<T> {
    type Output = Tvec3<T>;
    fn sub(self, _rhs: Tvec3<T>) -> Tvec3<T> {
        Tvec3{ x:self.x-_rhs.x, y:self.y-_rhs.y, z:self.z-_rhs.z }
    }
}
impl<T: ops::Neg<Output=T>> ops::Neg for Tvec3<T> {
    type Output = Tvec3<T>;
    fn neg(self) -> Tvec3<T> {
        Tvec3{ x:-self.x, y:-self.y, z:-self.z }
    }
}
impl<T: Copy+ops::Add<Output=T>> ops::AddAssign for Tvec3<T> {
    fn add_assign(&mut self, other:Self) {
        *self = Self { x:self.x+other.x, y:self.y+other.y, z:self.z+other.z }
    }
}
impl<T: Copy+ops::Sub<Output=T>> ops::SubAssign for Tvec3<T> {
    fn sub_assign(&mut self, other:Self) {
        *self = Self { x:self.x-other.x, y:self.y-other.y, z:self.z-other.z }
    }
}

// 1.3 Tvec4
impl<T: ops::Add<Output= T>> ops::Add<Tvec4<T>> for Tvec4<T> {
    type Output = Tvec4<T>;
    fn add(self, _rhs: Tvec4<T>) -> Tvec4<T> {
        Tvec4{ x:self.x+_rhs.x, y:self.y+_rhs.y, z:self.z+_rhs.z, w:self.w+_rhs.w }
    }
}
impl<T: ops::Sub<Output=T>> ops::Sub<Tvec4<T>> for Tvec4<T> {
    type Output = Tvec4<T>;
    fn sub(self, _rhs: Tvec4<T>) -> Tvec4<T> {
        Tvec4{ x:self.x-_rhs.x, y:self.y-_rhs.y, z:self.z-_rhs.z, w:self.w-_rhs.w }
    }
}
impl<T: ops::Neg<Output=T>> ops::Neg for Tvec4<T> {
    type Output = Tvec4<T>;
    fn neg(self) -> Tvec4<T> {
        Tvec4{ x:-self.x, y:-self.y, z:-self.z, w:-self.w}
    }
}
// 2. Methods for Vectors
// 2.1 Tvec2
impl<Float: num_traits::Float+ops::Add<Output=Float>+ops::Mul<Output=Float>+ops::Div<Output=Float>> Tvec2<Float> {
    // norm/length/dot: floating-point only functions
    pub fn norm2(self) -> Float {
        self.x.mul_add(self.x, self.y.clone()* self.y)
    }
    pub fn norm(self) -> Float {
        self.norm2().sqrt()
    }
    pub fn length2(&self) -> Float {
        self.x.mul_add(self.x, self.y.clone()* self.y)
    }
    pub fn length(&self) -> Float {
        self.length2().sqrt()
    }
    pub fn dot(self,_v:Tvec2<Float>) -> Float {
        self.x.mul_add(_v.x, self.y * _v.y)/self.length()
    }
}
//2.2 Tvec3
impl<Float: num_traits::Float + std::clone::Clone+ops::Add<Output=Float>+ops::Mul<Output=Float>+ops::Div<Output=Float>> Tvec3<Float> {
    // norm/length/dot: floating-point only functions
    pub fn norm2(self) -> Float {
        self.x.mul_add(self.x, self.y.mul_add(self.y, self.z.clone()*self.z))
    }
    pub fn norm(self) -> Float {
        self.norm2().sqrt()
    }
    pub fn length2(&self) -> Float {
        self.x.mul_add(self.x, self.y.mul_add(self.y, self.z.clone()*self.z))
    }
    pub fn length(&self) -> Float {
        self.length2().sqrt()
    }
    pub fn dot(self,_v:Tvec3<Float>) -> Float {
        self.x.mul_add(_v.x, self.y.mul_add(_v.y, _v.z*self.z))/self.length()
    }
}
//2.2 Tvec4
impl<Float: num_traits::Float + std::clone::Clone+ops::Add<Output=Float>+ops::Mul<Output=Float>+ops::Div<Output=Float>> Tvec4<Float> {
    // norm/length/dot: floating-point only functions
    pub fn norm2(self) -> Float {
        self.x.mul_add(self.x, self.y.mul_add(self.y, self.z.mul_add(self.z, self.w.clone()*self.z)))
    }
    pub fn norm(self) -> Float {
        self.norm2().sqrt()
    }
    pub fn length2(&self) -> Float {
        self.x.mul_add(self.x, self.y.mul_add(self.y, self.z.mul_add(self.z, self.w.clone()*self.z)))
    }
    pub fn length(&self) -> Float {
        self.length2().sqrt()
    }
    pub fn dot(self,_v:Tvec4<Float>) -> Float {
        self.x.mul_add(_v.x, self.y.mul_add(self.y, _v.z.mul_add(self.z, _v.w*self.z)))/self.length()
    }
}

pub type Fvec2 = Tvec2<f32>;
pub type Ivec2 = Tvec2<i32>;
pub type Uvec2 = Tvec2<u32>;

pub type Fvec3 = Tvec3<f32>;
pub type Ivec3 = Tvec3<i32>;
pub type Uvec3 = Tvec3<u32>;

pub type Fvec4 = Tvec4<f32>;
pub type Ivec4 = Tvec4<i32>;
pub type Uvec4 = Tvec4<u32>;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {

        let vec0 = Tvec2::new(2_f32, 2_f32);
        let vec1 = Tvec2::new(3_f32, 3_f32);
        let vec2 = vec0 + vec1;
        //let vec1 = tvec2::new(3, 3);
        //let vec2 = tvec2::new(5, 5);
        let vec_norm = vec2.norm2();
        assert_eq!(vec_norm, 50_f32);
    }
}
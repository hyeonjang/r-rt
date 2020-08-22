extern crate libc;
use std::ops;
use std::cmp;
///////////////////////////////
/// 0. tupled array sturctures 
pub struct Garr2<T>(pub T, pub T);
pub struct Garr3<T>(pub T, pub T, pub T);
pub struct Garr4<T>(pub T, pub T, pub T);
///////////////////////////////
// 0. Vector Structures
#[repr(C)]
pub struct Gvec2<T> { pub x:T, pub y:T }
pub struct Gvec3<T> { pub x:T, pub y:T, pub z : T }
pub struct Gvec4<T> { pub x:T, pub y:T, pub z : T, pub w:T }
//////////////////////////////
/// Operation trait functions
// 0.1.0 Contructors & Copy & Clone
impl<T> Gvec2<T> {
    pub fn new(_x:T, _y:T) -> Gvec2<T> {
        Gvec2{ x:_x, y:_y } 
    }
}
impl<T> Gvec3<T> {
    pub fn new(_x:T, _y:T, _z:T) -> Gvec3<T> {
        Gvec3{ x:_x, y:_y, z:_z } 
    }
}
impl<T> Gvec4<T> {
    pub fn new(_x:T, _y:T, _z:T, _w:T) -> Gvec4<T> {
        Gvec4{ x:_x, y:_y, z:_z, w:_w }
    }
}
// copy, clone features
impl<T: Copy> Copy for Gvec2<T> {}
impl<T: Clone> Clone for Gvec2<T>{ 
    fn clone(&self)->Self { 
            Gvec2 { x:self.x.clone(), y:self.y.clone() }
    }
}   
impl<T: Copy> Copy for Gvec3<T> {}
impl<T: Clone> Clone for Gvec3<T>{ 
    fn clone(&self)->Self { 
       Gvec3 { x:self.x.clone(), y:self.y.clone(), z:self.z.clone() }
    } 
}
impl<T: Copy> Copy for Gvec4<T> {}
impl<T: Clone> Clone for Gvec4<T>{ 
    fn clone(&self)->Self { 
       Gvec4 { x:self.x.clone(), y:self.y.clone(), z:self.z.clone(), w:self.w.clone() }
    } 
}
//0.1.1 Function Contructors for Convenience
pub fn vec2<T>(_x:T, _y:T) -> Gvec2<T> {
    Gvec2{ x:_x, y:_y }
}
pub fn vec3<T>(_x:T, _y:T, _z:T) -> Gvec3<T> {
    Gvec3{ x:_x, y:_y, z:_z }
}
pub fn vec4<T>(_x:T, _y:T, _z:T, _w:T) -> Gvec4<T> {
    Gvec4{ x:_x, y:_y, z:_z, w:_w }
}
///////////////////////////
// 1. Operator overloading
// 1.1 Gvec2
impl<T: ops::Add<Output= T>> ops::Add<Gvec2<T>> for Gvec2<T> {
    type Output = Gvec2<T>;
    fn add(self, _rhs: Gvec2<T>) -> Gvec2<T> {
        Gvec2{ x:self.x+_rhs.x, y:self.y+_rhs.y }
    }
}
impl<T: ops::Sub<Output=T>> ops::Sub<Gvec2<T>> for Gvec2<T> {
    type Output = Gvec2<T>;
    fn sub(self, _rhs: Gvec2<T>) -> Gvec2<T> {
        Gvec2{ x:self.x-_rhs.x, y:self.y-_rhs.y }
    }
}
impl<T: ops::Neg<Output=T>> ops::Neg for Gvec2<T> {
    type Output = Gvec2<T>;
    fn neg(self) -> Gvec2<T> {
        Gvec2{ x:-self.x, y:-self.y }
    }
}
impl<T: Copy+ops::Add<Output=T>> ops::AddAssign for Gvec2<T> {
    fn add_assign(&mut self, other:Self) {
        *self = Self { x:self.x+other.x, y:self.y+other.y }
    }
}
impl<T: Copy+ops::Sub<Output=T>> ops::SubAssign for Gvec2<T> {
    fn sub_assign(&mut self, other:Self) {
        *self = Self { x:self.x-other.x, y:self.y-other.y }
    }
}
impl<T: cmp::PartialEq> cmp::PartialEq for Gvec2<T> {
    fn eq(&self, other:&Self) -> bool {
        (self.x==other.x)&&(self.y==other.y)
    }
}
// 1.2 Gvec3
impl<T: ops::Add<Output= T>> ops::Add<Gvec3<T>> for Gvec3<T> {
    type Output = Gvec3<T>;
    fn add(self, _rhs: Gvec3<T>) -> Gvec3<T> {
        Gvec3{ x:self.x+_rhs.x, y:self.y+_rhs.y, z:self.z+_rhs.z }
    }
}
impl<T: ops::Sub<Output=T>> ops::Sub<Gvec3<T>> for Gvec3<T> {
    type Output = Gvec3<T>;
    fn sub(self, _rhs: Gvec3<T>) -> Gvec3<T> {
        Gvec3{ x:self.x-_rhs.x, y:self.y-_rhs.y, z:self.z-_rhs.z }
    }
}
impl<T: ops::Neg<Output=T>> ops::Neg for Gvec3<T> {
    type Output = Gvec3<T>;
    fn neg(self) -> Gvec3<T> {
        Gvec3{ x:-self.x, y:-self.y, z:-self.z }
    }
}
impl<T: Copy+ops::Add<Output=T>> ops::AddAssign for Gvec3<T> {
    fn add_assign(&mut self, other:Self) {
        *self = Self { x:self.x+other.x, y:self.y+other.y, z:self.z+other.z }
    }
}
impl<T: Copy+ops::Sub<Output=T>> ops::SubAssign for Gvec3<T> {
    fn sub_assign(&mut self, other:Self) {
        *self = Self { x:self.x-other.x, y:self.y-other.y, z:self.z-other.z }
    }
}
impl<T: cmp::PartialEq> cmp::PartialEq for Gvec3<T> {
    fn eq(&self, other:&Self) -> bool {
        (self.x==other.x)&&(self.y==other.y)&&(self.z==other.z)
    }
}
// 1.3 Gvec4
impl<T: ops::Add<Output= T>> ops::Add<Gvec4<T>> for Gvec4<T> {
    type Output = Gvec4<T>;
    fn add(self, _rhs: Gvec4<T>) -> Gvec4<T> {
        Gvec4{ x:self.x+_rhs.x, y:self.y+_rhs.y, z:self.z+_rhs.z, w:self.w+_rhs.w }
    }
}
impl<T: ops::Sub<Output=T>> ops::Sub<Gvec4<T>> for Gvec4<T> {
    type Output = Gvec4<T>;
    fn sub(self, _rhs: Gvec4<T>) -> Gvec4<T> {
        Gvec4{ x:self.x-_rhs.x, y:self.y-_rhs.y, z:self.z-_rhs.z, w:self.w-_rhs.w }
    }
}
impl<T: ops::Neg<Output=T>> ops::Neg for Gvec4<T> {
    type Output = Gvec4<T>;
    fn neg(self) -> Gvec4<T> {
        Gvec4{ x:-self.x, y:-self.y, z:-self.z, w:-self.w}
    }
}
impl<T: cmp::PartialEq> cmp::PartialEq for Gvec4<T> {
    fn eq(&self, other:&Self) -> bool {
        (self.x==other.x)&&(self.y==other.y)&&(self.z==other.z)&&(self.w==other.w)
    }
}
// 2. Floating Point operation Methods Trait for Vectors
pub trait VecOp<RHS=Self> {
    type Output;
    fn norm2(self) -> f32;
    fn norm(self) -> f32;
    fn length2(self) -> f32;
    fn length(self) -> f32;
    fn dot(self, rhs:RHS) ->f32;
}
// 2.1 Gvec2
impl VecOp<Gvec2<f32>> for Gvec2<f32> {
    type Output = f32;
    #[inline] fn norm2(self) -> f32 {
        self.x.mul_add(self.x, self.y.clone()* self.y)
    }
    #[inline] fn norm(self) -> f32 {
        self.norm2().sqrt()
    }
    #[inline] fn length2(self) -> f32 {
        self.x.mul_add(self.x, self.y.clone()* self.y)
    }
    #[inline] fn length(self) -> f32 {
        self.length2().sqrt()
    }
    #[inline] fn dot(self,_v:Gvec2<f32>) -> f32 {
        self.x.mul_add(_v.x, self.y * _v.y)/self.length()
    }
}
//2.2 Gvec3
impl VecOp<Gvec3<f32>> for Gvec3<f32> {
    type Output = f32;

    #[inline] fn norm2(self) -> f32 {
        self.x.mul_add(self.x, self.y.mul_add(self.y, self.z.clone()*self.z))
    }
    #[inline] fn norm(self) -> f32 {
        self.norm2().sqrt()
    }
    #[inline] fn length2(self) -> f32 {
        self.x.mul_add(self.x, self.y.mul_add(self.y, self.z.clone()*self.z))
    }
    #[inline] fn length(self) -> f32 {
        self.length2().sqrt()
    }
    #[inline] fn dot(self,_v:Gvec3<f32>) -> f32 {
        self.x.mul_add(_v.x, self.y.mul_add(_v.y, _v.z*self.z))/self.length()
    }
}
//2.2 Gvec4
impl VecOp<Gvec4<f32>> for Gvec4<f32> {
    type Output = f32;

    #[inline] fn norm2(self) -> f32 {
        self.x.mul_add(self.x, self.y.mul_add(self.y, self.z.mul_add(self.z, self.w.clone()*self.z)))
    }
    #[inline] fn norm(self) -> f32 {
        self.norm2().sqrt()
    }
    #[inline] fn length2(self) -> f32 {
        self.x.mul_add(self.x, self.y.mul_add(self.y, self.z.mul_add(self.z, self.w.clone()*self.z)))
    }
    #[inline] fn length(self) -> f32 {
        self.length2().sqrt()
    }
    #[inline] fn dot(self,_v:Gvec4<f32>) -> f32 {
        self.x.mul_add(_v.x, self.y.mul_add(self.y, _v.z.mul_add(self.z, _v.w*self.z)))/self.length()
    }
}
// exactly todo = array to vector copy
pub type Fvec2 = Gvec2<f32>;
pub type Ivec2 = Gvec2<i32>;
pub type Uvec2 = Gvec2<u32>;
pub type Fvec3 = Gvec3<f32>;
pub type Ivec3 = Gvec3<i32>;
pub type Uvec3 = Gvec3<u32>;
pub type Fvec4 = Gvec4<f32>;
pub type Ivec4 = Gvec4<i32>;
pub type Uvec4 = Gvec4<u32>;

impl_fmt!(Fvec2{ x y }, "<{} {}>");
impl_fmt!(Ivec2{ x y }, "<{} {}>");
impl_fmt!(Uvec2{ x y }, "<{} {}>");
impl_fmt!(Fvec3{ x y z }, "<{} {} {}>");
impl_fmt!(Ivec3{ x y z }, "<{} {} {}>");
impl_fmt!(Uvec3{ x y z }, "<{} {} {}>");
impl_fmt!(Fvec4{ x y z w }, "<{} {} {} {}>");
impl_fmt!(Ivec4{ x y z w }, "<{} {} {} {}>");
impl_fmt!(Uvec4{ x y z w }, "<{} {} {} {}>");


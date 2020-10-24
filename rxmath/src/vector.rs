//*********************************************************
// Copyright 2020-2020 Hyeonjang An
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.
//*********************************************************

extern crate libc;
//////////////////////////////
/// 0. tupled array sturctures 
pub struct Garr2<T>(pub T, pub T);
pub struct Garr3<T>(pub T, pub T, pub T);
pub struct Garr4<T>(pub T, pub T, pub T);
///////////////////////////////
// 0. Vector Structures
pub struct Gvec2<T> { pub x:T, pub y:T }
pub struct Gvec3<T> { pub x:T, pub y:T, pub z : T }
pub struct Gvec4<T> { pub x:T, pub y:T, pub z : T, pub w:T }
//////////////////////////////
// 
// 0.1.0 Contructors & Copy, Clone
impl<T> Gvec2<T> {
    #[inline] pub fn new(_x:T, _y:T) -> Gvec2<T> {
        Gvec2{ x:_x, y:_y } 
    }
}
impl<T> Gvec3<T> {
    #[inline] pub fn new(_x:T, _y:T, _z:T) -> Gvec3<T> {
        Gvec3{ x:_x, y:_y, z:_z } 
    }
}
impl<T> Gvec4<T> {
    #[inline] pub fn new(_x:T, _y:T, _z:T, _w:T) -> Gvec4<T> {
        Gvec4{ x:_x, y:_y, z:_z, w:_w }
    }
}
impl<T: Default> Default for Gvec2<T> {
    #[inline] fn default() -> Self {
        Gvec2{ x:T::default(), y:T::default() }
    }
}
impl<T: Default> Default for Gvec3<T> {
    #[inline] fn default() -> Self {
        Gvec3{ x:T::default(), y:T::default(), z:T::default() }
    }
}
impl<T: Default> Default for Gvec4<T> {
    #[inline] fn default() -> Self {
        Gvec4{ x:T::default(), y:T::default(), z:T::default(), w:T::default() }
    }
}
// 0.1.1 Function Contructors for Convenience
pub fn vec2<T>(_x:T, _y:T) -> Gvec2<T> {
    Gvec2{ x:_x, y:_y }
}
pub fn vec3<T>(_x:T, _y:T, _z:T) -> Gvec3<T> {
    Gvec3{ x:_x, y:_y, z:_z }
}
pub fn vec4<T>(_x:T, _y:T, _z:T, _w:T) -> Gvec4<T> {
    Gvec4{ x:_x, y:_y, z:_z, w:_w }
}
// 0.0.2 Copy & Clone
impl_cpy!(Gvec2<T>{ x y });
impl_cpy!(Gvec3<T>{ x y z });
impl_cpy!(Gvec4<T>{ x y z w });

//////////////////////////////
// 1. Operator overloading
// 1.1 cmp
impl_cmp!(Gvec2<T>{ x y });
impl_cmp!(Gvec3<T>{ x y z });
impl_cmp!(Gvec4<T>{ x y z w });
// 1.2. ops
impl_ops!(Gvec2<T>{ x y });
impl<T> std::ops::Index<usize> for Gvec2<T> {
    type Output = T;
    fn index<'a>(&'a self, i:usize) -> &T {
        match i {
            0 => &self.x,
            1 => &self.y,
            _ => &&self.x,
        }
    }
}
impl_ops!(Gvec4<T>{ x y z w });
// 1.2.1 cross product of Vector3 
impl<T: Copy+std::ops::Mul<Output=T>+std::ops::Sub<Output=T>> std::ops::Mul<Gvec3<T>> for Gvec3<T>{
    type Output = Self;
    fn mul(self, rhs:Gvec3<T>) -> Self {
        Gvec3::new( self.y*rhs.z-self.z*rhs.y, self.z*rhs.x-self.x*rhs.z, self.x*rhs.y-self.y*rhs.x )
     }
} 
impl_ops!(Gvec3<T>{ x y z });

//////////////////////////////
/// 2. Vector features
// 2.1 Floating Point operation Methods Trait for Vectors
pub trait VecOp<RHS=Self> {
    type Output;
    fn norm2(&self) -> f32;
    fn norm(&self) -> f32;
    fn length2(&self) -> f32;
    fn length(&self) -> f32;
    fn dot(self, rhs:RHS) ->f32;
    fn normalize(self) -> Self;
}
// 2.1.1 Gvec2
impl VecOp<Gvec2<f32>> for Gvec2<f32> {
    type Output = f32;
    #[inline] fn norm2(&self) -> f32 {
        self.x.mul_add(self.x, self.y.clone()* self.y)
    }
    #[inline] fn norm(&self) -> f32 {
        self.norm2().sqrt()
    }
    #[inline] fn length2(&self) -> f32 {
        self.x.mul_add(self.x, self.y.clone()* self.y)
    }
    #[inline] fn length(&self) -> f32 {
        self.length2().sqrt()
    }
    #[inline] fn dot(self,_v:Gvec2<f32>) -> f32 {
        self.x.mul_add(_v.x, self.y * _v.y)/self.length()
    }
    #[inline] fn normalize(self) -> Self {
        self/self.length()
    }
}
// 2.1.2 Gvec3
impl VecOp<Gvec3<f32>> for Gvec3<f32> {
    type Output = f32;

    #[inline] fn norm2(&self) -> f32 {
        self.x.mul_add(self.x, self.y.mul_add(self.y, self.z.clone()*self.z))
    }
    #[inline] fn norm(&self) -> f32 {
        self.norm2().sqrt()
    }
    #[inline] fn length2(&self) -> f32 {
        self.x.mul_add(self.x, self.y.mul_add(self.y, self.z.clone()*self.z))
    }
    #[inline] fn length(&self) -> f32 {
        self.length2().sqrt()
    }
    #[inline] fn dot(self,_v:Gvec3<f32>) -> f32 {
        self.x.mul_add(_v.x, self.y.mul_add(_v.y, _v.z*self.z))/self.length()
    }
    #[inline] fn normalize(self) -> Self {
        self/self.length()
    }
}
// 2.1.3 Gvec4
impl VecOp<Gvec4<f32>> for Gvec4<f32> {
    type Output = f32;

    #[inline] fn norm2(&self) -> f32 {
        self.x.mul_add(self.x, self.y.mul_add(self.y, self.z.mul_add(self.z, self.w.clone()*self.z)))
    }
    #[inline] fn norm(&self) -> f32 {
        self.norm2().sqrt()
    }
    #[inline] fn length2(&self) -> f32 {
        self.x.mul_add(self.x, self.y.mul_add(self.y, self.z.mul_add(self.z, self.w.clone()*self.z)))
    }
    #[inline] fn length(&self) -> f32 {
        self.length2().sqrt()
    }
    #[inline] fn dot(self,_v:Gvec4<f32>) -> f32 {
        self.x.mul_add(_v.x, self.y.mul_add(self.y, _v.z.mul_add(self.z, _v.w*self.z)))/self.length()
    }
    #[inline] fn normalize(self) -> Self {
        self/self.length()
    }
}

//////////////////////////////
// 3. Final type aliasing?? <<aliasing is right expression?>>
pub type Fvec2 = Gvec2<f32>;
pub type Ivec2 = Gvec2<i32>;
pub type Uvec2 = Gvec2<u32>;
pub type Fvec3 = Gvec3<f32>;
pub type Ivec3 = Gvec3<i32>;
pub type Uvec3 = Gvec3<u32>;
pub type Fvec4 = Gvec4<f32>;
pub type Ivec4 = Gvec4<i32>;
pub type Uvec4 = Gvec4<u32>;

//////////////////////////////
/// ** considering point type
pub type Fpoint2 = Gvec2<f32>;
pub type Ipoint2 = Gvec2<i32>;
pub type Upoint2 = Gvec2<u32>;
pub type Fpoint3 = Gvec3<f32>;
pub type Ipoint3 = Gvec3<i32>;
pub type Upoint3 = Gvec3<u32>;
pub type Fpoint4 = Gvec4<f32>;
pub type Ipoint4 = Gvec4<i32>;
pub type Upoint4 = Gvec4<u32>;

// exactly todo = array to vector copy
//////////////////////////////
// 4. Rust Display, Debug Printing Functions
impl_fmt!(Fvec2{ x y }, "<{} {}>");
impl_fmt!(Ivec2{ x y }, "<{} {}>");
impl_fmt!(Uvec2{ x y }, "<{} {}>");
impl_fmt!(Fvec3{ x y z }, "<{} {} {}>");
impl_fmt!(Ivec3{ x y z }, "<{} {} {}>");
impl_fmt!(Uvec3{ x y z }, "<{} {} {}>");
impl_fmt!(Fvec4{ x y z w }, "<{} {} {} {}>");
impl_fmt!(Ivec4{ x y z w }, "<{} {} {} {}>");
impl_fmt!(Uvec4{ x y z w }, "<{} {} {} {}>");

//@@todo pub function to calculate ~~~~

#[inline] pub fn dot( v0:Fvec3, v1:Fvec3 ) -> f32 {
    v0.dot(v1)
}
#[inline] pub fn normalize( v:Fvec3 ) -> Fvec3 {
    v.normalize()
}
#[inline] pub fn sqrt(f:f32) -> f32 {
    f.sqrt()
} 
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

use crate::random::*;
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
    #[inline] pub fn new(x:T, y:T) -> Gvec2<T> {
        Gvec2{ x:x, y:y } 
    }
}
impl<T> Gvec3<T> {
    #[inline] pub fn new(x:T, y:T, z:T) -> Gvec3<T> {
        Gvec3{ x:x, y:y, z:z } 
    }
}
impl<T> Gvec4<T> {
    #[inline] pub fn new(x:T, y:T, z:T, w:T) -> Gvec4<T> {
        Gvec4{ x:x, y:y, z:z, w:w }
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
// 1.1 c
impl_cmp!(Gvec2<T>{ x y });
impl_cmp!(Gvec3<T>{ x y z });
impl_cmp!(Gvec4<T>{ x y z w });
// 1.2. operator
impl_ops!(Gvec2<T>{ x y });
impl_ops!(Gvec3<T>{ x y z });
impl_ops!(Gvec4<T>{ x y z w });
// 1.3. index
impl_idx!(Gvec2<T>{ x 0 y 1 });
impl_idx!(Gvec3<T>{ x 0 y 1 z 2 });
impl_idx!(Gvec4<T>{ x 0 y 1 z 2 w 3 });

//////////////////////////////
/// 2. Vector features
// 2.1 Floating Point operation Methods Trait for Vectors
#[allow(non_camel_case_types)]
pub trait vec<RHS> {
    type Output;
    fn norm2(&self) -> Self::Output;
    fn norm(&self) -> Self::Output;
    fn length2(&self) -> Self::Output;
    fn length(&self) -> Self::Output;
    fn dot(&self, rhs:RHS) -> Self::Output;
    fn normalize(&self) -> Self;
    fn near_zero(&self) -> bool;
}
// 2.1.1 Gvec2
impl vec<Gvec2<f32>> for Gvec2<f32> {
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
    #[inline] fn dot(&self, rhs:Gvec2<f32>) -> f32 {
        self.x.mul_add(rhs.x, self.y * rhs.y)
    }
    #[inline] fn normalize(&self) -> Self {
        *self/self.length()
    }
    #[inline] fn near_zero(&self) -> bool {
        const S:f32 = 1e-8;
        return (self.x.abs()<S) && (self.y.abs()<S)
    }
}
// 2.1.2 Gvec3
impl vec<Gvec3<f32>> for Gvec3<f32> {
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
    #[inline] fn dot(&self,_v:Gvec3<f32>) -> f32 {
        self.x.mul_add(_v.x, self.y.mul_add(_v.y, _v.z*self.z))
    }
    #[inline] fn normalize(&self) -> Self {
        *self/self.length()
    }
    #[inline] fn near_zero(&self) -> bool {
        const S:f32 = 1e-8;
        return (self.x.abs()<S) && (self.y.abs()<S) && (self.z.abs()<S)
    }
}

impl Gvec3<f32> {
    #[inline] pub fn cross(&self, rhs:Gvec3<f32>) -> Self{
        Gvec3::new( self.y*rhs.z-self.z*rhs.y, self.z*rhs.x-self.x*rhs.z, self.x*rhs.y-self.y*rhs.x)
    } 
    #[inline] pub fn random() -> Self {
        vec3(random_f32(), random_f32(), random_f32())
    }
    #[inline] pub fn random_range(x:f32, y:f32) -> Self {
        vec3(random_range_f32(x, y), random_range_f32(x, y), random_range_f32(x, y))
    }
    #[inline] pub fn min() -> Self {
        vec3(f32::MIN, f32::MIN, f32::MIN)
    }
    #[inline] pub fn max() -> Self {
        vec3(f32::MAX, f32::MAX, f32::MAX)
    }
}

// 2.1.3 Gvec4
impl vec<Gvec4<f32>> for Gvec4<f32> {
    type Output = f32;

    #[inline] fn norm2(&self) -> f32 {
        self.x.mul_add(self.x, self.y.mul_add(self.y, self.z.mul_add(self.z, self.w.clone()*self.w)))
    }
    #[inline] fn norm(&self) -> f32 {
        self.norm2().sqrt()
    }
    #[inline] fn length2(&self) -> f32 {
        self.x.mul_add(self.x, self.y.mul_add(self.y, self.z.mul_add(self.z, self.w.clone()*self.w)))
    }
    #[inline] fn length(&self) -> f32 {
        self.length2().sqrt()
    }
    #[inline] fn dot(&self,v:Gvec4<f32>) -> f32 {
        self.x.mul_add(v.x, self.y.mul_add(self.y, v.z.mul_add(self.z, v.w*self.w)))
    }
    #[inline] fn normalize(&self) -> Self {
        *self/self.length()
    }
    #[inline] fn near_zero(&self) -> bool {
        const S:f32 = 1e-8;
        return (self.x.abs()<S) && (self.y.abs()<S) && (self.z.abs()<S) && (self.w.abs()<S)
    }
}

//////////////////////////////
// 3. Final type aliasing
#[allow(non_camel_case_types)] pub type vec2 = Gvec2<f32>;
#[allow(non_camel_case_types)] pub type bvec2 = Gvec2<bool>;
#[allow(non_camel_case_types)] pub type ivec2 = Gvec2<i32>;
#[allow(non_camel_case_types)] pub type uvec2 = Gvec2<u32>;
#[allow(non_camel_case_types)] pub type vec3 = Gvec3<f32>;
#[allow(non_camel_case_types)] pub type bvec3 = Gvec3<bool>;
#[allow(non_camel_case_types)] pub type ivec3 = Gvec3<i32>;
#[allow(non_camel_case_types)] pub type uvec3 = Gvec3<u32>;
#[allow(non_camel_case_types)] pub type vec4 = Gvec4<f32>;
#[allow(non_camel_case_types)] pub type bvec4 = Gvec4<bool>;
#[allow(non_camel_case_types)] pub type ivec4 = Gvec4<i32>;
#[allow(non_camel_case_types)] pub type uvec4 = Gvec4<u32>;

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
impl_fmt!(vec2{ x y }, "<{} {}>");
impl_fmt!(ivec2{ x y }, "<{} {}>");
impl_fmt!(uvec2{ x y }, "<{} {}>");
impl_fmt!(vec3{ x y z }, "<{} {} {}>");
impl_fmt!(ivec3{ x y z }, "<{} {} {}>");
impl_fmt!(uvec3{ x y z }, "<{} {} {}>");
impl_fmt!(vec4{ x y z w }, "<{} {} {} {}>");
impl_fmt!(ivec4{ x y z w }, "<{} {} {} {}>");
impl_fmt!(uvec4{ x y z w }, "<{} {} {} {}>");

#[inline] pub fn dot( v0:vec3, v1:vec3 ) -> f32 {
    v0.dot(v1)
}
#[inline] pub fn cross( v0:vec3, v1:vec3 ) -> vec3 {
    v1.cross(v0)
}
#[inline] pub fn normalize( v:vec3 ) -> vec3 {
    v.normalize()
}
#[inline] pub fn clamp( x:f32, min:f32, max:f32 ) -> f32 {
    if x<min { return min; } 
    if x>max { return max; }
    return x;
}
#[inline] pub fn saturate( x:f32 ) -> f32 {
    return clamp(x, 0.0, 1.0);
}
#[inline] pub fn saturate_vec3( v:vec3 ) -> vec3 {
    return vec3(saturate(v.x), saturate(v.y), saturate(v.z));
}
#[inline] pub fn reflect( v:vec3, n:vec3 ) ->vec3 {
    return v - n*dot(v, n)*2.0;
}
#[inline] pub fn refract( uv:vec3, n:vec3, etai_over_etat:f32 ) -> vec3 {
    let cos_theta = dot(-uv, n).min(1.0);
    let r_perp = (uv+n*cos_theta)*etai_over_etat;
    let r_para = n * (-f32::sqrt((1.0-r_perp.length2()).abs()));
    return r_perp + r_para;
}
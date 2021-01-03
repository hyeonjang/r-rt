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

use crate::vector::*;
/////////////////////////
/// 0. Matrix Sturctures
pub struct Gmat2<T>{
    pub  _00 : T, pub _01 : T,
    pub  _10 : T, pub _11 : T,
}
pub struct Gmat3<T> {
    pub  _00 : T, pub _01 : T, pub _02 : T,
    pub  _10 : T, pub _11 : T, pub _12 : T, 
    pub  _20 : T, pub _21 : T, pub _22 : T, 
}
pub struct Gmat4<T> {
    pub  _00 : T, pub _01 : T, pub _02 : T, pub _03 : T, 
    pub  _10 : T, pub _11 : T, pub _12 : T, pub _13 : T, 
    pub  _20 : T, pub _21 : T, pub _22 : T, pub _23 : T,
    pub  _30 : T, pub _31 : T, pub _32 : T, pub _33 : T,
}
///////////////////////////
/// 
// 0.1.0 Constructors & Copy, Clone
impl<T> Gmat2<T>{
    pub fn new(_00:T, _01:T, _10:T, _11:T) -> Gmat2<T> {
        Gmat2{ _00:_00, _01:_01, _10:_10, _11:_11 }
    }
}
impl<T> Gmat3<T>{
    pub fn new(_00:T, _01:T, _02:T, _10:T, _11:T, _12:T, _20:T, _21:T, _22:T) -> Gmat3<T> {
        Gmat3{ _00:_00, _01:_01, _02:_02, _10:_10, _11:_11, _12:_12, _20:_20, _21:_21, _22:_22 }
    }
}
impl<T> Gmat4<T>{
    pub fn new(_00:T, _01:T, _02:T, _03:T, _10:T, _11:T, _12:T, _13:T, _20:T, _21:T, _22:T, _23:T, _30:T, _31:T, _32:T, _33:T ) -> Gmat4<T> {
        Gmat4{ _00:_00, _01:_01, _02:_02, _03:_03, _10:_10, _11:_11, _12:_12, _13:_13, _20:_20, _21:_21, _22:_22, _23:_23, _30:_30, _31:_31, _32:_32, _33:_33, }
    }
}
// 0.0.1 Copy & Clone
impl_cpy!(Gmat2<T>{ _00 _01 _10 _11});
impl_cpy!(Gmat3<T>{ _00 _01 _02 _10 _11 _12 _20 _21 _22});
impl_cpy!(Gmat4<T>{ _00 _01 _02 _03 _10 _11 _12 _13 _20 _21 _22 _23 _30 _31 _32 _33 });
// Matrix Operator overloading for f32 //@@todo thinking about this

//////////////////////
// 1. Operator overloading 
// 1.1. ops 
impl_ops!(Gmat2<T>{ _00 _01 _10 _11});
impl<T: Copy+std::ops::Mul<Output=T>+std::ops::Sub<Output=T>> std::ops::Mul<Gvec2<T>> for Gmat2<T>{
    type Output = Gmat2<T>;
    fn mul(self, rhs:Gvec2<T>) -> Self {
        Gmat2::new(self._00*rhs.x, self._01*rhs.y, 
        /*2nd row*/self._10*rhs.x, self._11*rhs.y)
     }
}
impl<T: Copy+std::ops::Mul<Output=T>+std::ops::Add<Output=T>> std::ops::Mul<Gmat2<T>> for Gvec2<T>{
    type Output = Gvec2<T>;
    fn mul(self, rhs:Gmat2<T>) -> Self {
        vec2(self.x*rhs._00 + self.y*rhs._10, self.x*rhs._01 + self.y*rhs._11)
    }
}  
impl_ops!(Gmat3<T>{ _00 _01 _02 _10 _11 _12 _20 _21 _22});
impl<T: Copy+std::ops::Mul<Output=T>+std::ops::Sub<Output=T>> std::ops::Mul<Gvec3<T>> for Gmat3<T>{
    type Output = Gmat3<T>;
    fn mul(self, rhs:Gvec3<T>) -> Self {
        Gmat3::new(self._00*rhs.x, self._01*rhs.y, self._02*rhs.z,
        /*2nd row*/self._10*rhs.x, self._11*rhs.y, self._12*rhs.z,
        /*3rd row*/self._20*rhs.x, self._21*rhs.y, self._22*rhs.z)
    }
} 
impl<T: Copy+std::ops::Mul<Output=T>+std::ops::Add<Output=T>> std::ops::Mul<Gmat3<T>> for Gvec3<T>{
    type Output = Gvec3<T>;
    fn mul(self, rhs:Gmat3<T>) -> Self {
        vec3(self.x*rhs._00 + self.y*rhs._10 + self.z*rhs._20, self.x*rhs._01 + self.y*rhs._11 + self.z*rhs._21, self.x*rhs._02 + self.y*rhs._12 + self.z*rhs._22)
    }
} 
impl_ops!(Gmat4<T>{ _00 _01 _02 _03 _10 _11 _12 _13 _20 _21 _22 _23 _30 _31 _32 _33 });
impl<T: Copy+std::ops::Mul<Output=T>+std::ops::Sub<Output=T>> std::ops::Mul<Gvec4<T>> for Gmat4<T>{
    type Output = Gmat4<T>;
    fn mul(self, rhs:Gvec4<T>) -> Self {
        Gmat4::new(self._00*rhs.x, self._10*rhs.y, self._20*rhs.z, self._30*rhs.w, 
        /*2nd row*/self._01*rhs.x, self._11*rhs.y, self._21*rhs.z, self._31*rhs.w, 
        /*3nd row*/self._02*rhs.x, self._12*rhs.y, self._22*rhs.z, self._23*rhs.w,
        /*4th row*/self._03*rhs.x, self._13*rhs.y, self._23*rhs.z, self._33*rhs.w)
     }
}
impl<T: Copy+std::ops::Mul<Output=T>+std::ops::Add<Output=T>> std::ops::Mul<Gmat4<T>> for Gvec4<T>{
    type Output = Gvec4<T>;
    fn mul(self, rhs:Gmat4<T>) -> Self {
        vec4(self.x*rhs._00 + self.y*rhs._10 + self.z*rhs._20 + self.w*rhs._30, 
             self.x*rhs._01 + self.y*rhs._11 + self.z*rhs._21 + self.w*rhs._31, 
             self.x*rhs._02 + self.y*rhs._12 + self.z*rhs._22 + self.w*rhs._32,
             self.x*rhs._03 + self.y*rhs._12 + self.z*rhs._23 + self.w*rhs._33)
    }
} 
// 1.2 cmp 
impl_cmp!(Gmat2<T>{ _00 _01 _10 _11});
impl_cmp!(Gmat3<T>{ _00 _01 _02 _10 _11 _12 _20 _21 _22});
impl_cmp!(Gmat4<T>{ _00 _01 _02 _03 _10 _11 _12 _13 _20 _21 _22 _23 _30 _31 _32 _33 });

////////////////////////////
/// 2. Matrix features
// 2.1 Floating Point operation Methods Trait for Matrix
pub trait MatOp<T>  {
    fn ident() -> Self;
    fn det(&self) -> f32; 
    fn inverse(self) ->Self;
    fn transpose(self) -> Self;
    //fn scale(self, s:f32) -> Gmat2<T>; // todo change 
}
// 2.1.1 Gmat2 
impl MatOp<f32> for Gmat2<f32> {
    fn ident() -> Self {
        Gmat2::new(1f32, 0f32, 
                   0f32, 1f32)
    }
    fn det(&self) -> f32{ self._00*self._11 - self._01*self._10 }
    fn inverse(self) -> Self {
        let div = 1_f32/self.det();
        self*div
    }
    fn transpose(self) -> Self {
        Gmat2::new(self._00, self._10, 
                   self._01, self._11)
    }
}
// 2.1.1 Gmat3 
impl MatOp<f32> for Gmat3<f32> {
    fn ident() -> Gmat3<f32> {
        Gmat3::new(1_f32, 0_f32, 0_f32,
        /*2nd row*/0_f32, 1_f32, 0_f32,
        /*3rd row*/0_f32, 0_f32, 1_f32)
    }
    fn det(&self) -> f32{ self._00*self._11*self._22 - self._02*self._11*self._20 }
    fn inverse(self) -> Self {
        let div = 1_f32/self.det();
        self*div
    }
    fn transpose(self) -> Self {
        Gmat3::new(self._00, self._10, self._20, 
        /*2nd row*/self._01, self._11, self._21, 
        /*3nd row*/self._02, self._12, self._22,)
    }
}// 2.1.1 Gmat4 
impl MatOp<f32> for Gmat4<f32> {
    fn ident() -> Gmat4<f32> {
        Gmat4::new(1_f32, 0_f32, 0_f32, 0_f32,
        /*2nd row*/0_f32, 1_f32, 0_f32, 0_f32,
        /*3nd row*/0_f32, 0_f32, 1_f32, 0_f32,
        /*4th row*/0_f32, 0_f32, 0_f32, 1_f32)
    }
    fn det(&self) -> f32{ self._00*self._11*self._22*self._33 - self._03*self._12*self._21*self._30 }
    fn inverse(self) -> Self {
        let div = 1_f32/self.det();
        self*div
    }
    fn transpose(self) -> Self {
        Gmat4::new(self._00, self._10, self._20, self._30, 
        /*2nd row*/self._01, self._11, self._21, self._31, 
        /*3nd row*/self._02, self._12, self._22, self._23,
        /*4th row*/self._03, self._13, self._23, self._33,)
    }
}
// 4x4 matrix functions
impl Gmat4<f32> {
    pub fn perspective() -> Self {
        Gmat4::ident()
    }
}

//////////////////////////////
// 3. Final type aliasing
#[allow(non_camel_case_types)] pub type mat2 = Gmat2<f32>;
#[allow(non_camel_case_types)] pub type mat3 = Gmat3<f32>;
#[allow(non_camel_case_types)] pub type mat4 = Gmat4<f32>;

//////////////////////////////
// 4. Rust Display, Debug Printing Functions
impl_fmt!(mat2{ _00 _01 _10 _11 }, "|{} {}|\n|{} {}|");
impl_fmt!(mat3{ _00 _01 _02 _10 _11 _12 _20 _21 _22 }, "|{} {} {}|\n|{} {} {}|\n|{} {} {}|");
impl_fmt!(mat4{ _00 _01 _02 _03 _10 _11 _12 _13 _20 _21 _22 _23 _30 _31 _32 _33}, "|{} {} {} {}|\n|{} {} {} {}|\n|{} {} {} {}|\n|{} {} {} {}|");

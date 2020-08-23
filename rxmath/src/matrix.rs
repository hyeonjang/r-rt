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

//use crate::vector::*;
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
impl_ops!(Gmat3<T>{ _00 _01 _02 _10 _11 _12 _20 _21 _22});
impl_ops!(Gmat4<T>{ _00 _01 _02 _03 _10 _11 _12 _13 _20 _21 _22 _23 _30 _31 _32 _33 });
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
        Gmat2::new(1_f32, 0_f32, 1_f32, 0_f32)
    }
    fn det(&self) -> f32{ self._00*self._11 - self._01*self._10 }
    fn inverse(self) -> Self {
        let div = 1_f32/self.det();
        self*div
    }
    fn transpose(self) -> Self {
        Gmat2::new(self._00, self._10, self._01, self._11)
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
// 3. Final type aliasing?? <<aliasing is right expression?>>
pub type Fmat2 = Gmat2<f32>;
pub type Umat2 = Gmat2<u32>;
pub type Imat2 = Gmat2<i32>;
pub type Fmat3 = Gmat3<f32>;
pub type Umat3 = Gmat3<u32>;
pub type Imat3 = Gmat3<i32>;
pub type Fmat4 = Gmat4<f32>;
pub type Umat4 = Gmat4<u32>;
pub type Imat4 = Gmat4<i32>;

//////////////////////////////
// 4. Rust Display, Debug Printing Functions
impl_fmt!(Fmat2{ _00 _01 _10 _11 }, "|{} {}|\n|{} {}|");
impl_fmt!(Umat2{ _00 _01 _10 _11 }, "|{} {}|\n|{} {}|");
impl_fmt!(Imat2{ _00 _01 _10 _11 }, "|{} {}|\n|{} {}|");
impl_fmt!(Fmat3{ _00 _01 _02 _10 _11 _12 _20 _21 _22 }, "|{} {} {}|\n|{} {} {}|\n|{} {} {}|");
impl_fmt!(Umat3{ _00 _01 _02 _10 _11 _12 _20 _21 _22 }, "|{} {} {}|\n|{} {} {}|\n|{} {} {}|");
impl_fmt!(Imat3{ _00 _01 _02 _10 _11 _12 _20 _21 _22 }, "|{} {} {}|\n|{} {} {}|\n|{} {} {}|");
impl_fmt!(Fmat4{ _00 _01 _02 _03 _10 _11 _12 _13 _20 _21 _22 _23 _30 _31 _32 _33}, "|{} {} {} {}|\n|{} {} {} {}|\n|{} {} {} {}|\n|{} {} {} {}|");
impl_fmt!(Umat4{ _00 _01 _02 _03 _10 _11 _12 _13 _20 _21 _22 _23 _30 _31 _32 _33}, "|{} {} {} {}|\n|{} {} {} {}|\n|{} {} {} {}|\n|{} {} {} {}|");
impl_fmt!(Imat4{ _00 _01 _02 _03 _10 _11 _12 _13 _20 _21 _22 _23 _30 _31 _32 _33}, "|{} {} {} {}|\n|{} {} {} {}|\n|{} {} {} {}|\n|{} {} {} {}|");

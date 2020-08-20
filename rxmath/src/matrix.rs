use crate::vector;

pub struct Tmat2<T> {
   pub  _00 : T, pub _01 : T, 
   pub  _10 : T, pub _11 : T, 
}

pub struct Tmat3<T> {
    pub  _00 : T, pub _01 : T, pub _02 : T,
    pub  _10 : T, pub _11 : T, pub _12 : T, 
    pub  _20 : T, pub _21 : T, pub _22 : T, 
 }
 
pub struct Tmat4<T> {
    pub  _00 : T, pub _01 : T, pub _02 : T, pub _03 : T, 
    pub  _10 : T, pub _11 : T, pub _12 : T, pub _13 : T, 
    pub  _20 : T, pub _21 : T, pub _22 : T, pub _23 : T,
    pub  _30 : T, pub _31 : T, pub _32 : T, pub _33 : T,
}

impl<T> Tmat2<T> {
    
}
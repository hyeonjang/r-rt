//use crate::vector::*;
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
pub trait MatOp<T>  {
    fn new(_11:T, _12:T, _21:T, _22:T) -> Gmat2<T>;
    fn ident() -> Self;
    fn det(&self) -> f32; 
    fn inverse(&self) ->Self;
    fn scale(&self, s:f32) -> Gmat2<T>;
}

impl MatOp<f32> for Gmat2<f32> {
    fn new(_00:f32, _01:f32, _10:f32, _11:f32 ) -> Gmat2<f32> {
        Gmat2{_00:_00, _01:_01, _10:_10, _11:_11}
    }
    fn ident() -> Gmat2<f32> {
        Gmat2{ _00:1_f32, _01:0_f32, _10:1_f32, _11:0_f32}
    }
    fn det(&self) -> f32{ return self._00*self._11 - self._01*self._10; }
    fn inverse(&self) -> Self {
        let div = 1_f32/self.det();
        Gmat2{
            _00:self._11*div, _01:self._10*div,
            _10:self._01*div, _11:self._00*div,
        }
    }
    fn scale(&self, s:f32) -> Self{
        Gmat2{
            _00:self._00*s, _01:self._01,
            _10:self._10,   _11:self._11*s,
        }
    }
}

pub type Fmat2 = Gmat2<f32>;
impl_fmt!(Fmat2{ _00 _01 _10 _11 }, "|{} {}|\n|{} {}|");
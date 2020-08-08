//implementing with referencing cgmath

extern crate num_traits;
use std::ops;

pub struct Vector2<T> { 
    x : T, 
    y : T,
}

pub struct Vector3<T> {
    x : T,
    y : T,
    z : T,
}

pub struct Vector4<T> {
    x : T, 
    y : T,
    z : T,
    w : T,
}

impl<T: ops::Add<Output = T>> ops::Add<Vector2<T>> for Vector2<T> {
    type Output = Vector2<T>;

    fn add(self, _rhs: Vector2<T>) -> Vector2<T> {
        Vector2{ x:self.x+_rhs.x, y:self.y+_rhs.y }
    }
}

impl<T: std::clone::Clone+ops::Add<Output=T>+ops::Mul<Output=T>+ops::Div<Output=T>> Vector2<T> {
    pub fn new(_x:T, _y:T) -> Vector2<T> {
        Vector2 { x:_x, y:_y } 
    }

    // norm/length/dot: floating-point only functions
    pub fn norm2(self) -> T {
        self.x.clone() * self.x + self.y.clone()* self.y
    }
    pub fn norm(self) -> T {
        self.norm2()
    }
    pub fn length2(&self) -> T {
        self.x.clone() * self.x.clone() + self.y.clone() * self.y.clone()
    }
    pub fn length(&self) -> T {
        self.length2()
    }
    pub fn dot(self, v:Vector2<T>) -> T {
        self.x * v.x + self.y * v.y
    }
}

pub fn vec2<T>(_x:T, _y:T) -> Vector2<T> {
    Vector2 { x:_x, y:_y }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {

        let vec0 = Vector2::new(2, 2);
        let vec1 = Vector2::new(3, 3);
        let vec2 = vec0 + vec1;
        //let vec1 = Vector2::new(3, 3);
        //let vec2 = Vector2::new(5, 5);
        let vec_norm = vec2.norm2();
        assert_eq!(vec_norm, 50);
    }
}

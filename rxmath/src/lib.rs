extern crate num_traits;
use std::ops;

// 0. Vector Structures 
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

// 0.1.0 Contructors 
impl<T> Vector2<T> {
    pub fn new(_x:T, _y:T) -> Vector2<T> {
        Vector2 { x:_x, y:_y } 
    }
}

impl<T> Vector3<T> {
    pub fn new(_x:T, _y:T, _z:T) -> Vector3<T> {
        Vector3 { x:_x, y:_y, z:_z } 
    }
}

impl<T> Vector4<T> {
    pub fn new(_x:T, _y:T, _z:T, _w:T) -> Vector4<T> {
        Vector4 { x:_x, y:_y, z:_z, w:_w }
    }
}

// 0.1.1 Function Contructors for Convenience
pub fn vec2<T>(_x:T, _y:T) -> Vector2<T> {
    Vector2 { x:_x, y:_y }
}

pub fn vec3<T>(_x:T, _y:T, _z:T) -> Vector3<T> {
    Vector3 { x:_x, y:_y, z:_z }
}

// 1. Operator overloading
// 1.1 Vector2
impl<T: ops::Add<Output = T>> ops::Add<Vector2<T>> for Vector2<T> {
    type Output = Vector2<T>;

    fn add(self, _rhs: Vector2<T>) -> Vector2<T> {
        Vector2{ x:self.x+_rhs.x, y:self.y+_rhs.y }
    }
}

impl<T: ops::Sub<Output = T>> ops::Sub<Vector2<T>> for Vector2<T> {
    type Output = Vector2<T>;

    fn sub(self, _rhs: Vector2<T>) -> Vector2<T> {
        Vector2{ x:self.x-_rhs.x, y:self.y-_rhs.y }
    }
}

// 2. Methods for Vectors
// 2.1 Vector2
impl<Float: num_traits::Float + std::clone::Clone+ops::Add<Output=Float>+ops::Mul<Output=Float>+ops::Div<Output=Float>> Vector2<Float> {
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
    pub fn dot(self, v:Vector2<Float>) -> Float {
        self.x.mul_add(v.x, self.y * v.y)/self.length()
    }
}

// impl<T: std::clone::Clone+ops::Add<Output=T>+ops::Mul<Output=T>+ops::Div<Output=T>> Vector2<T> {
//     pub fn new(_x:T, _y:T) -> Vector2<T> {
//         Vector2 { x:_x, y:_y } 
//     }

//     // norm/length/dot: floating-point only functions
//     pub fn norm2(self) -> T {
//         self.x.clone() * self.x + self.y.clone()* self.y
//     }
//     pub fn norm(self) -> T {
//         self.norm2()
//     }
//     pub fn length2(&self) -> T {
//         self.x.clone() * self.x.clone() + self.y.clone() * self.y.clone()
//     }
//     pub fn length(&self) -> T {
//         self.length2()
//     }
//     pub fn dot(self, v:Vector2<T>) -> T {
//         self.x * v.x + self.y * v.y
//     }
// }

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {

        let vec0 = Vector2::new(2_f32, 2_f32);
        let vec1 = Vector2::new(3_f32, 3_f32);
        let vec2 = vec0 + vec1;
        //let vec1 = Vector2::new(3, 3);
        //let vec2 = Vector2::new(5, 5);
        let vec_norm = vec2.norm2();
        assert_eq!(vec_norm, 50_f32);
    }
}

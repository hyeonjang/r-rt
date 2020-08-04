extern crate num_traits;

pub struct Vector2<T> {
    x : T, 
    y : T,
}

impl<T: std::clone::Clone + std::ops::Add<Output = T> + std::ops::Mul<Output = T>> Vector2<T> {
    pub fn new(_x:T, _y:T) -> Vector2<T> {
        Vector2 { x:_x, y:_y } 
    }
    pub fn norm2(&self) -> T {
        self.x.clone() * self.x.clone() + self.y.clone()* self.y.clone()
    }
    pub fn norm(&self) -> T {
        self.norm2()
    }
    pub fn length2(&self) -> T {
        self.x.clone() * self.x.clone() + self.y.clone()* self.y.clone()
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
        let vec1 = vec0.norm2();
        assert_eq!(vec1, 8);
    }
}

#![crate_type="lib"]
pub mod vector;
pub mod matrix;

#[cfg(test)]
mod tests {
    use super::*; 

    #[test]
    fn it_works() {

        let mat0 = Tmat2::new(2_f32, 2_f32);
        let vec1 = Tvec2::new(3_f32, 3_f32);
        let vec2 = mat0 + vec1;
        //let vec1 = tvec2::new(3, 3);
        //let vec2 = tvec2::new(5, 5);
        let vec_norm = vec2.norm2();
        assert_eq!(vec_norm, 50_f32);
    }
}
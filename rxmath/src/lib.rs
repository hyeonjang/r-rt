#![crate_type="lib"]

#[macro_use]
pub mod macros;
pub mod vector;
pub mod matrix;

#[cfg(test)]
#[allow(unused_imports)]
mod tests {
    use super::*; 
    use crate::macros::*;
    use crate::vector::*;
    use crate::matrix::*;

    #[test]
    fn it_works() {
        
        // 0. vector tests
        // 0.0 vector contruction test
        let v0:Fvec2 = vector::Gvec2::new(1_f32, 1_f32);
        let v1 = vec2(1_f32, 1_f32);
        assert_eq!(v0, v1);

        // 1. matrix tests
        // 1.0 matrix construction test
        let m0 = Gmat2::new(1_f32, 0_f32, 1_f32, 0_f32);
        let m1 = Gmat2::ident();
        assert_eq!(m0, m1);
    }
}
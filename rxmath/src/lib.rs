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

        // 0.1 vector operations tests
        let v2 = v0 + v1*2_f32;
        let v3 = v0/2.0 + v1;
        assert_eq!(v2, vec2(3_f32, 3_f32));
        assert_eq!(v3, vec2(1.5, 1.5));

        // 0.2 vector assignment operaions tests
        let mut v4 = vec2(1_f32, 1_f32);
        v4 *= 4_f32;
        assert_eq!(v4, vec2(4_f32, 4_f32));

        // 1. matrix tests
        // 1.0 matrix construction test
        let m0 = Gmat2::new(1_f32, 0_f32, 1_f32, 0_f32);
        let m1 = Gmat2::ident();
        assert_eq!(m0, m1);

        // 0.1 matrix operations tests
        let m2 = m0 + m1*2_f32;
        let m3 = m0/2.0 + m1;
        assert_eq!(m2, Gmat2::ident()*3_f32);
        assert_eq!(m3, Gmat2::ident()*1.5_f32);

        // 0.2 matrix assignment operaions tests
        let mut m4 = Gmat2::ident();
        m4 *= 4_f32;
        assert_eq!(m4, Gmat2::ident()*4_f32);
    }
}
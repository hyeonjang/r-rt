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
#![crate_type="lib"]
#![allow(unused_must_use)]
////////////////////////////////////
/// 0. const variable macro funtions
#[macro_export] macro_rules! INFI {
    ($t:ident) => {
        std::$t::INFINITY
    };
}
#[macro_export] macro_rules! PI {
    ($t:ident   ) => { std::$t::consts::PI };
    ($t:ident, 2) => { std::$t::consts::FRAC_PI_2 };
    ($t:ident, 3) => { std::$t::consts::FRAC_PI_3 };
    ($t:ident, 4) => { std::$t::consts::FRAC_PI_4 };
    ($t:ident, 6) => { std::$t::consts::FRAC_PI_6 };
    ($t:ident, 8) => { std::$t::consts::FRAC_PI_8 };
}

#[inline] pub fn degrees_to_radians(degrees:f32) -> f32 {
    degrees * PI!(f32) / 180.0
}

#[macro_use]
pub mod macros;
pub mod vector;
pub mod matrix;
pub mod random;

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
        let v0 = vector::Gvec2::new(1.0, 1.0);
        let v1 = vec2(1.0, 1.0);
        assert_eq!(v0, v1);

        // 0.1 vector operations tests
        let v2 = v0 + v1*2.0;
        let v3 = v0/2.0 + v1;
        assert_eq!(v2, vec2(3.0, 3.0));
        assert_eq!(v3, vec2(1.5, 1.5));

        // 0.2 vector assignment operaions tests
        let mut v4 = vec2(1.0, 1.0);
        v4 *= 4.0;
        assert_eq!(v4, vec2(4.0, 4.0));

        // 0.2.1 vector cross product test
        let v_c0 = vec3(2.0, 3.0, 4.0);
        let v_c1 = vec3(1.0, 2.0, 3.0);
        let v_l = cross(v_c0, v_c1); 
        let v_r = cross(-v_c1, v_c0);
        assert_eq!(v_l, v_r);

        // 0.3 vector functions tests
        let v5 = vec2(3.0, 4.0);
        let length_v5 = v5.length();
        assert_eq!(length_v5, 5.0);

        let v6 = vec3(2.0, 3.0, 5.0);
        let length2_v6 = 38.0;
        assert_eq!(v6.length2(), length2_v6);

        let v7 = vec3(2.0, 3.0, 5.0);
        let dot_v7 = 38.0;
        assert_eq!(v7.dot(v7), dot_v7);

        // 1. matrix tests
        // 1.0 matrix construction test
        let m0 = Gmat2::new(1.0, 0.0, 0.0, 1.0);
        let m1 = Gmat2::ident();
        assert_eq!(m0, m1);

        // 0.1 matrix operations tests
        let m2 = m0 + m1*2.0;
        let m3 = m0/2.0 + m1;
        assert_eq!(m2, Gmat2::ident()*3.0);
        assert_eq!(m3, Gmat2::ident()*1.5);

        // 0.2 matrix assignment operaions tests
        let mut m4 = Gmat2::ident();
        m4 *= 4.0;
        assert_eq!(m4, Gmat2::ident()*4.0);

        // 0.3 vector vs matrix operations
        let m5 = Gmat2::ident();
        let v6 = vec2(2.0, 2.0);
        let r0 : mat2 = m5 * v6; // left matrix, right vector
        let r1 : vec2 = v6 * m5; // left vector, right matrix

        assert_eq!(r0, Gmat2::ident()*2.0);
        assert_eq!(r1, v6);

        // 0.4 matrix functions operations
        let m6 = Gmat3::ident();
        let m8 = Gmat3::ident();
        let m7 = m6.transpose();
        assert_eq!(m8, m7);
    }
}
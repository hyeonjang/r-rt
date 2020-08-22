extern crate libc;
///////////////////////////////
/// 0. tupled array sturctures 
pub struct Garr2<T>(pub T, pub T);
pub struct Garr3<T>(pub T, pub T, pub T);
pub struct Garr4<T>(pub T, pub T, pub T);
///////////////////////////////
// 0. Vector Structures
pub struct Gvec2<T> { pub x:T, pub y:T }
pub struct Gvec3<T> { pub x:T, pub y:T, pub z : T }
pub struct Gvec4<T> { pub x:T, pub y:T, pub z : T, pub w:T }
//////////////////////////////
// 
// 0.1.0 Contructors & Copy, Clone
impl<T> Gvec2<T> {
    #[inline] pub fn new(_x:T, _y:T) -> Gvec2<T> {
        Gvec2{ x:_x, y:_y } 
    }
}
impl<T> Gvec3<T> {
    #[inline] pub fn new(_x:T, _y:T, _z:T) -> Gvec3<T> {
        Gvec3{ x:_x, y:_y, z:_z } 
    }
}
impl<T> Gvec4<T> {
    #[inline] pub fn new(_x:T, _y:T, _z:T, _w:T) -> Gvec4<T> {
        Gvec4{ x:_x, y:_y, z:_z, w:_w }
    }
}
// 0.1.1 Function Contructors for Convenience
pub fn vec2<T>(_x:T, _y:T) -> Gvec2<T> {
    Gvec2{ x:_x, y:_y }
}
pub fn vec3<T>(_x:T, _y:T, _z:T) -> Gvec3<T> {
    Gvec3{ x:_x, y:_y, z:_z }
}
pub fn vec4<T>(_x:T, _y:T, _z:T, _w:T) -> Gvec4<T> {
    Gvec4{ x:_x, y:_y, z:_z, w:_w }
}
// 0.0.2 Copy & Clone
impl_cpy!(Gvec2<T>{ x y });
impl_cpy!(Gvec3<T>{ x y z });
impl_cpy!(Gvec4<T>{ x y z w });

///////////////////////////
// 1. Operator overloading
// 1.1. ops
impl_ops!(Gvec2<T>{ x y });
impl_ops!(Gvec3<T>{ x y z });
impl_ops!(Gvec4<T>{ x y z w });
// 1.2 cmp
impl_cmp!(Gvec2<T>{ x y });
impl_cmp!(Gvec3<T>{ x y z });
impl_cmp!(Gvec4<T>{ x y z w });

////////////////////////////
/// 2. Vector features
// 2.1 Floating Point operation Methods Trait for Vectors
pub trait VecOp<RHS=Self> {
    type Output;
    fn norm2(self) -> f32;
    fn norm(self) -> f32;
    fn length2(self) -> f32;
    fn length(self) -> f32;
    fn dot(self, rhs:RHS) ->f32;
}
// 2.1.1 Gvec2
impl VecOp<Gvec2<f32>> for Gvec2<f32> {
    type Output = f32;
    #[inline] fn norm2(self) -> f32 {
        self.x.mul_add(self.x, self.y.clone()* self.y)
    }
    #[inline] fn norm(self) -> f32 {
        self.norm2().sqrt()
    }
    #[inline] fn length2(self) -> f32 {
        self.x.mul_add(self.x, self.y.clone()* self.y)
    }
    #[inline] fn length(self) -> f32 {
        self.length2().sqrt()
    }
    #[inline] fn dot(self,_v:Gvec2<f32>) -> f32 {
        self.x.mul_add(_v.x, self.y * _v.y)/self.length()
    }
}
// 2.1.2 Gvec3
impl VecOp<Gvec3<f32>> for Gvec3<f32> {
    type Output = f32;

    #[inline] fn norm2(self) -> f32 {
        self.x.mul_add(self.x, self.y.mul_add(self.y, self.z.clone()*self.z))
    }
    #[inline] fn norm(self) -> f32 {
        self.norm2().sqrt()
    }
    #[inline] fn length2(self) -> f32 {
        self.x.mul_add(self.x, self.y.mul_add(self.y, self.z.clone()*self.z))
    }
    #[inline] fn length(self) -> f32 {
        self.length2().sqrt()
    }
    #[inline] fn dot(self,_v:Gvec3<f32>) -> f32 {
        self.x.mul_add(_v.x, self.y.mul_add(_v.y, _v.z*self.z))/self.length()
    }
}
// 2.1.3 Gvec4
impl VecOp<Gvec4<f32>> for Gvec4<f32> {
    type Output = f32;

    #[inline] fn norm2(self) -> f32 {
        self.x.mul_add(self.x, self.y.mul_add(self.y, self.z.mul_add(self.z, self.w.clone()*self.z)))
    }
    #[inline] fn norm(self) -> f32 {
        self.norm2().sqrt()
    }
    #[inline] fn length2(self) -> f32 {
        self.x.mul_add(self.x, self.y.mul_add(self.y, self.z.mul_add(self.z, self.w.clone()*self.z)))
    }
    #[inline] fn length(self) -> f32 {
        self.length2().sqrt()
    }
    #[inline] fn dot(self,_v:Gvec4<f32>) -> f32 {
        self.x.mul_add(_v.x, self.y.mul_add(self.y, _v.z.mul_add(self.z, _v.w*self.z)))/self.length()
    }
}

/////////////////////////////////////
// 3. Final type aliasing?? <<aliasing is right expression?>>
pub type Fvec2 = Gvec2<f32>;
pub type Ivec2 = Gvec2<i32>;
pub type Uvec2 = Gvec2<u32>;
pub type Fvec3 = Gvec3<f32>;
pub type Ivec3 = Gvec3<i32>;
pub type Uvec3 = Gvec3<u32>;
pub type Fvec4 = Gvec4<f32>;
pub type Ivec4 = Gvec4<i32>;
pub type Uvec4 = Gvec4<u32>;
// exactly todo = array to vector copy

impl_fmt!(Fvec2{ x y }, "<{} {}>");
impl_fmt!(Ivec2{ x y }, "<{} {}>");
impl_fmt!(Uvec2{ x y }, "<{} {}>");
impl_fmt!(Fvec3{ x y z }, "<{} {} {}>");
impl_fmt!(Ivec3{ x y z }, "<{} {} {}>");
impl_fmt!(Uvec3{ x y z }, "<{} {} {}>");
impl_fmt!(Fvec4{ x y z w }, "<{} {} {} {}>");
impl_fmt!(Ivec4{ x y z w }, "<{} {} {} {}>");
impl_fmt!(Uvec4{ x y z w }, "<{} {} {} {}>");


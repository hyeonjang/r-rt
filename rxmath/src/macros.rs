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

#[macro_export]
macro_rules! impl_fmt {
    ($type:ty{ $($field:ident)+ }, $st :tt ) => {
        impl std::fmt::Display for $type {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(f, $st, $(self.$field,)*)
            }
        }
        impl std::fmt::Debug for $type {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.debug_struct(stringify!($type))
                 $(.field(stringify!($field), &self.$field))+
                 .finish()
            }
        }
    };
}
// overloading std::copy, clone operators
#[macro_export]
macro_rules! impl_cpy {
    ($type:ty{ $($field:ident)+ }) => {
       
        // 0. copy
        impl<T: Copy> Copy for $type {}

        impl<T: Clone> Clone for $type{ 
            fn clone(&self)->Self { 
                <$type>::new($(self.$field.clone(),)+)
            }
        }   
    }
}

// overloading std::cmp operators
#[macro_export]
macro_rules! impl_cmp {
    ($type:ty{ $first:ident $($others:ident)+}) => {
        impl<T: std::cmp::PartialEq> std::cmp::PartialEq for $type {
            fn eq(&self, other:&Self) -> bool {
                (&self.$first==&other.$first)$(&&(&self.$others==&other.$others))+
            }
        }
    };
}

// overloading std::ops operators
#[macro_export]
macro_rules!  impl_ops {
    ($type:ty { $($field:ident)+ }) => {
        
        // 0.  scalar vs type
        // 0.0 scalar vs type operations
        impl<T: Copy + std::ops::Mul<Output=T>> std::ops::Mul<T> for $type {
            type Output = Self;
            fn mul(self, rhs:T) -> $type {
                <$type>::new($(self.$field*rhs,)+)
            }
        }
        impl<T: Copy + std::ops::Div<Output=T>> std::ops::Div<T> for $type {
            type Output = $type;
            fn div(self, rhs:T) -> $type {
                <$type>::new($(self.$field/rhs,)+)
            }
        }
        // 0.1 scalar vs type assignment operations
        impl<T: Copy + std::ops::Mul<Output=T>> std::ops::MulAssign<T> for $type {
            fn mul_assign(&mut self, other:T) {
                *self = Self::new($(self.$field*other,)+)
            }
        } 
        impl<T: Copy + std::ops::Div<Output=T>> std::ops::DivAssign<T> for $type {
            fn div_assign(&mut self, other:T) {
                *self = Self::new($(self.$field/other,)+)
            }
        } 
       
        // 1.  type vs type
        // 1.1 type vs type operations
        impl<T: std::ops::Neg<Output=T>> std::ops::Neg for $type {
            type Output = $type;
            fn neg(self) -> $type {
                <$type>::new($(-self.$field,)+)
            }
        }
        impl<T: std::ops::Add<Output=T>> std::ops::Add<$type> for $type {
            type Output = $type;
            fn add(self, rhs: $type) -> $type {
                <$type>::new($(self.$field+rhs.$field,)+)
            }
        }
        impl<T: std::ops::Sub<Output=T>> std::ops::Sub<$type> for $type {
            type Output = $type;
            fn sub(self, rhs: $type) -> $type {
                <$type>::new($(self.$field-rhs.$field,)+)
            }
        }
        impl<T: std::ops::Mul<Output=T>> std::ops::Mul<$type> for $type {
            type Output = $type;
            fn mul(self, rhs:$type) -> $type {
                <$type>::new($(self.$field*rhs.$field,)+)
             }
        } 
        // 1.2 type vs type assignment operations
        impl<T: Copy + std::ops::Add<Output=T>> std::ops::AddAssign for $type {
            fn add_assign(&mut self, other:Self) {
                *self = Self::new($(self.$field+other.$field,)+)
            }
        }
        impl<T: Copy + std::ops::Sub<Output=T>> std::ops::SubAssign for $type {
            fn sub_assign(&mut self, other:Self) {
                *self = Self::new($(self.$field-other.$field,)+)
            }
        }
    };
}

// overloading std::ops operators
#[macro_export]
macro_rules! impl_idx {
    ($type:ty { $($field:ident $li:literal)+ }) => {
        impl<T> std::ops::Index<usize> for $type {
            type Output = T;
            fn index<'a>(&'a self, i:usize) -> &T {
                match i {
                    $($li => &self.$field,)+
                    _ => &&self.x,
                }
            }
        }
    };
}
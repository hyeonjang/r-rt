// //@@todo 0. objective = generic vector type sturcture and array 1. script macro for vector operation 2. implement traits
#[macro_export]
macro_rules! impl_fmt {
    ($type:ty{$($field:ident)+}, $st :tt ) => {
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


//@@todo implement convienient sturcture contructors
// vec mat 















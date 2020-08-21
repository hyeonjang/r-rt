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

//@@todo implement convienient sturcture contructors
// vec mat 















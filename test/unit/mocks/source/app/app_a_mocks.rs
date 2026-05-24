#![allow(ambiguous_glob_reexports)]

pub mod mock_lib_b {
    use crate::app_a::*;
    include!("../../lib/mock_lib_b.rs");
}
pub use mock_lib_b::*;


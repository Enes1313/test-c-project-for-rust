#![allow(ambiguous_glob_reexports)]

pub mod mock_lib_example {
    use crate::app_example::*;
    include!("../../lib/mock_lib_example.rs");
}
pub use mock_lib_example::*;

pub mod mock_util_example {
    use crate::app_example::*;
    include!("../../source/util/mock_util_example.rs");
}
pub use mock_util_example::*;


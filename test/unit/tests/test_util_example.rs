#![feature(c_variadic)]
#![allow(unused_mut, unused_variables, unused_assignments, unused_imports, non_snake_case)]
#[allow(non_upper_case_globals, non_camel_case_types, non_snake_case, unused)]
#[path = "../bindings/source/util/util_example.rs"]
pub mod util_example;
use util_example::*;

#[cfg(test)]
mod util_sum {
    use super::*;

    #[test]
    fn sum__success() {
        // Pre Actions
        let mut result: ::core::ffi::c_int = 0;

        // Test Steps
        unsafe {
            // Step 1
            result = util_example_sum(2, 7, 5);

            // Step 2
            assert_eq!(result, 12);
        }
    }
}

#[cfg(test)]
mod util_mult {
    use super::*;

    #[test]
    fn multiplication__success() {
        // Pre Actions
        let mut b: bool = false;
        let mut out: ::core::ffi::c_int = 0;

        // Test Steps
        unsafe {
            // Step 1
            b = util_example_mult(5, 7, &mut out);

            // Step 2
            assert_eq!(b, true);
            
            // Step 3
            assert_eq!(out, 35);
        }
    }
}

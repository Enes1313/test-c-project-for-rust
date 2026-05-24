#![feature(c_variadic)]
#[allow(non_upper_case_globals, non_camel_case_types, non_snake_case, unused)]
#[path = "../bindings/source/app/app_example.rs"]
pub mod app_example;
use app_example::*;

#[allow(non_snake_case, unused)]
#[path = "../mocks/source/app/app_example_mocks.rs"]
pub mod mocks;

#[cfg(test)]
mod app_tests {
    use super::*;

    #[test]
    fn test_app_run_uninitialized() {
        // Test Steps
        unsafe {
            // Step 1
            let res = app_example_run();

            // Step 2
            assert_eq!(res, 1);
        }
    }
}

#![feature(c_variadic)]
#[allow(non_upper_case_globals, non_camel_case_types, non_snake_case, unused)]
#[path = "../bindings/source/app/app_example.rs"]
pub mod app_example;
use app_example::*;

#[allow(non_snake_case, unused)]
#[path = "../mocks/source/app/app_example_mocks.rs"]
pub mod mocks;
use mocks::*;

#[cfg(test)]
mod app_tests {
    use super::*;

    #[test]
    fn test_app_init() {
        // Pre Actions
        let ctx = lib_example_init_context();
        ctx.expect().once().returning(|| true);

        // Test Steps
        unsafe {
            // Step 1
            app_example_init();
        }
    }
}

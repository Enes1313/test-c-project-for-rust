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
        let ctx_init = lib_example_init_context();
        ctx_init.expect().once().returning(|| true);

        // Test Steps
        unsafe {
            // Step 1
            app_example_init();
        }

        // Post Actions
        let ctx_deinit = lib_example_deinit_context();
        ctx_deinit.expect().returning(|| ());
        unsafe { app_example_deinit(); }
    }
    
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
    
    #[test]
    fn test_app_run_first_iteration() {
        // Pre Actions
        let ctx_init = lib_example_init_context();
        ctx_init.expect().once().returning(|| true);
        unsafe { app_example_init(); }

        // Expected Calls
        let ctx_run = lib_example_show_int32_context();
        ctx_run.expect().once().with(mockall::predicate::eq(13)).returning(|_| true);

        // Test Steps
        unsafe {
            // Step 1
            let res = app_example_run();

            // Step 2
            assert_eq!(res, 0);
        }

        // Post Actions
        let ctx_deinit = lib_example_deinit_context();
        ctx_deinit.expect().returning(|| ());
        unsafe { app_example_deinit(); }
    }
}

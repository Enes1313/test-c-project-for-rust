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
    fn test_app_run_first_iteration() {
        // Setup state for this test ONLY
        {
            let ctx = lib_example_init_context();
            ctx.expect().once().returning(|| true);
            unsafe { app_example_init(); }
        }
        
        // Execute the run
        {
            let ctx = lib_example_show_int32_context();
            ctx.expect().once().with(mockall::predicate::eq(13)).returning(|_| true);
            
            unsafe {
                let res = app_example_run();
                assert_eq!(res, 0);
            }
        }
    }
}

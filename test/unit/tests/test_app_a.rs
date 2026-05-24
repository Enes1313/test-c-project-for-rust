#[allow(non_upper_case_globals, non_camel_case_types, non_snake_case, unused)]
#[path = "../bindings/source/app/app_a.rs"]
pub mod app_a;
use app_a::*;

#[allow(non_snake_case, unused)]
#[path = "../mocks/source/app/app_a_mocks.rs"]
pub mod mocks;
use mocks::*;

#[cfg(test)]
mod scenario_tests {
    use super::*;

    #[test]
    fn test_type_collision_solved() {
        // We create the struct using the shared type!
        let mut my_data = lib_b_complex_type_t {
            id: 42,
            is_active: false,
        };

        // We set up a mock expectation on b_do_something
        let ctx = lib_b_do_something_context();
        
        let expected_ptr = &mut my_data as *mut _;
        let expected_addr = expected_ptr as usize;

        // NO MORE COMPILER ERRORS! The types match perfectly!
        ctx.expect().once().withf(move |ptr| (*ptr as usize) == expected_addr).returning(|_| ());

        unsafe {
            // Calling the A function!
            app_a_init_with_lib_b(&mut my_data);
        }
    }
}

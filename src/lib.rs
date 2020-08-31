#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
include!("c_bindings.rs");

// Enable this for running tests
// Requires unstable compiler versions
//#![feature(is_sorted)]

pub mod sort;
pub mod macros;
mod tests;

pub fn generate_test_array(length: usize) -> Vec<i32> {
    (0..length).map(|_| rand::random()).collect()
}

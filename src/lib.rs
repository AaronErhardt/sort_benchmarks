#![feature(test)]
#![feature(is_sorted)]
#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

include!("bindings.rs");

extern crate test;

mod bench;
mod macros;
mod sort;
mod tests;

pub fn generate_test_arrays(size: usize, length: usize) -> Vec<Vec<i32>> {
    //let mut rng = rand::thread_rng();

    let mut arrays = Vec::with_capacity(size);

    for _ in 0..size {
        arrays.push((0..length).map(|_| rand::random()).collect());
    }

    arrays
}

pub fn is_sorted(arr: &[i32]) -> bool {
    arr.is_sorted()
}

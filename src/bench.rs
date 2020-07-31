use crate::macros::MAX_EXPECTED_ITERATIONS;
#[cfg(test)]
use crate::sort::*;
use crate::*;
use test::bench::{black_box, Bencher};

#[bench]
fn c_selection_sort_bench_10(b: &mut Bencher) {
    benchmark_c!(b, selection_sort_c, 10);
}

#[bench]
fn c_selection_sort_bench_100(b: &mut Bencher) {
    benchmark_c!(b, selection_sort_c, 100);
}

#[bench]
fn c_selection_sort_bench_1000(b: &mut Bencher) {
    benchmark_c!(b, selection_sort_c, 1000);
}

/*#[bench]
fn c_selection_sort_bench_10000(b: &mut Bencher) {
    benchmark_c!(b, selection_sort_c, 10000);
}*/

#[bench]
fn rs_selection_sort_bench_10(b: &mut Bencher) {
    benchmark_rs!(b, selection_sort_rs, 10);
}

#[bench]
fn rs_selection_sort_bench_100(b: &mut Bencher) {
    benchmark_rs!(b, selection_sort_rs, 100);
}

#[bench]
fn rs_selection_sort_bench_1000(b: &mut Bencher) {
    benchmark_rs!(b, selection_sort_rs, 1000);
}

/*#[bench]
fn rs_selection_sort_bench_10000(b: &mut Bencher) {
    benchmark_rs!(b, selection_sort_rs, 10000);
}*/

#[bench]
fn rs_native_sort_bench_10(b: &mut Bencher) {
    benchmark_rs_native!(b, sort, 10);
}

#[bench]
fn rs_native_sort_bench_100(b: &mut Bencher) {
    benchmark_rs_native!(b, sort, 100);
}

#[bench]
fn rs_native_sort_bench_1000(b: &mut Bencher) {
    benchmark_rs_native!(b, sort, 1000);
}

/*#[bench]
fn rs_native_sort_bench_10000(b: &mut Bencher) {
    benchmark_rs_native!(b, sort, 10000);
}*/

#[bench]
fn rs_native_sort_unstable_bench_10(b: &mut Bencher) {
    benchmark_rs_native!(b, sort_unstable, 10);
}

#[bench]
fn rs_native_sort_unstable_bench_100(b: &mut Bencher) {
    benchmark_rs_native!(b, sort_unstable, 100);
}

#[bench]
fn rs_native_sort_unstable_bench_1000(b: &mut Bencher) {
    benchmark_rs_native!(b, sort_unstable, 1000);
}

/*#[bench]
fn rs_native_sort_unstable_bench_10000(b: &mut Bencher) {
    benchmark_rs_native!(b, sort_unstable, 10000);
}*/

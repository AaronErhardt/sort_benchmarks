/* bench.rs
 *
 * Copyright (c) 2020 Aaron Erhardt
 *
 * This program is free software: you can redistribute it and/or modify
 * it under the terms of the GNU General Public License as published by
 * the Free Software Foundation, either version 3 of the License, or
 * (at your option) any later version.
 *
 * This program is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU General Public License for more details.
 *
 * You should have received a copy of the GNU General Public License
 * along with this program.  If not, see <http://www.gnu.org/licenses/>.
 */

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

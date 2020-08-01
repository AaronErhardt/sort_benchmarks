/* macros.rs
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


pub const MAX_EXPECTED_ITERATIONS: usize = 400000000;

#[macro_export]
macro_rules! benchmark_c {
    ( $bench: ident, $fn:ident, $iterations:expr ) => {

        // The amount of benchmarks can't be limited with the current API so enough arrays need to be allocated
        let mut arrays = generate_test_arrays(MAX_EXPECTED_ITERATIONS/$iterations, $iterations);
        let mut index = 0;

        $bench.iter(|| {
            unsafe {
                $fn(arrays[index].as_mut_ptr(), arrays[index].len() as i32);
            }
            index += 1;
        });
    };
}

#[macro_export]
macro_rules! test_c {
    ( $fn:ident ) => {
        let mut arrays = generate_test_arrays(3, 1000);

        for arr in arrays.iter_mut() {
            unsafe {
                $fn(arr.as_mut_ptr(), arr.len() as i32);
            }
        }

        for arr in arrays {
            assert!(arr.as_slice().is_sorted());
        }
    };
}

#[macro_export]
macro_rules! benchmark_rs {
    ( $bench: ident, $fn:ident, $iterations:expr ) => {

        // The amount of benchmarks can't be limited with the current API so enough arrays need to be allocated
        let mut arrays = generate_test_arrays(MAX_EXPECTED_ITERATIONS/$iterations, $iterations);
        let mut index = 0;

        $bench.iter(|| {
            $fn(arrays[index].as_mut_slice());
            index += 1;
        });
    };
}

#[macro_export]
macro_rules! test_rs {
    ( $fn:ident ) => {
        let mut arrays = generate_test_arrays(3, 1000);

        for arr in arrays.iter_mut() {
            $fn(arr);
        }

        for arr in arrays {
            assert!(arr.as_slice().is_sorted());
        }
    };
}

#[macro_export]
macro_rules! benchmark_rs_native {
    ( $bench: ident, $fn:ident, $iterations:expr ) => {

        // The amount of benchmarks can't be limited with the current API so enough arrays need to be allocated
        let mut arrays = generate_test_arrays(MAX_EXPECTED_ITERATIONS/$iterations, $iterations);
        let mut index = 0;

        $bench.iter(|| {
            arrays[index].$fn();
            index += 1;
        });
    };
}

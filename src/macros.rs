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

#[macro_export]
macro_rules! test_c {
    ( $fn:ident ) => {

        let mut arr = generate_test_array(1000);
        unsafe {
            $fn(arr.as_mut_ptr(), arr.len() as i32);
        }

        assert!(arr.as_slice().is_sorted());

    };
}

#[macro_export]
macro_rules! test_rs {
    ( $fn:ident ) => {

        let mut arr = generate_test_array(1000);
        $fn(arr.as_mut_slice());

        assert!(arr.as_slice().is_sorted());
    };
}

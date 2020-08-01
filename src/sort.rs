/* sort.rs
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


pub fn selection_sort_rs(arr: &mut [i32]) {
    let mut min: usize;
    let len = arr.len();
    for i in 1..len {
        min = i - 1;
        for j in i..len {
            unsafe {
                if arr.get_unchecked(min) > arr.get_unchecked(j) {
                    min = j;
                }
            }
        }
        arr.swap(i - 1, min);
    }
}

use crate::sort::selection_sort_rs;
#[cfg(test)]
use crate::{generate_test_arrays, selection_sort_c, test_c, test_rs};

#[test]
fn c_selection_sort_test() {
    test_c!(selection_sort_c);
}

#[test]
fn rs_selection_sort_test() {
    test_rs!(selection_sort_rs);
}

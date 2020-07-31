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

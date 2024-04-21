/// Sorts the elements of the slice in non-decreasing order using the quick sort algorithm.
pub fn quick_sort<T: Ord>(arr: &mut [T]) {
    if arr.len() <= 1 {
        return;
    }
    let pivot = partition(arr);
    quick_sort(&mut arr[..pivot]);
    quick_sort(&mut arr[pivot + 1..]);
}

fn partition<T: Ord>(arr: &mut [T]) -> usize {
    let pivot_index = arr.len() / 2;
    arr.swap(pivot_index, arr.len() - 1);
    let mut i = 0;
    for j in 0..arr.len() - 1 {
        if arr[j] <= arr[arr.len() - 1] {
            arr.swap(i, j);
            i += 1;
        }
    }
    arr.swap(i, arr.len() - 1);
    i
}

/// Sorts the elements of the slice in non-decreasing order using the selection sort algorithm.
pub fn selection_sort<T: Ord>(arr: &mut [T]) {
    for i in 0..arr.len() {
        let mut min_index = i;
        for j in i + 1..arr.len() {
            if arr[j] < arr[min_index] {
                min_index = j;
            }
        }
        arr.swap(i, min_index);
    }
}

/// Sorts the elements of the slice in non-decreasing order using the insertion sort algorithm.
pub fn insertion_sort<T: Ord>(arr: &mut [T]) {
    for i in 1..arr.len() {
        let mut j = i;
        while j > 0 && arr[j - 1] > arr[j] {
            arr.swap(j - 1, j);
            j -= 1;
        }
    }
}

/// Sorts the elements of the slice in non-decreasing order using the merge sort algorithm.
pub fn merge_sort<T: Ord + Clone>(arr: &mut [T]) {
    if arr.len() <= 1 {
        return;
    }
    let mid = arr.len() / 2;
    merge_sort(&mut arr[..mid]);
    merge_sort(&mut arr[mid..]);
    let mut result = Vec::with_capacity(arr.len());
    merge(&arr[..mid], &arr[mid..], &mut result);
    arr.clone_from_slice(&result);
}

fn merge<T: Ord + Clone>(left: &[T], right: &[T], result: &mut Vec<T>) {
    let mut l = 0;
    let mut r = 0;
    while l < left.len() && r < right.len() {
        if left[l] <= right[r] {
            result.push(left[l].clone());
            l += 1;
        } else {
            result.push(right[r].clone());
            r += 1;
        }
    }
    result.extend_from_slice(&left[l..]);
    result.extend_from_slice(&right[r..]);
}

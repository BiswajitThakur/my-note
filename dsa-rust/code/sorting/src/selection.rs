#[allow(dead_code)]
fn selection_sort<T: PartialOrd>(arr: &mut [T]) {
    for i in 0..arr.len() - 1 {
        let mut min_idx = i;
        for j in i + 1..arr.len() {
            if arr[j] < arr[min_idx] {
                min_idx = j;
            }
        }
        arr.swap(min_idx, i);
    }
}

#[cfg(test)]
mod tests {
    use super::selection_sort;

    #[test]
    fn test_selection_sort_int() {
        let mut arr = vec![9, 8, 7, 6, 5, 4, 3, 2, 1];
        selection_sort(arr.as_mut_slice());
        assert_eq!(arr, vec![1, 2, 3, 4, 5, 6, 7, 8, 9]);
    }
    #[test]
    fn test_selection_sort_float() {
        let mut arr = vec![7.5, 0.0, 1.3, 1.0, -33.8, 45.0, 20.3];
        selection_sort(arr.as_mut_slice());
        assert_eq!(arr, vec![-33.8, 0.0, 1.0, 1.3, 7.5, 20.3, 45.0]);
    }
}

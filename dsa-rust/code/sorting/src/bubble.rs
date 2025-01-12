#[allow(dead_code)]
fn bubble_sort<T: PartialOrd>(arr: &mut [T]) {
    let len = arr.len();
    for i in 0..len {
        let mut flag = true;
        for j in 0..(len - i - 1) {
            if arr[j] > arr[j + 1] {
                flag = false;
                arr.swap(j, j + 1);
            }
        }
        if flag {
            break;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::bubble_sort;

    #[test]
    fn test_bubble_sort_int() {
        let mut arr = vec![9, 8, 7, 6, 5, 4, 3, 2, 1];
        bubble_sort(arr.as_mut_slice());
        assert_eq!(arr, vec![1, 2, 3, 4, 5, 6, 7, 8, 9]);
    }
    #[test]
    fn test_bubble_sort_float() {
        let mut arr = vec![7.5, 0.0, 1.3, 1.0, -33.8, 45.0, 20.3];
        bubble_sort(arr.as_mut_slice());
        assert_eq!(arr, vec![-33.8, 0.0, 1.0, 1.3, 7.5, 20.3, 45.0]);
    }
}

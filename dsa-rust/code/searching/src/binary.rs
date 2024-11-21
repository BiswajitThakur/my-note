pub fn binary_search<T: PartialEq + PartialOrd>(arr: &[T], value: &T) -> Option<usize> {
    if arr.is_empty() {
        return None;
    }
    let mut left = 0;
    let mut right = arr.len() - 1;
    while left <= right {
        let mid = (left + right) / 2;
        if &arr[mid] == value {
            return Some(mid);
        }
        if &arr[mid] > value {
            if mid == 0 {
                break; // Prevents underflow when mid is 0
            }
            right = mid - 1;
        } else {
            left = mid + 1;
        }
    }
    None
}

#[test]
fn test_binary_search() {
    let arr = [1, 3, 5, 7, 9, 11, 15];
    assert_eq!(binary_search(&arr, &1), Some(0));
    assert_eq!(binary_search(&arr, &3), Some(1));
    assert_eq!(binary_search(&arr, &5), Some(2));
    assert_eq!(binary_search(&arr, &7), Some(3));
    assert_eq!(binary_search(&arr, &9), Some(4));
    assert_eq!(binary_search(&arr, &11), Some(5));
    assert_eq!(binary_search(&arr, &15), Some(6));
    assert_eq!(binary_search(&arr, &0), None);
    assert_eq!(binary_search(&arr, &-1), None);
    assert_eq!(binary_search(&arr, &-3), None);

    let arr1 = [-9.0, -5.3, 1.3, 1.5, 2.5, 5.3, 77.8];
    assert_eq!(binary_search(&arr1, &-9.0), Some(0));
    assert_eq!(binary_search(&arr1, &-5.3), Some(1));
    assert_eq!(binary_search(&arr1, &1.3), Some(2));
    assert_eq!(binary_search(&arr1, &1.5), Some(3));
    assert_eq!(binary_search(&arr1, &2.5), Some(4));
    assert_eq!(binary_search(&arr1, &77.8), Some(6));
    assert_eq!(binary_search(&arr1, &0.0), None);
    assert_eq!(binary_search(&arr1, &1.4), None);
}

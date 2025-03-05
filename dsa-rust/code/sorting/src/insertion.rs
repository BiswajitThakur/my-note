#[allow(unused)]
fn insertion_sort<T: PartialOrd>(arr: &mut [T]) {
    for i in 1..arr.len() {
        let key = &arr[i];
        let mut j = i - 1;
        while j >= 0 && &arr[j] > &arr[i] {
            //arr[j + 1] = arr[j];
            if let Some(v) = arr.get_mut(j + 1) {
                *v = arr[j];
            }
            j = j - 1;
        }
        //arr[j + 1] = key;
        arr.swap(i, j + 1);
    }
}

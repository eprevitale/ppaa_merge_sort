pub fn merge_sort(data: &mut Vec<i32>) -> (usize, usize, f64) {
    let start = std::time::Instant::now();
    let mut comparisons = 0;
    let mut swaps = 0;

    merge_sort_recursive(data, &mut comparisons, &mut swaps);

    let duration = start.elapsed().as_secs_f64() * 1000.0;
    (comparisons, swaps, duration)
}

fn merge_sort_recursive(data: &mut Vec<i32>, comparisons: &mut usize, swaps: &mut usize) {
    let n = data.len();
    if n <= 1 {
        return;
    }

    let mid = n / 2;
    let mut left = data[0..mid].to_vec();
    let mut right = data[mid..].to_vec();

    merge_sort_recursive(&mut left, comparisons, swaps);
    merge_sort_recursive(&mut right, comparisons, swaps);

    let mut i = 0;
    let mut j = 0;
    let mut k = 0;

    while i < left.len() && j < right.len() {
        *comparisons += 1;
        if left[i] <= right[j] {
            data[k] = left[i];
            i += 1;
        } else {
            data[k] = right[j];
            j += 1;
        }
        k += 1;
        *swaps += 1;
    }

    while i < left.len() {
        data[k] = left[i];
        i += 1;
        k += 1;
        *swaps += 1;
    }

    while j < right.len() {
        data[k] = right[j];
        j += 1;
        k += 1;
        *swaps += 1;
    }
}

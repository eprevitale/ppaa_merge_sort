pub fn quick_sort(data: &mut Vec<i32>) -> (usize, usize, f64) {
    let start = std::time::Instant::now();
    let mut comparisons = 0;
    let mut swaps = 0;

    quick_sort_recursive(data, 0, data.len() - 1, &mut comparisons, &mut swaps);

    let duration = start.elapsed().as_secs_f64() * 1000.0;
    (comparisons, swaps, duration)
}

fn quick_sort_recursive(data: &mut Vec<i32>, low: usize, high: usize, comparisons: &mut usize, swaps: &mut usize) {
    if low < high {
        let pivot_idx = partition(data, low, high, comparisons, swaps);
        if pivot_idx > 0 {
            quick_sort_recursive(data, low, pivot_idx - 1, comparisons, swaps);
        }
        quick_sort_recursive(data, pivot_idx + 1, high, comparisons, swaps);
    }
}

fn partition(data: &mut Vec<i32>, low: usize, high: usize, comparisons: &mut usize, swaps: &mut usize) -> usize {
    let pivot = data[high];
    let mut i = low;

    for j in low..high {
        *comparisons += 1;
        if data[j] <= pivot {
            data.swap(i, j);
            *swaps += 1;
            i += 1;
        }
    }

    data.swap(i, high);
    *swaps += 1;
    i
}

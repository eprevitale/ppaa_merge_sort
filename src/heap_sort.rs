pub fn heap_sort(data: &mut Vec<i32>) -> (usize, usize, f64) {
    let start = std::time::Instant::now();
    let mut comparisons = 0;
    let mut swaps = 0;

    let n = data.len();

    for i in (0..n / 2).rev() {
        heapify(data, n, i, &mut comparisons, &mut swaps);
    }

    for i in (1..n).rev() {
        data.swap(0, i);
        swaps += 1;
        heapify(data, i, 0, &mut comparisons, &mut swaps);
    }

    let duration = start.elapsed().as_secs_f64() * 1000.0;
    (comparisons, swaps, duration)
}

fn heapify(data: &mut Vec<i32>, n: usize, i: usize, comparisons: &mut usize, swaps: &mut usize) {
    let mut largest = i;
    let left = 2 * i + 1;
    let right = 2 * i + 2;

    if left < n {
        *comparisons += 1;
        if data[left] > data[largest] {
            largest = left;
        }
    }

    if right < n {
        *comparisons += 1;
        if data[right] > data[largest] {
            largest = right;
        }
    }

    if largest != i {
        data.swap(i, largest);
        *swaps += 1;
        heapify(data, n, largest, comparisons, swaps);
    }
}

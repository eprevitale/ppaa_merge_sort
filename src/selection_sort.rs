pub fn selection_sort(data: &mut Vec<i32>) -> (usize, usize, f64) {
    let start = std::time::Instant::now();
    let mut comparisons = 0;
    let mut swaps = 0;

    let n = data.len();
    for i in 0..n {
        let mut min_idx = i;
        for j in (i + 1)..n {
            comparisons += 1;
            if data[j] < data[min_idx] {
                min_idx = j;
            }
        }
        if min_idx != i {
            data.swap(i, min_idx);
            swaps += 1;
        }
    }

    let duration = start.elapsed().as_secs_f64() * 1000.0;
    (comparisons, swaps, duration)
}

pub fn bubble_sort(data: &mut Vec<i32>) -> (usize, usize, f64) {
    let start = std::time::Instant::now();
    let mut comparisons = 0;
    let mut swaps = 0;

    let n = data.len();
    for i in 0..n {
        for j in 0..(n - i - 1) {
            comparisons += 1;
            if data[j] > data[j + 1] {
                data.swap(j, j + 1);
                swaps += 1;
            }
        }
    }

    let duration = start.elapsed().as_secs_f64() * 1000.0; // Convert to milliseconds
    (comparisons, swaps, duration)
}

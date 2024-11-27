pub fn insertion_sort(data: &mut Vec<i32>) -> (usize, usize, f64) {
    let start = std::time::Instant::now();
    let mut comparisons = 0;
    let mut swaps = 0;

    let n = data.len();
    for i in 1..n {
        let mut j = i;
        while j > 0 && data[j - 1] > data[j] {
            comparisons += 1;
            data.swap(j, j - 1);
            swaps += 1;
            j -= 1;
        }
        if j > 0 {
            comparisons += 1; // Final comparison in the while loop
        }
    }

    let duration = start.elapsed().as_secs_f64() * 1000.0;
    (comparisons, swaps, duration)
}

use rand::Rng;

mod bubble_sort;
mod selection_sort;
mod insertion_sort;
mod merge_sort;
mod quick_sort;
mod heap_sort;

fn generate_random_vector(size: usize) -> Vec<i32> {
    let mut rng = rand::thread_rng();
    (0..size).map(|_| rng.gen_range(1..100_000)).collect()
}

fn main() {
    let sizes = [1000, 10_000, 100_000];

    println!("{:20} | {:10} | {:10} | {:10} | {:10}", "Algoritmo", "Tamanho (n)", "Tempo (ms)", "Trocas", "Comparações");
    println!("{}", "-".repeat(70));

    for &size in &sizes {
        let original = generate_random_vector(size);

        let mut data = original.clone();
        let (comparisons, swaps, time) = bubble_sort::bubble_sort(&mut data);
        println!("{:20} | {:10} | {:10.2} | {:10} | {:10}", "Bubble Sort", size, time, swaps, comparisons);

        let mut data = original.clone();
        let (comparisons, swaps, time) = selection_sort::selection_sort(&mut data);
        println!("{:20} | {:10} | {:10.2} | {:10} | {:10}", "Selection Sort", size, time, swaps, comparisons);

        let mut data = original.clone();
        let (comparisons, swaps, time) = insertion_sort::insertion_sort(&mut data);
        println!("{:20} | {:10} | {:10.2} | {:10} | {:10}", "Insertion Sort", size, time, swaps, comparisons);

        let mut data = original.clone();
        let (comparisons, swaps, time) = merge_sort::merge_sort(&mut data);
        println!("{:20} | {:10} | {:10.2} | {:10} | {:10}", "Merge Sort", size, time, swaps, comparisons);

        let mut data = original.clone();
        let (comparisons, swaps, time) = quick_sort::quick_sort(&mut data);
        println!("{:20} | {:10} | {:10.2} | {:10} | {:10}", "Quick Sort", size, time, swaps, comparisons);

        let mut data = original.clone();
        let (comparisons, swaps, time) = heap_sort::heap_sort(&mut data);
        println!("{:20} | {:10} | {:10.2} | {:10} | {:10}", "Heap Sort", size, time, swaps, comparisons);
    }
}

fn merge_sort(arr: &mut [i32]) {
    let len: usize = arr.len();
    if len <= 1 {
        return;
    }

    let mid: usize = len / 2;
    let mut left: Vec<i32> = arr[..mid].to_vec();
    let mut right: Vec<i32> = arr[mid..].to_vec();

    merge_sort(&mut left);
    merge_sort(&mut right);

    merge(arr, &left, &right);
}

fn merge(arr: &mut [i32], left: &[i32], right: &[i32]) {
    let mut i: usize = 0;
    let mut j: usize = 0;
    let mut k: usize = 0;

    while i < left.len() && j < right.len() {
        if left[i] <= right[j] {
            arr[k] = left[i];
            i += 1;
        } else {
            arr[k] = right[j];
            j += 1;
        }
        k += 1;
    }

    while i < left.len() {
        arr[k] = left[i];
        i += 1;
        k += 1;
    }

    while j < right.len() {
        arr[k] = right[j];
        j += 1;
        k += 1;
    }
}

fn main() {
    let mut arr: [i32; 7] = [38, 27, 43, 3, 9, 82, 10];
    println!("Antes da ordenação: {:?}", arr);

    merge_sort(&mut arr);

    println!("Depois da ordenação: {:?}", arr);
}

pub fn run() {
    println!("############### Bubble Sort in Rust ###############");

    let data: Vec<i32> = vec![9, 8, 7, 6, 5, 4, 3, 2, 1];
    println!("Unsorted data: {:?}", data);

    let sorted_data = bubble_sort(data);
    println!("Sorted data: {:?}", sorted_data);

    println!("Inplace Bubble Sort");
    let data: Vec<i32> = vec![9, 8, 7, 6, 5, 4, 3, 2, 1];
    bubble_sort_inplace(&mut data.clone());
}

fn bubble_sort(mut data: Vec<i32>) -> Vec<i32> {
    let mut sorted = false;

    while !sorted {
        sorted = true; // Assume sorted initially
        for i in 0..data.len() - 1 {
            if data[i] > data[i + 1] {
                data.swap(i, i + 1);
                sorted = false; // If we swap, it's not sorted
            }
        }
    }

    data
}

fn bubble_sort_inplace(data: &mut [i32]) {
    let mut n = data.len();
    while n > 1 {
        let mut new_n = 0;
        for i in 0..n - 1 {
            if data[i] > data[i + 1] {
                data.swap(i, i + 1);
                new_n = i + 1; // Record the last swap position
            }
        }
        n = new_n; // Reduce the range to the last unsorted element
    }

    println!("Inplace bubble sorted data: {:?}", data);
}

pub fn run() {
    println!("############### Bubble Sort in Rust ###############");

    let data: Vec<i32> = vec![9, 8, 7, 6, 5, 4, 3, 2, 1];
    println!("Unsorted data: {:?}", data);

    let sorted_data = bubble_sort(data);
    println!("Sorted data: {:?}", sorted_data);
}

// SUDOCODE:
// 1. Iterate throught the data and keep a pointer for each
// iteration to see if the value of the first pointer is less
// then the second if this condition hits swap the values do it again
// till all the values of the array are sorted
// 2. keep a pointer to track that all the values have been sorted
// 3. run a a while loop till the flag that all values are same doesnt turn true
//

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

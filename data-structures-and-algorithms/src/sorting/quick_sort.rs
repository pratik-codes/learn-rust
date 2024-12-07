// Quick Sort is a highly efficient divide-and-conquer sorting algorithm. It works by selecting a "pivot" element from the array and partitioning the other elements into two groups:
//
// Elements less than or equal to the pivot.
// Elements greater than the pivot.
// These groups are then sorted recursively.
//
// Steps of Quick Sort
// Choose a pivot: Select an element from the array (commonly the last, first, or a random element).
// Partition the array: Rearrange the array so that elements smaller than the pivot go to its left, and larger elements go to its right.
// Recursively apply quick sort:
// Sort the left subarray.
// Sort the right subarray.
// The recursion ends when the array has one or zero elements, as these are already sorted.
//
// Example
// Given an array [10, 7, 8, 9, 1, 5]:
//
// Choose 5 as the pivot.
// Partition: [1, 5, 7, 8, 9, 10].
// Recursively sort [1] (left) and [7, 8, 9, 10] (right).
//
// Why Use Quick Sort?
// It's faster than many other algorithms like bubble sort or insertion sort for large datasets.
// Often performs better in practice than merge sort, despite having the same average time complexity.

pub fn run() {
    println!("############### Quick Sort in Rust ###############");

    let mut data: Vec<i32> = vec![9, 8, 7, 6, 5, 4, 3, 2, 1];
    println!("Unsorted data: {:?}", data);

    quick_sort(&mut data);
    println!("Sorted data: {:?}", data);
}

fn quick_sort(data: &mut [i32]) {
    if data.len() <= 1 {
        return;
    }

    let pivot_index = partition(data);
    quick_sort(&mut data[0..pivot_index]);
    quick_sort(&mut data[pivot_index + 1..]);
}

fn partition(data: &mut [i32]) -> usize {
    let pivot_index = data.len() - 1;
    let pivot = data[pivot_index];
    let mut i = 0;

    for j in 0..pivot_index {
        if data[j] <= pivot {
            data.swap(i, j);
            i += 1;
        }
    }
    data.swap(i, pivot_index);
    i
}

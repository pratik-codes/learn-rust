use std::cmp;

pub fn run() {
    println!("############### Binary search in Rust ###############");

    let data = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17];
    let val = 5;
    println!("Data: {:?}", data);
    println!("Value: {}", val);

    match binary_search(&data, val) {
        -1 => println!("Value {} not found", val),
        index => println!("Value {} found at index {}", val, index),
    }

    println!("Recursice binary search");

    match binary_search_recursive(&data, val) {
        -1 => println!("Value {} not found", val),
        index => println!("Value {} found at index {}", val, index),
    }
}

fn binary_search(data: &[i32], val: i32) -> i32 {
    let mut low = 0;
    let mut high = data.len() - 1;
    while low <= high {
        let mid = low + (high - low) / 2;

        match data[mid].cmp(&val) {
            cmp::Ordering::Equal => return mid as i32,
            cmp::Ordering::Less => low = mid + 1,
            cmp::Ordering::Greater => high = mid - 1,
        }
    }
    -1
}

fn binary_search_recursive(data: &[i32], val: i32) -> i32 {
    binary_search_recursive_helper(data, val, 0, data.len() as i32 - 1)
}

fn binary_search_recursive_helper(data: &[i32], val: i32, low: i32, high: i32) -> i32 {
    if low > high {
        return -1;
    }
    let mid = low + (high - low) / 2;
    match data[mid as usize].cmp(&val) {
        cmp::Ordering::Equal => mid,
        cmp::Ordering::Less => binary_search_recursive_helper(data, val, mid + 1, high),
        cmp::Ordering::Greater => binary_search_recursive_helper(data, val, low, mid - 1),
    }
}

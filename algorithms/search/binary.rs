fn main() {
    println!("Binary search in Rust");

    let data = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17];
    let val = 5;
    let index = binary_search(&data, val);
    if index != -1 {
        println!("Value {} found at index {}", val, index);
    } else {
        println!("Value {} not found", val);
    }

    println!("Calling binary_search_recursive");

    let index = binary_search_recursive(&data, val);
    if index != -1 {
        println!("Value {} found at index {}", val, index);
    } else {
        println!("Value {} not found", val);
    }
}

fn binary_search(data: &[i32], val: i32) -> i32 {
    let mut low = 0;
    let mut high = data.len() - 1;
    while low <= high {
        let mid = low + (high - low) / 2;
        if data[mid] == val {
            return mid as i32;
        } else if data[mid] < val {
            low = mid + 1;
        } else {
            high = mid - 1;
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
    if data[mid as usize] == val {
        return mid;
    } else if data[mid as usize] < val {
        return binary_search_recursive_helper(data, val, mid + 1, high);
    } else {
        return binary_search_recursive_helper(data, val, low, mid - 1);
    }
}

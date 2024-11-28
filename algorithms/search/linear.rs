fn main() {
    println!("Linear search in Rust");

    // sample data
    let data = vec![1, 2, 3, 4, 5, 6, 7, 8, 9];
    let val = 5;
    let res: i32 = linear_search(data, val);
    println!("Result: {}", res);
}

fn linear_search(data: Vec<i32>, val: i32) -> i32 {
    println!("Data: {:?}", data);
    println!("Value: {}", val);
    for i in 0..data.len() {
        if data[i] == val {
            println!("Value found at index: {}", i);
            return i as i32;
        }
    }

    println!("Matching value not found");
    return 0;
}

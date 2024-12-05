pub fn run() {
    println!("############### Linear search in Rust ###############");

    // sample data
    let data = vec![1, 2, 3, 4, 5, 6, 7, 8, 9];
    let val = 5;
    println!("Data: {:?}", data);
    println!("Value: {}", val);

    match linear_search(data, val) {
        0 => println!("Value not found"),
        index => println!("Value found at index: {}", index),
    }
}

fn linear_search(data: Vec<i32>, val: i32) -> i32 {
    for i in 0..data.len() {
        if data[i] == val {
            return i as i32;
        }
    }

    // return 0 if not found
    0
}

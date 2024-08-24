use std::string::String; 

pub fn run() {
    // loop concepts
    println!("\n#################################");
    println!("Loop Module");
    println!("#################################\n");
 
    loop_example();
    while_example();
    for_example();
}

fn loop_example() {
    let mut x = 0;
    loop {
        x += 1;
        println!("Loop Example - x: {}", x);
        if x == 5 {
            break;
        }
    }
}

fn while_example() {
    let mut x = 0;
    while x < 5 {
        x += 1;
        println!("While Example - x: {}", x);
    }
}

// how is char and string stored in memory by rust
// char is stored in 4 bytes in memory
// string is stored in 24 bytes in memory
// these can be seen by using the size_of::<T>() function in the std::mem module
fn for_example() {
    let arr = [10, 20, 30, 40, 50];
    for element in arr.iter() {
        println!("For Example - element: {}", element);
    }

    let name: String = String::from("Rust");

    for character in name.chars() {
        println!("For Example - on character: {}", character);
    }

    for byte in name.bytes() {
        println!("you can also iterate over bytes of a string{}", byte);
    }
}


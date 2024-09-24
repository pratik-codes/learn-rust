// Option enum
// - Option<T> is a generic enum with two variants: Some(T) and None
// - It is used when a value is optional
// - It is similar to null in other languages
// - It is used to handle the absence of a value
// in this example, we say if the value is present, return the value, else return None

pub fn run() {
    println!("\n#################################");
    println!("Option Module");
    println!("#################################\n");

    option_example();
}

fn option_example() {
    let result = divide(10, 0);
    match result {
        Some(value) => println!("Result is {}", value),
        None => println!("Error: Cannot divide by zero"),
    }
}

fn divide(x: i32, y: i32) -> Option<i32> {
    if y == 0 {
        return None;
    }
    Some(x / y)
}
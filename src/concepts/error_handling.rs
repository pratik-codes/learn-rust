// Error handling
// Rust has a powerful error handling mechanism. It uses the Result enum to return either a value or an error. The Result enum has two variants: Ok and Err. Ok holds the value returned by the function, and Err holds the error message.
// this is similar to try catch in other languages


pub fn run() {
    println!("\n#################################");
    println!("Error Handling Module");
    println!("#################################\n");

   error_handling_example();
}

fn error_handling_example() {
    let result = divide(10, 0);
    match result {
        Ok(value) => println!("Result is {}", value),
        Err(error) => println!("Error is {}", error),
    }

    println!("End of error handling example function");  // this will run because the error is handled gracefully
}

fn divide(x: i32, y: i32) -> Result<i32, String> {
    if y == 0 {
        return Err("Cannot divide by zero".to_string());
    }
    Ok(x / y)
}
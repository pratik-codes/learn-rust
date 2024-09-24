pub fn run() {
     // function concepts
    println!("\n#################################");
     println!("Function Module");
    println!("#################################\n");
 

     function_example();
     function_parameters();
     closure_example();
}

fn function_example() {
    println!("Function Example - Hello, World!");
}

fn function_parameters() {
    let x = 5;
    let y = 10;
    let sum = add(x, y);
    println!("Function Parameters - Sum: {}", sum);
}

fn add(x: i32, y: i32) -> i32 {
    x + y
}

// The line let sum = |x, y| x + y; in the given code defines a closure function and assigns it to the variable sum.
// A closure is a special type of function that can capture variables from its surrounding environment. In this case, the closure takes two parameters x and y, and returns the sum of x and y. The closure is defined using the |x, y| syntax, where x and y are the input parameters.
// The closure function can be called just like a regular function. In this code, the closure is called with the parameters x and y using the expression sum(x, y). The result of the closure function, which is the sum of x and y, is then printed to the console using the println! macro.
// In summary, this line defines a closure function that calculates the sum of two numbers, assigns it to the variable sum, and then calls the closure function with specific values for x and y.
fn closure_example() -> fn(i32, i32) -> i32 {
    let x = 5;
    let y = 10;
    let sum = |x, y| x + y;
    println!("Function Closure - Sum: {}", sum(x, y));
    sum
}


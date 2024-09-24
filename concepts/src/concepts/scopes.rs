pub fn run() {
    println!("\n#################################");
    println!("Scopes Module");
    println!("#################################\n");
 
    variable_scope();
    shadowing_example();
    function_scope();
}

fn variable_scope() {
    {
        let x = 5;
        println!("Variable Scope - x inside block: {}", x); // x = 5
    }
}

fn shadowing_example() {
    let x = 5;
    {
        let x = x * 2;
        println!("Shadowing Example - x in inner scope: {}", x); // x = 10
    }
    println!("Shadowing Example - x in outer scope: {}", x); // x = 5
}

fn function_scope() {
    let outer_var = 10;

    fn inner_function() {
        let inner_var = 20;
        println!("Function Scope - Inner variable: {}", inner_var); // inner_var = 20
    }

    inner_function();
    println!("Function Scope - Outer variable: {}", outer_var); // outer_var = 10
}

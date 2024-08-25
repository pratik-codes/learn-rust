// - ownership is a set of rules that governs how a rust programs manages memory.
// - in other languages garbage collectors automatically clean up memory that is no longer being used. In Rust, the memory is managed through a system of ownership with a set of rules that the compiler checks at compile time. None of the ownership features slow down your program while it’s running.
// - if these rules are violated, the program will not compile.
// Rules:
// 1. Each value in Rust has a variable that’s called its owner.
// 2. There can only be one owner at a time.
// 3. When the owner goes out of scope, the value will be dropped.
// - Rust will never automatically create “deep” copies of your data. Therefore, any automatic copying can be assumed to be inexpensive in terms of runtime performance.

pub fn run() {
    println!("\n#################################");
    println!("Ownership and Borrowing");
    println!("#################################\n");

    stack_variables_ownership();
    heap_ownership_example();
    return_ownership();
}

// ownership for stack variables is pretty straightforward. The values are pushed onto the stack and popped off when the function in this particular case will go out of scope.
fn stack_variables_ownership() {
    let x = 5;
    let y = 2;
    let c = x + y;
    println!("Stack Variables Ownership - c: {}", c);
}

// - memory management for heap variables is a bit more complex, that is why common issue like dangling pointer, double free error etc happen commonly.
// - That is why Rust has a system of ownership with a set of rules that the compiler checks at compile time.
// - What happens here is s1 here would be on stack and will point to the value of s1 which is a string on heap. When we assign s1 to s2, the ownership of the value on heap is transferred to s2. So, s1 is no longer valid and will give an error if we try to use it.
// - if in case rust doesnt do this and doesnt complain about it, then it would be a dangling pointer issue. where s1 would be a dangling pointer.
fn heap_ownership_example() {
    let s1 = String::from("hello");
    print!("Heap Ownership Example - s1: {}", s1); // this works here because the ownership of s1 is not transferred to s2
    let s2 = s1;
    // println!("Heap Ownership Example - s1: {}", s1); // this will give an error because s1 is no longer valid
    println!("Ownership Example - s2: {}", s2);

    take_ownership(s2);
    // now the ownership of s2 is transferred to take_ownership function. So, s2 is no longer valid here.
    //  print!("Heap Ownership Example - s2: {}", s2); // this will give an error because s2 is no longer valid
}

// there are two ways to do this
fn return_ownership() {
     let mut s1 = String::from("hello");
     // either create a new variable and take ownership there 
     let s2: String = take_ownership(s1); 

     // or take ownership from the function back and assign it to the same variable by making it a mutable variable. 
     s1 = take_ownership(s2);

     // or you can also use reference which would be in borrow and refernce file

     println!("Ownership return Example - s1: {}", s1);
}


// utility that takes the ownership of the string and returns it back if needed.
fn take_ownership(s: String) -> String {
    println!("Ownership Example - The value of s is {}", s);
    return s;
}

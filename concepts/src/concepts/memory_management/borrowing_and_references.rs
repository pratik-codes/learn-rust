// - In rust borrowing is a way to pass a reference of a value to a function or a variable without transferring the ownership of the value.
// - This is done by using the & symbol before the variable name.
// - The reference is immutable by default, meaning the value cannot be changed.
// - To make the reference mutable, we can use the &mut symbol before the variable name.
// - The reference is valid only till the scope of the variable it is referencing to.
// - The reference is also valid only till the scope of the function it is passed to.
// - you can send a mutable reference to someone aswell without transferring the ownership of the value. But the mutable reference can be only one at a time.

// Why does rust do this?
// - Rust’s design has been shaped by a few key goals:
// - - Safety: Rust is designed to be safe, and references are one of its key safety features.
// - - Speed: References in Rust are a zero-cost abstraction, meaning that references incur no additional runtime overhead.
// - - Concurrency: Rust’s type system and ownership model ensure that concurrent access to data is data (Race condition) is not possible.

pub fn run() {
    println!("\n#################################");
    println!("Borrowing and References");
    println!("#################################\n");

    references();
    borrowing();
    borrow_mutable_data();
}

fn references() {
    let s1 = String::from("hello");
    // s2 has a reference to s1 means it borrows the value of s1
    let s2 = &s1;

    println!("s1: {}", s1);
    println!("s2: {}", s2);
}

fn borrowing() {
    let s1 = String::from("hello");

    borrow_variable(&s1); // passes the reference to the function

    // that is why you can still use the s1 as the ownership is not transferred to the function it still belongs to s1
    println!("s1: {}", s1);
}

// you can take a mutable reference to the variable and mutate it by passing the variable as a mutable reference to the function.
// but the mutable reference for a variable can be only one at a time.
// if you try to take another mutable reference to the same variable, it will give an error.
fn borrow_mutable_data() {
    let mut s1 = String::from("hello");
    println!("s1 value before: {}", s1);
    let s2 = &mut s1; // takes the first mutanle reference for s1
    s2.push_str(" world"); // the error below will only show when you take a mutable reference and try to mutate the value
    println!("s2 value before: {}", s2);

    // borrow_and_mutate_variable(&mut s1); // this gives error because s1 is already borrowed as mutable reference
    // let s3 = &mut s1; // this will give an error because s1 is already borrowed as mutable reference

    println!("Borrowing and mutating the value s1: {}", s1);
}

// utility that takes the ownership of the string and returns it back if needed.
fn borrow_variable(s: &String) {
    println!("Ownership Example - The value of s is {}", s);
}

// fn borrow_and_mutate_variable(s: &mut String) {
//     s.push_str(" world");
// }

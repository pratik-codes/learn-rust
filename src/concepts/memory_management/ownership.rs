// pub fn run() {
//     println!("\n#################################");
//     println!("Ownership and Borrowing");
//     println!("#################################\n");
 
//     ownership_example();
//     borrowing_example();
//     lifetimes_example();
// }

// fn ownership_example() {
//     let s1 = String::from("hello");
//     let s2 = s1;
//     println!("Ownership Example - s2: {}", s2);
// }

// fn borrowing_example() {
//     let s = String::from("hello");
//     let len = calculate_length(&s);
//     println!("Borrowing Example - The length of '{}' is {}", s, len);
// }

// fn calculate_length(s: &String) -> usize {
//     s.len()
// }

// fn lifetimes_example() {
//     // let r;
//     // {
//     //     let x = 5;
//     //     r = &x;
//     // }

//     // the code above will give a warning because Once the inner scope ends, x goes out of scope, meaning that r now holds a dangling reference to a value that no longer exists. Rust will not allow this because it violates Rust's safety guarantees.
//     // To fix this, we can use lifetimes as shown below
//     let x = 5; // `x` lives for the entire scope of `lifetimes_example`
//     let r = &x; // `r` borrows `x` for as long as `x` is in scope

//     println!("r: {}", r); // This is fine because `x` is still in scope

//     // Rust’s borrowing and lifetime system is designed to prevent memory safety issues like dangling references
// }

// WARNING: This is a very important concept and is very useful in memory management
// In case the example doesnt make much sense or you want to know more about it.
// Please refer to the official documentation here -https://doc.rust-lang.org/book/ch10-03-lifetime-syntax.html
// I have used a very naive example to explain the concept in the most easy way possible

// LIFETIMES
// Lifetimes are a way to tell the compiler how long references are valid for. They are a way to prevent dangling references.
// INFO: This is how you do lifetime annotations in Rust
// &i32        // a reference
// &'a i32     // a reference with an explicit lifetime
// &'a mut i32 // a mutable reference with an explicit lifetime

// Lifetime annotations don’t change how long any of the references live. Rather, they describe the relationships of the lifetimes of multiple references to each other without affecting the lifetimes. Just as functions can accept any type when the signature specifies a generic type parameter, functions can accept references with any lifetime by specifying a generic lifetime parameter.
// Lifetime annotations have a slightly unusual syntax: the names of lifetime parameters must start with an apostrophe (') and are usually all lowercase and very short, like generic types. Most people use the name 'a for the first lifetime annotation. We place lifetime parameter annotations after the & of a reference, using a space to separate the annotation from the reference’s typ

// The answer is that Rust's ownership and borrowing system, including lifetimes, is designed to manage memory safely and efficiently. When you pass references around in a program, Rust needs to ensure that these references are valid for the entire duration they are being used.
// While learning about lifetimes a question arise to me that why do we need lifetimes in the first place?
// If a reference outlives the data it points to, it becomes a dangling reference, leading to undefined behavior and potential security vulnerabilities.

pub fn run() {
    println!("\n#################################");
    println!("Lifetimes");
    println!("#################################");

    basic_example();
    generic_lifetime();
    static_lifetime();
}

// Here, we’ve annotated the lifetime of r with 'a and the lifetime of x with 'b.
// As you can see, the inner 'b block is much smaller than the outer 'a lifetime block. At compile time,
// Rust compares the size of the two lifetimes and sees that r has a lifetime of 'a but that it refers to memory with a lifetime of 'b.
// The program is rejected because 'b is shorter than 'a: the subject of the reference doesn’t live as long as the reference.
// And this avoids dangling pointers and memory leaks
// fn basic_example0() {
//     let r;                // ---------+-- 'a
//                           //          |
//     {                     //          |
//         let x = 5;        // -+-- 'b  |
//         r = &x;           //  |       |
//     }                     // -+       |
//                           //          |
//     println!("r: {r}");   //          |
// }

fn basic_example() {
    let my_string: String = String::from("Hello, world!");
    let mut string_vec: Vec<&String> = vec![];

    push_string(&mut string_vec, &my_string);

    // drop(my_string); // this will give an error because the reference in the vector is still valid

    println!("stringVec: {:?}", string_vec);
}

// if we push to the vec in this form it gives an error which is intresting
// fn push_string(str_vec: &mut Vec<&String>, my_string: &String) {
//     str_vec.push(my_string); // this gives an error saying that my_string does not live long enough
// }

// this doesnot give an error so maybe we are not passing the type correctly because the generic
// clearly works here
// fn push_string<T>(str_vec: &mut Vec<T>, my_string: T) {
//     str_vec.push(my_string); // this gives an error saying that my_string does not live long enough
// }

// but what we need to  do is to tell the compiler that the reference is valid for a certain lifetime
// so we need to specify the lifetime of the reference
// what this says is that the reference my_string is valid for the lifetime 'a
// and the reference in the vector is also valid for the lifetime 'a
// so the reference in the vector is valid as long as the reference my_string is valid
// so in case the reference my_string goes out of scope the reference in the vector will also be invalid
// hence in situations like this lifetime can help prevent dangling references and is amazing for memory management
fn push_string<'a>(str_vec: &mut Vec<&'a String>, my_string: &'a String) {
    str_vec.push(my_string);
}

fn generic_lifetime() {
    let string1 = String::from("abcd");
    let string2 = "xyz";
    let result = longest(string1.as_str(), string2);
    println!("The longest string is {}", result);
}

// utils function to find the longest string
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

// In this case the lifetime of s would be as long as the program run because it has a static lifetime
// and is stored in the read only memory
// before anotating any variable as static make sure it lives for the entire program
fn static_lifetime() {
    let s: &'static str = "I have a static lifetime";
    println!("s: {}", s); // prints "I have a static lifetime
}

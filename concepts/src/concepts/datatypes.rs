// u is for unsigned integers (positive numbers)
// i is for signed integers (positive and negative numbers)
// The number after u or i indicates the number of bits
// For example, u8 is an unsigned integer with 8 bits
// The number of bits determines the range of values that can be stored
// For example, an u8 can store values from 0 to 255
// i8 can store values from -128 to 127
// The number of bits also determines the memory used by the variable
// For example, u8 uses 1 byte of memory (8 bits)
// i8 uses 1 byte of memory (8 bits)
// i32 uses 4 bytes of memory (32 bits)
// i64 uses 8 bytes of memory (64 bits)
// i128 uses 16 bytes of memory (128 bits)
// The default integer type is i32

pub fn run() {
    // datatype concepts
    println!("\n#################################");
    println!("Data Types Module");
    println!("#################################\n");
    integer_types();
    floating_point_types();
    boolean_type();
    array_type();
    tuple_type();
    character_type();
    vectors();
    immutable_variable_example();
    mutable_variable_example();
}

fn integer_types() {
    let x: i32 = 10;
    let y: u8 = 255;
    println!("Integer Types - x(i32): {}, y(u8): {}", x, y);
}

fn floating_point_types() {
    let x: f32 = 2.5;
    let y: f64 = 3.14;
    println!("Floating Point Types - x: {}, y: {}", x, y);
}

fn boolean_type() {
    let is_rust_awesome: bool = true;
    println!("Boolean Type - Is Rust awesome? {}", is_rust_awesome);
}

fn character_type() {
    let letter: char = 'R';
    let emoji: char = '😊';
    println!("Character Type - letter: {}, emoji: {}", letter, emoji);
}

fn tuple_type() {
    let tuple: (i32, f64, char) = (42, 3.14, 'R');
    let (x, y, z) = tuple;
    println!("Tuple Type - x: {}, y: {}, z: {}", x, y, z);
}

fn array_type() {
    let array: [i32; 3] = [1, 2, 3];
    println!("Array Type - Array: {:?}", array);
}

fn immutable_variable_example() {
    let x = 5;
    // x = 6; // This would cause a compile-time error because `x` is immutable
    println!("Immutable x: {}", x);
}

// what is vector?
// A vector is a dynamic array that can grow or shrink in size at runtime
fn vectors() {
    let mut v = vec![1, 2, 3];
    v.push(4);
    println!("Vector: {:?}", v);
}

fn mutable_variable_example() {
    let mut x = 5;
    println!("Before mutation: {}", x);

    x = 6; // This is allowed because `x` is mutable
    println!("After mutation: {}", x);
}

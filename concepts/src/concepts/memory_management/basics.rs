// - SSD and RAM are examples of storage devices
// - all the apps you run in your computer are stored in RAM
// - RAM is volatile memory, meaning that it loses its contents when the power is turned off
// - Memory management is the process of allocating and deallocating memory in a computer system

// ****************** JAVASCRIPT WAY *******************************
// - lets see what happens when we run the code above in javascript
// - so the if we look at the javascript code below
// function add(x, y) {
//   const sum = x + y;
//   return sum;
// }
// - the sum variable is stored in the stack memory (temporary memory/RAM) and is automatically deallocated when the function returns and is out of scope
// - this is done by the garbage collector in javascript and hence you cant do it manually 
// - usually avoids dangling pointers and memory leaks if written correctly
// - this also makes it slow because the garbage collector has to run and clean up the memory

// ****************** C, C++ WAY *******************************
// - in c and c++ you have to manually allocate and deallocate memory
// - can lead to memory leaks and dangling pointers if not done correctly because it is not automatically done and it is very easy to make mistakes here 
// - learning curve is also high for this 

// ****************** RUST WAY *******************************
// - Rust uses a concept called ownership to manage memory
// - Ownership is a unique feature of Rust that allows it to manage memory efficiently without the need for a garbage collector
// - make it safe and not very hard to do 
// it achieves this by concepts like mutability, ownership model, borrowing and lifetimes


pub fn run () {
     // memory management concepts
     println!("\n#################################");
     println!("Memory Management Module");
     println!("#################################\n");
     
    simple_memory_management();
    multiple_functions_mm(); // mm - memory management
    dynamic_memory_allocation();

    mutability();
}

// **************************** STACK AND HEAP ******************************* 
// - Rust has two types of memory allocation: stack and heap
// - rust stores things like variables and function calls on the stack
// - variables like let u8 = 5; are stored on the stack as it is known at compile time how much space this particular variable is going to take 
// but some variables can change at runtime and we dont know how much space they are going to take like a vector or a string
// - these are stored on the heap
// - the heap is less organized than the stack and is slower to access
// - that is why accessing a vector and string is slower than accessing a variable on the stack
// - if you put anything in stack recovering it is easy but if you put it in the heap it is hard to recover it and it takes time because you have to ask the operating system for memory
// - 32bit + 32bit = 64bit would be put in the stack as a stack frame and would be deallocated when the function is done
fn simple_memory_management() {
    // stack memory
    let _x: i32 = 5;
    let _y: i32 = 10;

} 

// here the first frame for multiple_functions_mm()(64 bit) would be created and then the frame for simple_memory_management()(64 bit) would be created
// both the frames would be put in the stack and would be deallocated when the function is done
// STACK FRAME 
// 32 bit + 32 bit = 64 bit (simple_memory_management())
// 32 bit + 32 bit = 64 bit (multiple_functions_mm())
// when the execution happens simple_memory_management() would be popped first and then the multiple_functions_mm() 
fn multiple_functions_mm() {
    let _x: i32 = 5;
    let _y: i32 = 10;
    simple_memory_management();
}

// in this case rust stores the string on heap but to access it we need to use a reference to the string which is stored on the stack
// so the thing on stack that is store looks something like this
// |------------------------------|
// |  pointer to heap  | address  |
// |  len              | 5        |
// |  capacity         | 5        |
// |------------------------------|
// the thing the stored on heap looks something likes this 
// |--------------|
// |index | value |
// |0     | h     |
// |1     | e     |
// |2     | l     |
// |3     | l     |
// |4     | 0     |
// |------|-------|
// when length of the string increases the capacity is increased and the string is moved to a new location in the heap and pointer is updated
// after the string "world" is added the tables above changes with the updated value of world as well
fn dynamic_memory_allocation() {
    // heap memory
    let s: String = String::from("hello"); 
    println!("Dynamic Memory Allocation - s: {}", s);
    // lets print the capacity and length of the string and the address of the string
    println!("Dynamic Memory Allocation - s: capacity: {}", s.capacity());
    println!("Dynamic Memory Allocation - s: length: {}", s.len());
    println!("Dynamic Memory Allocation - s: address: {:p}", s.as_ptr());

    // lets add a character to the string "world"
    let s = s + " world";
    println!("Dynamic Memory Allocation - s: {}", s);
    println!("Dynamic Memory Allocation - s: capacity: {}", s.capacity());
    println!("Dynamic Memory Allocation - s: length: {}", s.len());
    println!("Dynamic Memory Allocation - s: address: {:p}", s.as_ptr());
}

// ************************* MUTABILITY ***************************** 
// - Mutability is the ability to change the value of a variable after it has been declared
// - In Rust, variables are immutable by default, meaning that once a variable is assigned a value, it cannot be changed
// - This helps prevent bugs and makes the code easier to reason about
// - To make a variable mutable, you can use the mut keyword before the variable name
// - knowing when to use mutability is important in Rust, as it can affect the safety and performance of your code
// - language like javascript and python have mutable variables by default even the const keyword in javascript is mutable
// for example 
// const x = [1, 2, 3];
// x.push(4);
// console.log(x); // [1, 2, 3, 4]
// - even tho the the array is const we can update it because the const keyword only makes the reference to the array immutable not the array itself
fn mutability() {
    // immutable variable
    let x = 5;
    println!("Immutable Variable - x: {}", x);

    // mutable variable
    let mut y = 10;
    println!("Mutable Variable - y: {}", y);

    // change the value of y
    y = 15;
    println!("Mutable Variable - y: {}", y);
}
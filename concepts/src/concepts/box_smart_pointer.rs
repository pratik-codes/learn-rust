// box in rust are the most straight forward smart pointer. whos types are written Box<T>
// Boxes allow you to store data on the heap rather than the stack. What remains on the stack is the pointer to the heap data
// When should we use Box<T>?
// When you have a type whose size can’t be known at compile time and you want to use a value of that type in a context that requires an exact size
// When you have a large amount of data and you want to transfer ownership but ensure the data won’t be copied when you do so
// When you want to own a value and you care only that it’s a type that implements a particular trait rather than being of a specific type

// Why do we need Box<T>?
// Transferring ownership of a large amount of data can take a long time because the data is copied around on the stack. To improve performance in this situation, we can store the large amount of data on the heap in a box. Then, only the small amount of pointer data is copied around on the stack, while the data it references stays in one place on the heap.

pub fn run() {
    println!("\n#################################");
    println!("Box Smart Pointer");
    println!("#################################");

    syntax();
}

fn syntax() {
    let b = Box::new(5);
    println!("b = {}", b);
}

trait Animal {
    fn make_sound(&self);
}

struct Dog;

impl Animal for Dog {
    fn make_sound(&self) {
        println!("Woof!");
    }
}

// let my_pet: Box<dyn Animal> = Box::new(Dog);
// my_pet.make_sound(); // Outputs "Woof!"

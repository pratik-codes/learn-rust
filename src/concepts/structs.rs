// Structs
// Structs are used to create custom data types.

struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

pub fn run() {
    println!("\n#################################");
    println!("Structs");
    println!("#################################\n");

    struct_example();
    implementing_structs();
}

fn struct_example() {
    // lets see how the data is created herer
    let user1 = User {
        // value stored on the heap and reference is stored in the stack
        email: String::from("John doe"),
        // value stored on the heap and reference is stored in the stack
        username: String::from("john@gmail.com"),
        // value stored on the stack
        active: true,
        // value stored on the stack
        sign_in_count: 1,
    };

    println!(
        "Struct Example - User1 email: {}, username: {}, active: {}, sign_in_count: {}",
        user1.email, user1.username, user1.active, user1.sign_in_count
    );
}


// Implementing Structs
// We can implement methods on structs using impl keyword.
struct Rect {
     width: u32,
     height: u32,
}

impl Rect {
     fn area(&self) -> u32 {
          self.width * self.height
     }
     fn perimeter(&self) -> u32 {
          2 * (self.width + self.height)
     }
}

fn implementing_structs() {
    let rect1 = Rect {
        width: 30,
        height: 50,
    };

     println!(
          "Implementing Structs - The area of the rectangle is: {} square pixels",
          rect1.area()
     ); 
     println!(
          "Implementing Structs - The perimeter of the rectangle is: {} pixels",
          rect1.perimeter()
     );
}
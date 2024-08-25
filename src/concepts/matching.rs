// pattern matching
// Matching in rust is similar to switch in other languages, but it is more powerful. It can be used to destructure structs, enums, tuples, and more.


pub fn run() {
    println!("\n#################################");
    println!("Matching Module");
    println!("#################################\n");
     
     match_example();
     match_enums(Shape::Circle);
     match_tuples();
}

fn match_example() {
    let x = 5;
    match x {
        1 => println!("x is 1"),
        2 => println!("x is 2"),
        3 => println!("x is 3"),
        4 => println!("x is 4"),
        5 => println!("x is 5"),
        _ => println!("x is not 1, 2, 3, 4, or 5"),
    }
}

// matching example with the help of enums
enum Shape {
     Circle,
     // Square,
     // Rectangle,
     // Triangle,
}

fn match_enums(shape: Shape) {
     match shape {
          Shape::Circle => println!("Shape is a circle"),
          // Shape::Square => println!("Shape is a square"),
          // Shape::Rectangle => println!("Shape is a rectangle"),
          // Shape::Triangle => println!("Shape is a triangle"),
     }
}

// matching with tuples
fn match_tuples() {
     let tuple = (0, 1);
     // what happens here is that the first value of the tuple is matched with the first value of the tuple in the match arm
     // and the second value of the tuple is matched with the second value of the tuple in the match arm
     match tuple {
          (0, y) => println!("y is {}", y),
          (x, 0) => println!("x is {}", x),
          _ => println!("No match"),
     }
}
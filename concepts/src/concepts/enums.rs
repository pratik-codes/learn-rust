// enums
// what are enums?
// - enums are a way to define a type by enumerating its possible variants.

// Enums in Rust are similar to enums in other languages, but they have some extra features.
// Enums can be used to create custom data types that can be one of a few different variants.
// Enums are useful when you have a fixed set of values that a variable can have.

pub fn run() {
    println!("\n#################################");
    println!("Enums");
    println!("#################################\n");

    enum_example();
    enums_with_values();
}

enum Movement {
    // variants
    Up,
    Down,
    Left,
    Right,
}

fn enum_example() {
    let player1 = Movement::Up;
    let player2 = Movement::Down;
    let player3 = Movement::Left;
    let player4 = Movement::Right;

    move_player(player1);
    move_player(player2);
    move_player(player3);
    move_player(player4);
}

fn move_player(_m: Movement) {
    // do something
}

// ******************* Enums with values *************************

enum Shape {
     Rectangle(u32, u32),
     Circle(f64),
     Triangle(f64, f64, f64),
}

fn enums_with_values() {
     let rect = Shape::Rectangle(20, 30);
     let circle = Shape::Circle(3.14);
     let triangle = Shape::Triangle(3.0, 4.0, 5.0);
     
     calculate_area(rect);
     calculate_area(circle);
     calculate_area(triangle);
}

fn calculate_area(shape: Shape) {
     match shape {
          Shape::Rectangle(width, height) => {
               println!("Area of rectangle: {}", width * height);
          }
          Shape::Circle(radius) => {
               println!("Area of circle: {}", 3.14 * radius * radius);
          }
          Shape::Triangle(a, b, c) => {
               let s = (a + b + c) / 2.0;
               let area = (s * (s - a) * (s - b) * (s - c)).sqrt();
               println!("Area of triangle: {}", area);
          }
     }
}


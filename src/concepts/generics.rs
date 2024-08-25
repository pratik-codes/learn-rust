// Generics
// - Generics are a way to make code more flexible and reusable.
// - They allow you to write code that can work with any type.
// - Generics are a way to abstract over types.
// We use generics to create definitions for items like function signatures or structs, which we can then use with many different concrete data types.
// things like options and results are use genrics as well. The alphabet T is used to represent the type in them or it can be represented with other alphabet as well.

use std::cmp::PartialOrd;

pub fn run() {
    println!("\n#################################");
    println!("Generics Module");
    println!("#################################\n");

    generics_example();
     generic_in_structs();
}

fn generics_example() {
    let number_list = vec![34, 50, 25, 100, 65];

    let result = largest(&number_list);
    println!("The largest number is {result}");
}

// The function signature for largest now reads as follows:
// T is a generic type parameter. hence any type can be passed to the function.
// but it also has a constraint that the type must implement the PartialOrd trait.
// This means we can compare values of type T in the function.
// hence you cant pass any type to the function, it must be a type that implements the PartialOrd trait. which means it should be a type that can be ordered or compared.
// This makes the function very flexible and reusable and safe.
// you can refer to traits.rs for more information on traits.
fn largest<T: PartialOrd>(list: &[T]) -> &T {
     let mut largest = &list[0];

     for item in list {
          if item > largest {
               largest = item;
          }
     }

     largest
}

struct Point<T, U> {
    x: T,
    y: U,
}

fn generic_in_structs() {
    let _both_integer = Point { x: 5, y: 10 };
    let _both_float = Point { x: 1.0, y: 4.0 };
    let _integer_and_float = Point { x: 5, y: 4.0 };
}
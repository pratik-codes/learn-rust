// Just like the nodejs ecosystem has npm, the rust ecosystem has cargo
// Cargo is a package manager for rust, which means we can use it to bring packages (crates in case of rust) to our projects

// To create a new project, we can use the following command
// cargo new <project_name>
// This will create a new project with the following structure
// <project_name>
// ├── Cargo.toml
// └── src
//     └── main.rs

// Cargo.toml is the configuration file for the project
// It contains the metadata about the project and the dependencies

// To add a dependency to the project, we can add the following line to the Cargo.toml file
// [dependencies]
// <crate_name> = "<version>"

// To install the dependencies, we can run the following command
// cargo build
// This will download the dependencies and build the project

// To run the project, we can use the following command
// cargo run

// To update the dependencies, we can use the following command
// cargo update

// To remove the dependencies, we can remove the line from the Cargo.toml file and run the following command
// cargo build

use rand::{Rng, thread_rng};
fn create_random_num() {
    let mut rng = thread_rng();
    let n: u32 = rng.gen();
    println!("Random number: {}", n);
}

use chrono::{Local, Utc};
fn main() {
    // Get the current date and time in UTC
    let now = Utc::now();
    println!("Current date and time in UTC: {}", now);

    // Format the date and time
    let formatted = now.format("%Y-%m-%d %H:%M:%S");
    println!("Formatted date and time: {}", formatted);

    // Get local time
    let local = Local::now();
    println!("Current date and time in local: {}", local);
}


// What all libraries does rust have?
// A lot of them
// https://actix.rs/ - Extremely fast http server
// https://serde.rs/ - Serializing and deserialising data in rust
// https://tokio.rs/ - Asynchronous runtime in rust
// https://docs.rs/reqwest/latest/reqwest/ - S
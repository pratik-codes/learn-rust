mod search;
mod sorting;

fn main() {
    println!("$$$$$$$$$$$$$$$$$$$ Data Structures and Algorithms in Rust $$$$$$$$$$$$$$$$$$$");
    println!("\n");

    search::linear::run();
    search::binary::run();

    sorting::bubble_sort::run()
}

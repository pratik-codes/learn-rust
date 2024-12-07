mod search;
mod sorting;

fn main() {
    // Search Algorithms
    search::linear::run();
    search::binary::run();

    // Sorting Algorithms
    sorting::bubble_sort::run();
    sorting::quick_sort::run();
}

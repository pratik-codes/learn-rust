mod concepts;

fn main() {
    println!("\n#################################");
    println!("Learn rust!");
    println!("#################################");

    concepts::datatypes::run();
    concepts::scopes::run();
    concepts::loops::run();
    concepts::functions::run();
    concepts::structs::run();
    concepts::enums::run();
    concepts::matching::run();
    concepts::error_handling::run();
    concepts::memory_management::basics::run();
    concepts::memory_management::ownership::run();
    concepts::memory_management::borrowing_and_references::run();
    concepts::memory_management::lifetimes::run();
    concepts::option::run();
    concepts::generics::run();
    concepts::traits::run();
    concepts::box_smart_pointer::run();
}

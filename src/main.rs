mod hello;
mod primitives;
mod custom_types;

fn main() {
    println!("-Hello World-");
    hello::hello_world();
    println!("\n-Formatted Print-");
    hello::formatted_print();
    println!("\n-Debug-");
    hello::debug();
    println!("\n-Display-");
    hello::display();
    println!("\n-Testcase-");
    hello::testcase();
    println!("\n-Formatting-");
    hello::formatting();

    println!("\n-Primitives-");
    primitives::primitives();
    println!("\n-Literals & Operators-");
    primitives::literals_and_operators();
    println!("\n-Tuples-");
    primitives::tuples();
    println!("\n-Arrays & Slices-");
    primitives::arrays_and_slices();

    println!("\n-Structures-");
    custom_types::structures();
    println!("\n-Enums-");
    custom_types::enums();
    println!("\n-Type Aliases-");
    custom_types::aliases();
    println!("\n-Use-");
    custom_types::_use();
    println!("\n-C-like-");
    custom_types::c_like();
    println!("\n-Link-list-");
    custom_types::linked_list();
    println!("\n-Constants-");
    custom_types::constants();
}

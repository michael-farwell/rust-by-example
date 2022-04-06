mod hello;
mod primitives;

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
}

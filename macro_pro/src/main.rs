use hello_macro::HelloMacro;
use hello_macro_attribute::route;
use hello_macro_derive::HelloMacro;

#[derive(HelloMacro)] // derive 派生，派生宏只能作用于struct和enum
struct Pancakes;

#[route("GET", "/")]
fn index() {
    println!("Hello, world!");
}

fn main() {
    Pancakes::hello_macro();
}

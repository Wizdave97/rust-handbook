use hello_macro::{self, my_vec, HelloMacro};
use hello_macro_derive::HelloMacro;

#[derive(HelloMacro)]
pub struct Pancakes {}
fn main() {

    let vec = my_vec!(1,2,3);

    println!("{:?}", vec);

    Pancakes::hello_macro();
}
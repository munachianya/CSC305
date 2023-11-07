mod greeting;
mod how_you_hold_data_for_operations;

// extern crate hello_world_lib;
///Optionally load each member of greetings
/*use greetings::default_greeting;
use greetings::spanish;
use greetings::french;*/
///Alternatively, use * to load them all
//use greetings::*;
///Alternatively, load them in one line
use greeting::{english, french, igbo, spanish};
fn main() {
    let number = [1, 2, 3, 4, 5, 6];
    println!("Hello, world!");
    println!("English : {}", english::default_greeting());
    println!("Igbo : {}", igbo::default_greeting());
    println!("Spanish : {}", spanish::default_greeting());
    println!("French : {}", french::default_greeting());

    //*calling the main function from scalar */
    how_you_hold_data_for_operations::primitive::scalar::main();
    
    //*calling the main and analyze_slice function from compound */
    how_you_hold_data_for_operations::primitive::compound::analyze_slice(&number);
    how_you_hold_data_for_operations::primitive::compound::main();
}

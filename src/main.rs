extern crate pi_explorer;

use std::env;
use pi_explorer::get_digit;

fn main() {
    let id = env::args().nth(1).expect("Give me the digit you wanna know!");
    let digit = get_digit(id.parse().expect("Invalid index!"));

    println!("0x{:x}", digit);
}

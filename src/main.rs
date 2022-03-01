/*
RPRIME by Alexander Abraham,
a.k.a. "The Black Unicorn", a.k.a. "Angeldust Duke".
Licensed under the MIT license.
*/

use std::env;
//use std::num::ParseIntError;
use rprime::rprime::is_prime;
use rprime::rprime::next_prime;

/// Checks if a string is an integer or not.
/// Returns a boolean to that effect.
fn is_int(number: String) -> bool {
    let mut _result: Vec<bool> = Vec::new();
    let num = number.parse::<i128>();
    match num {
        Ok(_value) => _result.push(true),
        Err(_error) => _result.push(false)
    }
    return _result[0];
}

/// A small error message
/// in case someone misuses
/// the tool.
fn error_message() {
    println!("Wrong usage!");
}

/// Main entry-point for the Rust compiler.
fn main(){
    let args: Vec<String> = env::args().collect();
    let arg_len = args.len();
    if arg_len == 3 {
        if args[1].clone() == "i" && is_int(args[2].clone()) == true {
            let number = args[2].clone().parse::<i128>().unwrap();
            println!("{:?}", is_prime(number));
        }
        else if args[1].clone() == "n" && is_int(args[2].clone()) == true {
            let number = args[2].clone().parse::<i128>().unwrap();
            println!("{:?}", next_prime(number));
        }
        else {
            error_message();
        }
    }
    else {
        error_message();
    }
}

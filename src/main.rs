/*
RPRIME by Alexander Abraham,
a.k.a. "The Black Unicorn", a.k.a. "Angeldust Duke".
Licensed under the MIT license.
*/

use std::env;
use std::num::ParseIntError;

/// Gets all factors of a signed integer
/// with maximum length of 128 and returns them
/// in a vector.
fn number_factors(number: i128) -> Vec<i128> {
    let end_range: i128 = number + 1;
    let mut factor_list: Vec<i128> = Vec::new();
    for i in 1..end_range {
        if number%i == 0 {
            factor_list.push(i);
        }
        else {}
    }
    return factor_list;
}

/// Checks whether a signed integer
/// of length 128 is a prime
/// number or not. Returns a boolean
/// to that effect.
fn is_prime(number: i128) -> bool {
    let mut result: bool = false;
    if number_factors(number).len() == 2 && number_factors(number)[0] == 1 && number_factors(number)[1] == number {
        result = true;
    }
    else {}

    return result;
}


/// Gets the next biggest prime
/// and returns it as a signed integer of
/// maximum length 128.
fn next_prime(number: i128) -> i128 {
    let mut _result: i128 = 0;
    let mut subject: i128 = number;
    loop {
        subject = subject + 1;
        if is_prime(subject) == true {
            _result = subject;
            break;
        }
        else {}
    }
    return _result;
}

/// Checks if a string is an integer or not.
/// Returns a boolean to that effect.
fn is_int(number: String) -> bool {
    let mut _result: bool = false;
    let conversion = || -> Result<i128, ParseIntError> {
        let int = number.parse::<i128>().unwrap();
        return Ok(int);
    };
    if let Err(_err) = conversion() {}
    else {
        _result = true;
    }
    return _result;
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

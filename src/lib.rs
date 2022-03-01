/*
RPRIME by Alexander Abraham,
a.k.a. "The Black Unicorn", a.k.a. "Angeldust Duke".
Licensed under the MIT license.
*/

pub mod rprime {

    /// Gets all factors of a signed integer
    /// with maximum length of 128 and returns them
    /// in a vector.
    pub fn number_factors(number: i128) -> Vec<i128> {
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
    pub fn is_prime(number: i128) -> bool {
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
    pub fn next_prime(number: i128) -> i128 {
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
}

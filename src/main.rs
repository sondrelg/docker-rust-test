use std::env;
use std::str::FromStr;

fn is_prime(n: u32) -> bool {
    if n <= 1 {
        return false;
    }
    for a in 2..n {
        if n % a == 0 {
            return false; // if it is not the last statement you need to use `return`
        }
    }
    true // last value to return
}

fn main() {
    let args: Vec<String> = env::args().collect();

    let number = u32::from_str(&args[1]).expect("Not a positive number");

    if is_prime(number) {
        println!("{} is prime", number)
    } else {
        println!("{} is not prime", number)
    }
}
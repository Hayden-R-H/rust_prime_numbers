mod prime_numbers;
use std::env;

use prime_numbers::sieve_of_eratosthenes;
use prime_numbers::sieve_of_atkin;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("Usage: {} <positive_integer>", args[0]);
        return;
    }
    let input_string = &args[1];
    match input_string.parse::<u64>() {
        Ok(unsigned_int) => {
            let limit: usize = unsigned_int as usize;
            sieve_of_eratosthenes(limit);
            sieve_of_atkin(limit);
        }
        Err(_) => {
            println!("Oops! Invalid input. Make sure it's a positive integer.");
        }
    }
}

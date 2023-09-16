/*
Here's a brief overview of the Sieve of Eratosthenes algorithm:

    Start with a list of numbers from 2 to some upper limit, representing potential primes.
    Initialize a variable p to 2, the first prime number.
    Mark all multiples of p as non-prime by setting their corresponding entries in the list to a special value (e.g., 0).
    Find the next non-zero number in the list greater than p, and set it as the new value of p. Repeat the marking process.
    Repeat step 4 until there are no more non-zero numbers in the list.

At the end of the process, the list will contain only the prime numbers within the specified range.
*/



const NOT_PRIME: bool = false;
const PRIME: bool = true;

///
/// Sieve of Eratosthenes
/// 
/// # Arguments
/// 
/// * `limit` - The upper limit of the range of numbers to check for primality.
/// 
/// # References
///     * [Sieve of Eratosthenes](https://en.wikipedia.org/wiki/Sieve_of_Eratosthenes)
///    * [Sieve of Eratosthenes in Rust](https://www.geeksforgeeks.org/sieve-of-eratosthenes-in-rust/)
pub fn sieve_of_eratosthenes(limit: usize) {
    let mut primes = vec![PRIME; limit + 1];
    primes[0] = NOT_PRIME;
    primes[1] = NOT_PRIME;

    for i in 2..=limit {
        if primes[i] {
            let mut j = i * i;
            while j <= limit {
                primes[j] = NOT_PRIME;
                j += i;
            }
        }
    }

    for (i, &is_prime) in primes.iter().enumerate() {
        if is_prime {
            println!("Eratosthenes: {}", i);
        }
    }
}

/*
Here's a high-level overview of the Sieve of Atkin algorithm:

    Initialize an array of boolean values, indicating whether each number is prime or not. By default, all values are set to false.
    Set the values for 2, 3, and 5 to true, as they are the known prime numbers.
    Loop through all the numbers up to the square root of the limit (let's call this value sqrt_limit).
    For each number x in the range of 1 to sqrt_limit, calculate its square (x_square).
    For each number y in the range of 1 to sqrt_limit, calculate its square (y_square).
    For each pair of x_square and y_square, evaluate three conditions based on the modulo values to determine if the number is potentially prime.
    Mark the numbers that satisfy the conditions as either prime or non-prime in the boolean array.
    Finally, iterate through the boolean array and output the prime numbers.
*/

pub fn sieve_of_atkin(limit: usize) {
    const NOT_PRIME: bool = false;
    const PRIME: bool = true;

    let mut primes = vec![NOT_PRIME; limit + 1];
    // primes[0] = PRIME;
    // primes[1] = PRIME;
    primes[3] = PRIME;

    let sqrt_limit = (limit as f64).sqrt() as usize;
    for x in 1..=sqrt_limit {
        let x_square = x * x;
        for y in 1..=sqrt_limit {
            let y_square = y * y;
            let n = 4 * x_square + y_square;
            if n <= limit && (n % 12 == 1 || n % 12 == 5) {
                primes[n] = !primes[n];
            }
            let n = 3 * x_square + y_square;
            if n <= limit && n % 12 == 7 {
                primes[n] = !primes[n];
            }
            if x > y {
                let n = 3 * x_square - y_square;
                if n <= limit && n % 12 == 11 {
                    primes[n] = !primes[n];
                }
            }
        }
    }
    primes[2] = PRIME;

    for (i, &is_prime) in primes.iter().enumerate() {
        if is_prime {
            println!("Atkin: {}", i);
        }
    }
}

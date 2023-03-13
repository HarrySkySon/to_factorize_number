// The Rust program that asks the user to enter a number and then factorizes it using the factorize function

use std::io;

fn main() {
    println!("Enter a number to factorize:");

    let mut user_input = String::new();
    io::stdin().read_line(&mut user_input).expect("Failed to read line");
    let user_input: u64 = match user_input.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            eprintln!("Please enter a valid number");
            return;
        }
    };

    let factors = factorize(user_input);

    println!("The prime factors of {} are: {:?}", user_input, factors);
}

fn factorize(n: u64) -> Vec<u64> {
    let mut i = 2;
    let mut factors = Vec::new();

    let mut n = n;
    while i * i <= n {
        if n % i != 0 {
            i += 1;
        } else {
            n /= i;
            factors.push(i);
        }
    }

    if n > 1 {
        factors.push(n);
    }

    factors
}

use std::io::{stdin, stdout, Write};
fn is_prime(number: u32) -> bool {
    if number <= 1 {
        return false;
    }

    for i in 2..=number / 2 {
        if number % i == 0 {
            return false;
        }
    }

    true
}

fn make_prime(mut number: u32) -> u32 {
    if is_prime(number) {
        return number;
    } else {
        number += 1;
        make_prime(number)
    }
}

fn main() {
    loop {
        print!("Input your number (or q to quit):");
        stdout().flush().unwrap();
        let mut input_value = String::new();
        if let Err(_) = stdin().read_line(&mut input_value) {
            println!("Failed to read input. Try again");
            continue;
        }
        let input_value = input_value.trim();
        if input_value.eq_ignore_ascii_case("q") {
            println!("ByeBye");
            break;
        }
        match input_value.parse::<u32>() {
            Ok(input_value) => {
                if is_prime(input_value) {
                    println!("{} is prime", input_value);
                } else {
                    let ret = make_prime(input_value);
                    println!("{} is prime", ret);
                }
            }
            Err(_) => {
                println!("Wrong value entered, try again");
            }
        }
    }
}

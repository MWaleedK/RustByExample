mod converter;
use converter::Converter;
use std::io::{Write, stdin, stdout};
fn input_age() -> u16 {
    print!("Input your age in years: ");
    stdout().flush().unwrap();
    let mut buffer = String::new();
    stdin().read_line(&mut buffer);
    match buffer.trim().parse::<u16>() {
        Ok(age) => age,
        Err(_) => {
            println!("Invalid value, try again");
            input_age()
        }
    }
}

fn main() {
    let mut con = Converter::new();
    let age = input_age();
    println!("This program is not accounting leap years.");
    con.set_value_in_years(age);
    let days = con.get_value_in_days();
    let years = con.get_value_in_years();
    println!("Your age in days is: {days}, which makes {years} years")
}

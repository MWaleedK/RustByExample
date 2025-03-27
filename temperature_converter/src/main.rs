use std::io;

fn conv_f_to_c(f_value:f64) -> f64{
    (f_value-32.0) * (5.0/9.0)
}

fn conv_c_to_f(c_value:f64) -> f64{
    ((9.0/5.0) * (c_value)) + 32.0
}

fn main() {
    println!("Temperature Converter");
    let mut choice:String = String::new();
    let mut user_value:String  = String::new();
    loop{
        choice.clear();
        user_value.clear();
        println!("1.Convert Temperature from Celcius to Fahrenheit?\n2.Convert Temperature from Fahrenheit to Celcius?\n3.Exit");
        io::stdin().read_line(&mut choice).expect("Incorrect option");
        let choice: u32 = match choice.trim().parse(){
            Ok(num) => num,
            Err(_) => continue,
        };

        if choice == 3{
            println!("Exiting");
            break;
        }

        println!("Enter the temperature:");
        io::stdin().read_line(&mut user_value).expect("Incorrect option");
        let user_value: f64 = match user_value.trim().parse(){
            Ok(num) => num,
            Err(_) => continue,
        };

        
        

        match choice{
            1 => {
                let result:f64 = conv_c_to_f(user_value);
                println!("Your temperature in Fahrenheit is: {}", result);
            },
            2 => {
                let result:f64 = conv_f_to_c(user_value);
                println!("Your temperature in Celcius is: {}", result);
            },
            _ => {
                println!("Wrong option");
                continue;
            },
        }
    }
}

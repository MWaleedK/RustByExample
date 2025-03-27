use std::io;


fn main() {
    let mut current:u64 = 0;
    let mut next:u64 = 1;
    let mut _temp: u64 = 0;

    println!("Calculating the fibonacci sequence, enter the index you want the sequence to go till");
    let mut user_input:String = String::new();

    io::stdin().read_line(&mut user_input).expect("Error with readline");

    let user_input:u64 = match user_input.trim().parse(){
        Ok(num) =>num,
        Err(_) => 0,
    };
    if user_input ==0 {
        println!("Your sequence: {}", current);
    }
    else{
        println!("Your sequence is:");
        for _i in 0..user_input{
            println!("{}",current);
            _temp = current;
            current = next;
            next = current+_temp;
        }

    }
}

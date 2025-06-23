fn find_nemo(string: String) {
    let mut count: u32 = 0;

    for word in string.split_whitespace() {
        count += 1;
        if word.to_lowercase() == "nemo" {
            println!("I found Nemo at position {}", count);
            return;
        }
    }

    println!("Nemo was not found.");
}

fn main() {
    find_nemo(String::from("Where is Nemo"));
}

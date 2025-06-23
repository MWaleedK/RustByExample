fn pair_in_drawer(my_socks: String) -> u16 {
    let mut num_pairs: usize = 0;
    let mut alphabet_vec = vec![0; 26];
    for chars in my_socks.chars() {
        let ascii_value = chars as u8;
        let index = ascii_value - 65;
        alphabet_vec[index as usize] += 1;
    }
    for pairs in 0..alphabet_vec.len() {
        num_pairs += alphabet_vec[pairs] / 2;
    }

    num_pairs as u16
}

fn main() {
    println!(
        "I have a total of {} pairs",
        pair_in_drawer("ABBAA".to_string())
    )
}

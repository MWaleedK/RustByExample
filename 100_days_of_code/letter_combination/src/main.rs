struct Solution;

impl Solution {
    pub fn letter_combinations(digits: String) -> Vec<String> {
        if digits.is_empty() {
            return vec![];
        }

        let phone_map = vec!["abc", "def", "ghi", "jkl", "mno", "pqrs", "tuv", "wxyz"];
        let mut output = Vec::new();

        fn backtrack(
            combination: String,
            next_digits: &str,
            phone_map: &Vec<&str>,
            output: &mut Vec<String>,
        ) {
            if next_digits.is_empty() {
                output.push(combination);
            } else {
                let index = next_digits.chars().next().unwrap() as u8 - b'2';
                let letters = phone_map[index as usize];
                for letter in letters.chars() {
                    let new_combination = combination.clone() + &letter.to_string();
                    backtrack(new_combination, &next_digits[1..], phone_map, output);
                }
            }
        }

        backtrack(String::new(), &digits, &phone_map, &mut output);
        output
    }
}

fn main() {
    let input_digits = String::from("28");
    let lc = Solution::letter_combinations(input_digits);
    println!("{:?}", lc);
}

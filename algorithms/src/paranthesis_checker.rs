mod stack;
use stack::Stack;
pub struct Paranthesis_Checker {}

impl Paranthesis_Checker {
    fn para_match(open: char, close: char) -> bool {
        let opens = "([{";
        let closers = ")]}";
        match (opens.find(open), closers.find(close)) {
            (Some(i), Some(j)) => i == j,
            _ => false,
        }
    }

    pub fn check_para(string: &str) -> bool {
        let mut char_list = Vec::new();
        for c in string.chars() {
            char_list.push(c);
        }
        let mut index = 0;
        let mut stack = Stack::new();
        let mut balanced = true;
        while index < char_list.len() && balanced {
            let c = char_list[index];
            if c == '(' || c == '{' || c == '[' {
                stack.push(char_list[index]);
            }
            if ')' == c || c == ']' || c == '}' {
                if stack.is_empty() {
                    balanced = false;
                } else {
                    let top = stack.pop().unwrap();
                    if !Self::para_match(top, c) {
                        balanced = false
                    }
                }
            }
            index += 1;
        }
        balanced && stack.is_empty()
    }
}

mod anagrams;
use anagrams::Anagram;
mod shared_stack;
use shared_stack::baseconverter::BaseConverter;
use shared_stack::infix_to_postfix::InfixToPostfix;

fn main() {
    let infix = "( A + B) * (C + D)";
    let mut i_to_p = InfixToPostfix::convert(infix);
    match i_to_p {
        Some(val) => {
            println!("{infix}->{val}");
        }
        None => {
            println!("{infix} is not acorrect infix string");
        }
    }
}

mod anagrams;
use anagrams::Anagram;
mod shared_stack;
use shared_stack::baseconverter::BaseConverter;
use shared_stack::postfix_eval::PostfixEval;

fn main() {
    let postfix = "1 2 + 1 2 + *";
    let p_eval = PostfixEval::postfix_eval(postfix);
    match p_eval {
        Some(val) => println!("res={}", val),
        None => println!("{postfix} isn't a valid postfix"),
    }
}

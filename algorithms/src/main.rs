mod anagrams;
use anagrams::Anagram;
mod paranthesis_checker;
use paranthesis_checker::Paranthesis_Checker;
fn main() {
    let p = Paranthesis_Checker::check_para("{[]}");
    println!("{}", p);
}

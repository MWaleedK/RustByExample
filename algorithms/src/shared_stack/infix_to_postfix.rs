use super::paranthesis_checker::Paranthesis_Checker;
use super::stack::Stack;
use std::collections::HashMap;

pub struct InfixToPostfix;

impl InfixToPostfix {
    pub fn convert(infix: &str) -> Option<String> {
        if !Paranthesis_Checker::check_para(infix) {
            return None;
        }

        let mut prec = HashMap::new();
        prec.insert("(", 1);
        prec.insert(")", 1);
        prec.insert("+", 2);
        prec.insert("-", 2);
        prec.insert("*", 3);
        prec.insert("/", 3);

        let mut ops = Stack::new();
        let mut postfix = Vec::new();

        for token in Self::tokenize(infix) {
            if token.chars().all(|c| c.is_ascii_alphanumeric()) {
                postfix.push(token.to_string());
            } else if token == "(" {
                ops.push(token.to_string());
            } else if token == ")" {
                while let Some(top) = ops.pop() {
                    if top == "(" {
                        break;
                    }
                    postfix.push(top);
                }
            } else {
                while let Some(top) = ops.peek() {
                    let top_prec = prec.get(top.as_str());
                    let token_prec = prec.get(token.as_str());

                    match (top_prec, token_prec) {
                        (Some(&tp), Some(&cp)) if tp >= cp => {
                            postfix.push(ops.pop().unwrap());
                        }
                        _ => break,
                    }
                }
                ops.push(token.to_string());
            }
        }

        while let Some(op) = ops.pop() {
            postfix.push(op);
        }

        Some(postfix.join(" ") + " ")
    }

    fn tokenize(expr: &str) -> Vec<String> {
        let mut tokens = Vec::new();
        let mut curr = String::new();

        for ch in expr.chars() {
            if ch.is_whitespace() {
                continue;
            }
            if "()*/+-".contains(ch) {
                if !curr.is_empty() {
                    tokens.push(curr.clone());
                    curr.clear();
                }
                tokens.push(ch.to_string());
            } else {
                curr.push(ch);
            }
        }

        if !curr.is_empty() {
            tokens.push(curr);
        }

        tokens
    }
}

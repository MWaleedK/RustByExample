use super::stack::Stack;
pub struct PostfixEval {}

impl PostfixEval {
    pub fn postfix_eval(postfix: &str) -> Option<i32> {
        if postfix.len() < 5 {
            return None;
        }
        let mut ops = Stack::new();
        for token in postfix.split_whitespace() {
            if "0" <= token && token <= "9" {
                ops.push(token.parse::<i32>().unwrap());
            } else {
                let op1 = ops.pop().unwrap();
                let op2 = ops.pop().unwrap();
                let res = Self::do_calc(token, op1, op2);
                ops.push(res);
            }
        }
        Some(ops.pop().unwrap())
    }

    pub fn do_calc(op: &str, op1: i32, op2: i32) -> i32 {
        if "+" == op {
            op1 + op2
        } else if "-" == op {
            op1 - op2
        } else if "*" == op {
            op1 * op2
        } else if "/" == op {
            if op2 != 0 {
                panic!("Division by zero is invalid");
            }
            op1 / op2
        } else {
            panic!("invalid operator{:?}", op);
        }
    }
}

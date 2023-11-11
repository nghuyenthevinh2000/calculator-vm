#![allow(clippy::only_used_in_recursion)]

use calculator_ast_parser::{Compile, Node, Operator, Result, Sign};

// ANCHOR: interpreter
pub struct Interpreter;

impl Compile for Interpreter {
    type Output = Result<f64>;

    // f64 computation
    fn from_ast(ast: Vec<Node>) -> Self::Output {
        let mut ret = 0 as f64;
        let evaluator = Eval::new();
        for node in ast {
            ret += evaluator.eval(&node);
        }
        Ok(ret)
    }
}
// ANCHOR_END: interpreter

// ANCHOR: interpreter_recursive
struct Eval;

impl Eval {
    pub fn new() -> Self {
        Self
    }
    // ANCHOR: interpreter_eval
    pub fn eval(&self, node: &Node) -> f64 {
        match node {
            Node::Number(n) => *n,
            Node::UnaryExpr { op, child } => {
                let child = self.eval(child);
                match op {
                    Sign::Positive => child,
                    Sign::Negative => -child,
                }
            }
            Node::BinaryExpr { op, lhs, rhs } => {
                let lhs_ret = self.eval(lhs);
                let rhs_ret = self.eval(rhs);

                match op {
                    Operator::Add => lhs_ret + rhs_ret,
                    Operator::Sub => lhs_ret - rhs_ret,
                    Operator::Mul => lhs_ret * rhs_ret,
                    Operator::Div => lhs_ret / rhs_ret,
                }
            }
        }
    }
    // ANCHOR_END: interpreter_eval
}
// ANCHOR_END: interpreter_recursive

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basics() {
        assert_eq!(Interpreter::from_source("1 + 2").unwrap().to_string(), "3");
        // assert_eq!(Interpreter::source("(1 + 2)").unwrap() as i32, 3);
        assert_eq!(Interpreter::from_source("2 + (2 - 1)").unwrap().to_string(), "3");
        assert_eq!(Interpreter::from_source("(2 + 3) - 1").unwrap().to_string(), "4");
        assert_eq!(
            Interpreter::from_source("1 + ((2 + 3) - (2 + 3))").unwrap().to_string(),
            "1"
        );

        // float number
        assert_eq!(Interpreter::from_source("0.5 + 0.3").unwrap().to_string(), "0.8");
        assert_eq!(Interpreter::from_source("0.5 + 0.3 + 1").unwrap().to_string(), "1.8");
    }
    #[test]
    fn multiply() {
        // multiplication
        assert_eq!(Interpreter::from_source("0.5 * 0.3").unwrap().to_string(), "0.15");
        assert_eq!(Interpreter::from_source("1 + 0.5 * 0.3").unwrap().to_string(), "1.15");
        assert_eq!(Interpreter::from_source("0.7*0.8 + 0.5* (0.3 - 4 + 5)").unwrap().to_string(), "1.21");

        //division
        // 0.5 here will return error, strange though, but acceptable
        assert_eq!(Interpreter::from_source("5/(3+7*1)").unwrap().to_string(), "0.50");
    }
}

#![allow(clippy::only_used_in_recursion)]
use rust_decimal::{Decimal, prelude::FromPrimitive};

use calculator_ast_parser::{Compile, Node, Operator, Result, Sign};

// ANCHOR: interpreter
pub struct Interpreter;

impl Compile for Interpreter {
    type Output = Result<f64>;

    // f64 computation
    fn from_ast(ast: Vec<Node>) -> Self::Output {
        let mut ret: f64 = 0f64;
        let evaluator = Eval::new();
        for node in ast {
            evaluator.eval(&node);
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
    pub fn eval(&self, node: &Node) -> Decimal {
        match node {
            Node::Int(n) => Decimal::from_i32(*n).unwrap(),
            Node::Float(n) => *n,
            Node::UnaryExpr { op, child } => {
                let child = self.eval(child);
                match op {
                    Sign::Plus => child,
                    Sign::Minus => -child,
                }
            }
            Node::BinaryExpr { op, lhs, rhs } => {
                let lhs_ret = self.eval(lhs);
                let rhs_ret = self.eval(rhs);

                match op {
                    Operator::Plus => lhs_ret + rhs_ret,
                    Operator::Minus => lhs_ret - rhs_ret,
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
        assert_eq!(Interpreter::from_source("1 + 2").unwrap(), 3);
        // assert_eq!(Interpreter::source("(1 + 2)").unwrap() as i32, 3);
        assert_eq!(Interpreter::from_source("2 + (2 - 1)").unwrap() as i32, 3);
        assert_eq!(Interpreter::from_source("(2 + 3) - 1").unwrap() as i32, 4);
        assert_eq!(
            Interpreter::from_source("1 + ((2 + 3) - (2 + 3))").unwrap() as i32,
            1
        );

        // float number
        assert_eq!(Interpreter::from_source("0.5 + 0.3").unwrap() as f64, 0.8);
        assert_eq!(Interpreter::from_source("0.5 + 0.3 + 1").unwrap() as f64, 1.8);

        // multiplication

        //division

        // power
    }
}

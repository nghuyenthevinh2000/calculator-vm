use std::fmt::{self};
use rust_decimal::Decimal;

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum Operator {
    Plus,
    Minus,
    Mul,
    Div
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum Sign {
    Positive,
    Negative,
}

impl fmt::Display for Operator {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        match &self {
            Operator::Plus => write!(f, "+"),
            Operator::Minus => write!(f, "-"),
            Operator::Mul => write!(f, "*"),
            Operator::Div => write!(f, "/")
        }
    }
}

impl fmt::Display for Sign {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        match &self {
            Sign::Positive => write!(f, "+"),
            Sign::Negative => write!(f, "-"),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
// ANCHOR: node
// node to construct AST tree
pub enum Node {
    Number(Decimal),
    UnaryExpr {
        op: Sign,
        child: Box<Node>,
    },
    BinaryExpr {
        op: Operator,
        lhs: Box<Node>,
        rhs: Box<Node>,
    },
}
// ANCHOR_END: node

impl fmt::Display for Node {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        match &self {
            Node::Number(n) => write!(f, "{}", n),
            Node::UnaryExpr { op, child } => write!(f, "{}{}", op, child),
            Node::BinaryExpr { op, lhs, rhs } => write!(f, "{} {} {}", lhs, op, rhs),
        }
    }
}
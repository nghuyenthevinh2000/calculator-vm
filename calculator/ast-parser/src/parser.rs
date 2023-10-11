#![allow(clippy::upper_case_acronyms, clippy::result_large_err)]

use pest::{self, Parser, pratt_parser::PrattParser, iterators::Pairs};
use rust_decimal::Decimal;

use crate::{ast::{Node, Operator}, Sign};

// https://pest.rs/book/precedence.html?highlight=prefix#operator-precedence
lazy_static::lazy_static! {
    static ref PRATT_PARSER: PrattParser<Rule> = {
        use pest::pratt_parser::{Assoc::*, Op};
        use Rule::*;

        // Precedence is defined lowest to highest
        PrattParser::new()
            // Addition and subtract have equal precedence
            .op(Op::prefix(positive) | Op::prefix(negative))
            .op(Op::infix(plus, Left) | Op::infix(minus, Left))
            .op(Op::infix(mul, Left) | Op::infix(div, Left))
    };
}

// ANCHOR: parser
#[derive(pest_derive::Parser)]
#[grammar = "grammar.pest"]
struct CalcParser;
// ANCHOR_END: parser

// ANCHOR: parse_source
pub fn parse(source: &str) -> std::result::Result<Vec<Node>, pest::error::Error<Rule>> {
    let mut ast = vec![];
    let pairs = CalcParser::parse(Rule::Program, source).unwrap_or_else(
        |e| {
            panic!("error = {:?}", e);
        }
    );

    for pair in pairs {
        if let Rule::Expr = pair.as_rule() {
            ast.push(parse_binary_expr(pair.into_inner()));
        }
    }
    Ok(ast)
}

pub fn parse_binary_expr(pairs: Pairs<Rule>) -> Node {
    PRATT_PARSER
        .map_primary(|primary| {            
            match primary.as_rule() {
                Rule::Expr => parse_binary_expr(primary.into_inner()),
                Rule::UnaryExpr => parse_binary_expr(primary.into_inner()),
                Rule::BinaryExpr => parse_binary_expr(primary.into_inner()),
                Rule::Number => Node::Number(primary.as_str().parse::<Decimal>().unwrap()),
                rule => unreachable!("Expr::parse expected atom, found {:?}", rule)
}})
        .map_infix(|lhs, op, rhs| {
            let op = match op.as_rule() {
                Rule::plus => Operator::Plus,
                Rule::minus => Operator::Minus,
                Rule::mul => Operator::Mul,
                Rule::div => Operator::Div,
                rule => unreachable!("Expr::parse expected infix operation, found {:?}", rule),
            };
            Node::BinaryExpr { 
                op, 
                lhs: Box::new(lhs), 
                rhs: Box::new(rhs) 
            }
        })
        .map_prefix(|op, n| {
            let op = match op.as_rule() {
                Rule::positive => Sign::Positive,
                Rule::negative => Sign::Negative,
                rule => unreachable!("Expr::parse expected prefix operation, found {:?}", rule),
            };

            Node::UnaryExpr { 
                op, 
                child: Box::new(n)
            }
        })
        .parse(pairs)
}

#[cfg(test)]
mod tests {
    use rust_decimal::prelude::FromPrimitive;

    use super::*;
    #[test]
    #[should_panic]
    fn basics() {
        let _ = parse("b");
    }

    #[test]
    fn unary_expr() {
        let plus_one = parse("+1");
        assert!(plus_one.is_ok());
        assert_eq!(
            plus_one.clone().unwrap(),
            vec![Node::UnaryExpr {
                op: Sign::Positive,
                child: Box::new(Node::Number(Decimal::from_i32(1).unwrap()))
            }]
        );
        assert_eq!(format!("{}", plus_one.unwrap()[0]), "+1");

        let neg_two = parse("-2");
        assert!(neg_two.is_ok());
        assert_eq!(
            neg_two.clone().unwrap(),
            vec![Node::UnaryExpr {
                op: Sign::Negative,
                child: Box::new(Node::Number(Decimal::from_i32(2).unwrap()))
            }]
        );
        assert_eq!(format!("{}", neg_two.unwrap()[0]), "-2");
    }
    #[test]
    fn parse_float_unary_expr() {
        let positive = parse("+1.235");
        assert!(positive.is_ok());
        assert_eq!(
            positive.clone().unwrap(),
            vec![Node::UnaryExpr {
                op: Sign::Positive,
                child: Box::new(Node::Number(Decimal::from_f64(1.235).unwrap()))
            }]
        );
        assert_eq!(format!("{}", positive.unwrap()[0]), "+1.235");

        let negative = parse("-0.78");
        assert!(negative.is_ok());
        assert_eq!(
            negative.clone().unwrap(),
            vec![Node::UnaryExpr { 
                op: Sign::Negative, 
                child: Box::new(Node::Number(Decimal::from_f64(0.78).unwrap()))
            }]
        );
        assert_eq!(format!("{}", negative.unwrap()[0]), "-0.78");
    }
    #[test]
    fn binary_expr() {
        let sum = parse("1.7 + 2");
        assert!(sum.is_ok());
        assert_eq!(
            sum.clone().unwrap(),
            vec![Node::BinaryExpr {
                op: Operator::Plus,
                lhs: Box::new(Node::Number(Decimal::from_f64(1.7).unwrap())),
                rhs: Box::new(Node::Number(Decimal::from_i32(2).unwrap()))
            }]
        );
        assert_eq!(format!("{}", sum.unwrap()[0]), "1.7 + 2");
        
        let minus = parse("1.7   -  \t  2");
        assert!(minus.is_ok());
        assert_eq!(
            minus.clone().unwrap(),
            vec![Node::BinaryExpr {
                op: Operator::Minus,
                lhs: Box::new(Node::Number(Decimal::from_f64(1.7).unwrap())),
                rhs: Box::new(Node::Number(Decimal::from_i32(2).unwrap()))
            }]
        );
        assert_eq!(format!("{}", minus.unwrap()[0]), "1.7 - 2");
        // fails as there's no rhs:
        // let paran_sum = parse("(1 + 2)");
        // assert!(paran_sum.is_ok());

        let mul = parse("3.56 * 4");
        assert!(mul.is_ok());
        assert_eq!(
            mul.clone().unwrap(),
            vec![Node::BinaryExpr { 
                op: Operator::Mul, 
                lhs: Box::new(Node::Number(Decimal::from_f64(3.56).unwrap())), 
                rhs: Box::new(Node::Number(Decimal::from_i32(4).unwrap())) 
            }]
        );
        assert_eq!(format!("{}", mul.unwrap()[0]), "3.56 * 4");

        let div = parse("3/4");
        assert!(div.is_ok());
        assert_eq!(
            div.clone().unwrap(),
            vec![Node::BinaryExpr { 
                op: Operator::Div, 
                lhs: Box::new(Node::Number(Decimal::from_i32(3).unwrap())), 
                rhs: Box::new(Node::Number(Decimal::from_i32(4).unwrap())) 
            }]
        );
        assert_eq!(format!("{}", div.unwrap()[0]), "3 / 4");

    }

    #[test]
    fn nested_expr() {
        fn test_expr(expected: &str, src: &str) {
            assert_eq!(
                expected,
                parse(src)
                    .unwrap()
                    .iter()
                    .fold(String::new(), |acc, arg| acc + &format!("{}", &arg))
            );
        }

        test_expr("1 + 2 + 3", "(1 + 2) + 3");
        test_expr("1 + 2 + 3", "1 + (2 + 3)");
        test_expr("1 + 2 + 3 + 4", "1 + (2 + (3 + 4))");
        test_expr("1 + 2 + 3 - 4", "(1 + 2) + (3 - 4)");
        test_expr("1 + 0.5 * 0.3", "1 + (0.5 * 0.3)")
    }
    #[test]
    fn mixed_mul_add() {
        assert_eq!(
            parse("1 + 0.5 * 0.3").unwrap(),
            vec![Node::BinaryExpr { 
                op: Operator::Plus, 
                lhs: Box::new(Node::Number(Decimal::from_i32(1).unwrap())), 
                rhs: Box::new(Node::BinaryExpr {
                    op: Operator::Mul,
                    lhs: Box::new(Node::Number(Decimal::from_f32(0.5).unwrap())),
                    rhs: Box::new(Node::Number(Decimal::from_f32(0.3).unwrap()))
                }),
            }]
        )
    }

    #[test]
    fn multiple_operators() {
        assert_eq!(
            parse("1+2+3").unwrap(),
            vec![Node::BinaryExpr {
                op: Operator::Plus,
                lhs: Box::new(Node::BinaryExpr {
                    op: Operator::Plus,
                    lhs: Box::new(Node::Number(Decimal::from_i32(1).unwrap())),
                    rhs: Box::new(Node::Number(Decimal::from_i32(2).unwrap())),
                }),
                rhs: Box::new(Node::Number(Decimal::from_i32(3).unwrap())),
            }]
        )
    }

}

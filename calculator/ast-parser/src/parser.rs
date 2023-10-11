#![allow(clippy::upper_case_acronyms, clippy::result_large_err)]

use pest::{self, Parser};
use rust_decimal::{Decimal, prelude::FromPrimitive};

use crate::{ast::{Node, Operator}, Sign};

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
            ast.push(build_ast_from_expr(pair));
        }
    }
    Ok(ast)
}
// ANCHOR_END: parse_source

fn build_ast_from_expr(pair: pest::iterators::Pair<Rule>) -> Node {
    match pair.as_rule() {
        Rule::Expr => build_ast_from_expr(pair.into_inner().next().unwrap()),
        Rule::UnaryExpr => {
            let mut pair = pair.into_inner();
            let op = pair.next().unwrap();
            let child = pair.next().unwrap();
            let child = build_ast_from_term(child);
            parse_unary_expr(op, child)
        }
        Rule::BinaryExpr => {
            let mut pair = pair.into_inner();
            let lhspair = pair.next().unwrap();
            let mut lhs = build_ast_from_term(lhspair);
            let op = pair.next().unwrap();
            let rhspair = pair.next().unwrap();
            let mut rhs = build_ast_from_term(rhspair);
            let mut retval = parse_binary_expr(op, lhs, rhs);
            loop {
                let pair_buf = pair.next();
                if let Some(op) = pair_buf {
                    lhs = retval;
                    rhs = build_ast_from_term(pair.next().unwrap());
                    retval = parse_binary_expr(op, lhs, rhs);
                } else {
                    return retval;
                }
            }
        }
        unknown => panic!("Unknown expr: {:?}", unknown),
    }
}

fn build_ast_from_term(pair: pest::iterators::Pair<Rule>) -> Node {
    match pair.as_rule() {
        Rule::Int => {
            let istr = pair.as_str();
            let (sign, istr) = match &istr[..1] {
                "-" => (-1, &istr[1..]),
                _ => (1, istr),
            };
            let int: i32 = istr.parse().unwrap();
            Node::Int(sign * int)
        }
        Rule::Float => {
            let fstr: &str = pair.as_str();
            let (sign, fstr) = match &fstr[..1] {
                "-" => (-1.0, &fstr[1..]),
                _ => (1.0, fstr),
            };
            let float: f64 = fstr.parse().unwrap();
            let a = Decimal::from_f64(float).unwrap();
            let b = Decimal::from_f64(sign).unwrap();
            Node::Float(a.checked_mul(b).unwrap())
        }
        Rule::Expr => build_ast_from_expr(pair),
        unknown => panic!("Unknown term: {:?}", unknown),
    }
}

fn parse_unary_expr(pair: pest::iterators::Pair<Rule>, child: Node) -> Node {
    Node::UnaryExpr {
        op: match pair.as_str() {
            "+" => Sign::Plus,
            "-" => Sign::Minus,
            _ => unreachable!(),
        },
        child: Box::new(child),
    }
}

fn parse_binary_expr(pair: pest::iterators::Pair<Rule>, lhs: Node, rhs: Node) -> Node {
    Node::BinaryExpr {
        op: match pair.as_str() {
            "+" => Operator::Plus,
            "-" => Operator::Minus,
            "*" => Operator::Mul,
            "/" => Operator::Div,
            _ => unreachable!(),
        },
        lhs: Box::new(lhs),
        rhs: Box::new(rhs),
    }
}

#[cfg(test)]
mod tests {
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
                op: Sign::Plus,
                child: Box::new(Node::Int(1))
            }]
        );
        assert_eq!(format!("{}", plus_one.unwrap()[0]), "+1");

        let neg_two = parse("-2");
        assert!(neg_two.is_ok());
        assert_eq!(
            neg_two.clone().unwrap(),
            vec![Node::UnaryExpr {
                op: Sign::Minus,
                child: Box::new(Node::Int(2))
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
                op: Sign::Plus,
                child: Box::new(Node::Float(Decimal::from_f64(1.235).unwrap()))
            }]
        );
        assert_eq!(format!("{}", positive.unwrap()[0]), "+1.235");

        let negative = parse("-0.78");
        assert!(negative.is_ok());
        assert_eq!(
            negative.clone().unwrap(),
            vec![Node::UnaryExpr { 
                op: Sign::Minus, 
                child: Box::new(Node::Float(Decimal::from_f64(0.78).unwrap()))
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
                lhs: Box::new(Node::Float(Decimal::from_f64(1.7).unwrap())),
                rhs: Box::new(Node::Int(2))
            }]
        );
        assert_eq!(format!("{}", sum.unwrap()[0]), "1.7 + 2");
        
        let minus = parse("1.7   -  \t  2");
        assert!(minus.is_ok());
        assert_eq!(
            minus.clone().unwrap(),
            vec![Node::BinaryExpr {
                op: Operator::Minus,
                lhs: Box::new(Node::Float(Decimal::from_f64(1.7).unwrap())),
                rhs: Box::new(Node::Int(2))
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
                lhs: Box::new(Node::Float(Decimal::from_f64(3.56).unwrap())), 
                rhs: Box::new(Node::Int(4)) 
            }]
        );
        assert_eq!(format!("{}", mul.unwrap()[0]), "3.56 * 4");

        let div = parse("3/4");
        assert!(div.is_ok());
        assert_eq!(
            div.clone().unwrap(),
            vec![Node::BinaryExpr { 
                op: Operator::Div, 
                lhs: Box::new(Node::Int(3)), 
                rhs: Box::new(Node::Int(4)) 
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
    }

    #[test]
    fn multiple_operators() {
        assert_eq!(
            parse("1+2+3").unwrap(),
            vec![Node::BinaryExpr {
                op: Operator::Plus,
                lhs: Box::new(Node::BinaryExpr {
                    op: Operator::Plus,
                    lhs: Box::new(Node::Int(1)),
                    rhs: Box::new(Node::Int(2)),
                }),
                rhs: Box::new(Node::Int(3)),
            }]
        )
    }

}

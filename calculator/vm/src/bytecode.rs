use std::convert::TryInto;

use calculator_ast_parser::{Compile, Node, Sign, Operator};

use crate::opcode::OpCode;

#[derive(Debug, Clone, PartialEq, Eq)]
// ANCHOR: bytecode
pub struct Bytecode {
    pub instructions: Vec<u8>,
}

impl Bytecode {
    fn new() -> Self {
        Self {
            instructions: Vec::new()
        }
    }

    // based on position 0 or 1, we will get the constant value
    // if lhs: 0
    // if not lhs, which is rhs: 1
    pub fn bytes_to_constants(&self, position: usize) -> (f64, usize) {
        // this is to skip the first code sign (0x01) for constant
        // no need for further evaluation since our calculator can only accept constant anyway
        let first = position;
        let end = position + 8;
        let bytes: [u8;8] = self.instructions[first..end].try_into().unwrap();
        (f64::from_be_bytes(bytes), end)
    }
}

#[derive(Debug)]
pub struct Interpreter {
    bytecode: Bytecode
}

impl Interpreter {
    fn new() -> Self {
        Self{
            bytecode: Bytecode::new()
        }
    }

    // adding instructions
    fn add_instructions(&mut self, opcode: OpCode) {
        self.bytecode.instructions.extend(opcode.bytes())
    }

    fn eval_node(&mut self, n: Node) {
        match n {
            Node::Number(dec) => self.add_instructions(OpCode::OpConstant(dec)),
            Node::UnaryExpr { op, child } => {
                self.eval_node(*child);

                match op {
                    Sign::Positive => self.add_instructions(OpCode::OpPlus),
                    Sign::Negative => self.add_instructions(OpCode::OpMinus)
                }
            }
            Node::BinaryExpr { op, lhs, rhs } => {
                self.eval_node(*lhs);
                self.eval_node(*rhs);

                match op {
                    Operator::Add => self.add_instructions(OpCode::OpAdd),
                    Operator::Sub => self.add_instructions(OpCode::OpSub),
                    Operator::Mul => self.add_instructions(OpCode::OpMul),
                    Operator::Div => self.add_instructions(OpCode::OpDiv)
                }
            }
        }
    }
}

impl Compile for Interpreter {
    type Output = Bytecode;

    fn from_ast(ast: Vec<calculator_ast_parser::Node>) -> Self::Output {
        let mut intepreter = Interpreter::new();

        // travserse ast tree
        for n in ast {
            intepreter.eval_node(n);

            // add end of instruction
            intepreter.add_instructions(OpCode::OpPop);
        }

        intepreter.bytecode.to_owned()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basics() {
        infix_template("+", OpCode::OpAdd);
        infix_template("-", OpCode::OpSub);
    }

    fn infix_template(infix_str: &str, op_code: OpCode) {
        let input = format!("1 {} 2;", infix_str);
        let bytecode = Interpreter::from_source(&input);

        let expected_instructions = vec![
            OpCode::OpConstant(1.0),
            OpCode::OpConstant(2.0),
            op_code,
            OpCode::OpPop,
        ]
        .into_iter()
        .flat_map(|a| a.bytes())
        .collect();

        assert_eq!(
            Bytecode {
                instructions: expected_instructions,
            },
            bytecode
        );
    }
}
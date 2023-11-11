use calculator_ast_parser::Node;

use crate::bytecode::Bytecode;

const STACK_SIZE: usize = 512;
pub struct VM {
    bytecode: Bytecode,
    stack: [Node; STACK_SIZE],
    stack_ptr: usize
}

impl VM {
    pub fn new(bytecode: Bytecode) -> Self {
        Self {
            bytecode,
            stack: unsafe { std::mem::zeroed() },
            stack_ptr: 0
        }
    }

    pub fn run(&mut self) {
        // instruction pointer
        let mut ip = 0;
        // fetch instructions
        while ip < self.bytecode.instructions.len() {
            let opcode = self.bytecode.instructions[ip];
            ip += 1;

            match opcode {
                0x01 => {
                    let (val, next) = self.bytecode.bytes_to_constants(ip);
                    ip = next;
                    self.push(Node::Number(val));
                },
                0x02 => _ = self.pop(),
                0x03 => {
                    match (self.pop(), self.pop()) {
                        (Node::Number(rhs), Node::Number(lhs)) => self.push(Node::Number(lhs + rhs)),
                        _ => panic!("Unknown types to OpAdd"),
                    }
                }
                0x04 => {
                    match (self.pop(), self.pop()) {
                        (Node::Number(rhs), Node::Number(lhs)) => self.push(Node::Number(lhs - rhs)),
                        _ => panic!("Unknown types to OpSub"),
                    }
                }
                0x05 => {
                    match (self.pop(), self.pop()) {
                        (Node::Number(rhs), Node::Number(lhs)) => self.push(Node::Number(lhs * rhs)),
                        _ => panic!("Unknown types to OpMul"),
                    }
                }
                0x06 => {
                    match (self.pop(), self.pop()) {
                        (Node::Number(rhs), Node::Number(lhs)) => self.push(Node::Number(lhs / rhs)),
                        _ => panic!("Unknown types to OpDiv"),
                    }
                }
                0x0A => {
                    match self.pop() {
                        Node::Number(child) => self.push(Node::Number(child)),
                        _ => panic!("Unknown types to OpPlus"),
                    }
                }
                0x0B => {
                    match self.pop() {
                        Node::Number(child) => self.push(Node::Number(-child)),
                        _ => panic!("Unknown types to OpMinus"),
                    }
                }
                _ => panic!("unrecognized opcode")
            }
        }
    }

    fn push(&mut self, node: Node) {
        self.stack[self.stack_ptr] = node;
        self.stack_ptr += 1;
    }

    // original stack is preserved
    fn pop(&mut self) -> Node {
        let node = self.stack[self.stack_ptr - 1].clone();
        self.stack_ptr -= 1;
        node
    }

    pub fn pop_last(&self) -> &Node {
        // the stack pointer points to the next "free" space,
        // which also holds the most recently popped element
        &self.stack[self.stack_ptr]
    }

    pub fn get_result(&self) -> f64 {
        let node = self.pop_last();
        match node {
            Node::Number(dec) => *dec,
            Node::BinaryExpr { op, lhs, rhs } => panic!("not a number"),
            Node::UnaryExpr { op, child } => panic!("not a"),
        }
    }
}

#[cfg(test)]
mod tests {
    use calculator_ast_parser::Compile;

    use crate::bytecode::Interpreter;

    use super::*;

    fn assert_pop_last(source: &str, node: Node) {
        let byte_code = Interpreter::from_source(source);
        println!("byte code: {:?}", byte_code);
        let mut vm = VM::new(byte_code);
        vm.run();
        assert_eq!(&node, vm.pop_last());
    }

    #[test]
    fn unary() {
        assert_pop_last("+1", Node::Number(1.0));
        assert_pop_last("-2", Node::Number(-2.0));
    }

    #[test]
    fn binary() {
        assert_pop_last("1 + 2;", Node::Number(3.0));
        assert_pop_last("1 - 2;", Node::Number(-1.0));
    }
}
use std::default;

// define op code
#[derive(Debug, Copy, Clone)]
pub enum OpCode {
    OpConstant(f64), // pointer to constant table
    OpPop,           // pop is needed for execution 
    OpAdd,
    OpSub,
    OpMul,
    OpDiv,
    OpPlus,
    OpMinus
}

// const byte op will be in this format [0x01, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff]
fn make_const_byte_op(code: u8, data: f64) -> Vec<u8> {
    let mut output = vec![code];
    output.extend(data.to_be_bytes());
    output
}

impl OpCode {
    pub fn bytes(self) -> Vec<u8>{
        match self {
            OpCode::OpConstant(arg) => make_const_byte_op(0x01, arg),
            OpCode::OpPop => vec![0x02],  // decimal repr is 2
            OpCode::OpAdd => vec![0x03],  // decimal repr is 3
            OpCode::OpSub => vec![0x04],  // decimal repr is 4
            OpCode::OpMul => vec![0x05],  // decimal repr is 5
            OpCode::OpDiv => vec![0x06],  // decimal repr is 6
            OpCode::OpPlus => vec![0x0A], // decimal repr is 10
            OpCode::OpMinus => vec![0x0B], // decimal repr is 11
        }
    }
}

impl From::<u8> for OpCode {
    fn from(value: u8) -> Self {
        match value {
            0x02 => OpCode::OpPop,
            0x03 => OpCode::OpAdd,
            0x04 => OpCode::OpSub,
            0x05 => OpCode::OpMul,
            0x06 => OpCode::OpDiv,
            0x0A => OpCode::OpPlus,
            0x0B => OpCode::OpMinus,
            _ => panic!("not recognized opcode")
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn make_op_constant() {
        assert_eq!(vec![0x01, 64, 239, 255, 192, 0, 0, 0, 0], OpCode::OpConstant(65534.0).bytes());
    }

    #[test]
    fn make_op_pop() {
        assert_eq!(vec![0x02], OpCode::OpPop.bytes());
    }

    #[test]
    fn make_op_add() {
        assert_eq!(vec![0x03], OpCode::OpAdd.bytes());
    }
}
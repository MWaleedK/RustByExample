#[derive(Debug, PartialEq)]
pub enum Opcode {
    LOAD, //LOAD
    ADD,  //ADDITION
    SUB,  //SUBTRACTION
    MUL,  //MULTIPLICATION
    DIV,  //DIVISION
    HLT,  //HALT
    JMP,  //JUMP
    JMPF, // JUMP 5 spaces forwards
    JMPB, //JUMP 5 spaced backwards
    EQ,   //EQUAL
    NEQ,  //NOT EQUAL
    GTE,  //GREATER THAN EQUAL
    LTE,  //LESS THAN EQUAL
    LT,   //LESS THAN
    GT,   //GREATER THAN
    JEQ,  //JUMP IF EQUAL
    JNEQ, //JUMP IF NOT EQUAL
    IGL,  //ILLEGAL
}

impl From<u8> for Opcode {
    fn from(v: u8) -> Self {
        match v {
            0 => return Opcode::LOAD,
            1 => return Opcode::ADD,
            2 => return Opcode::SUB,
            3 => return Opcode::MUL,
            4 => return Opcode::DIV,
            5 => return Opcode::HLT,
            6 => return Opcode::JMP,
            7 => return Opcode::JMPF,
            8 => return Opcode::JMPB,
            9 => return Opcode::EQ,
            10 => return Opcode::NEQ,
            11 => return Opcode::GTE,
            12 => return Opcode::LTE,
            13 => return Opcode::LT,
            14 => return Opcode::GT,
            15 => return Opcode::JEQ,
            16 => return Opcode::JNEQ,
            _ => return Opcode::IGL,
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct Instruction {
    opcode: Opcode,
}

impl Instruction {
    pub fn new(opcode: Opcode) -> Instruction {
        Instruction { opcode: opcode }
    }
}

//**************TESTS*******************
mod tests {
    #[allow(unused_imports)]
    use super::*;

    #[test]
    fn test_create_hlt() {
        let opcode = Opcode::HLT;
        assert_eq!(opcode, Opcode::HLT);
    }

    #[test]
    fn test_create_instruction() {
        let instruction = Instruction::new(Opcode::HLT);
        assert_eq!(instruction.opcode, Opcode::HLT);
    }
}

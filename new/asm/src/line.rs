use crate::instruction::{Instruction, OperandType, INSTRUCTION_LIST};
use crate::register::Register;

/// Methods related to machine code encoding
pub mod encode;

/// Assembly line information
#[derive(Clone, Copy, Debug)]
pub enum Line<'a> {
    None,
    Label(&'a str),
    AsmCommand(&'a str),
    Instruction(&'a str),
    UnKnown(&'a str),
}

impl<'a> Line<'a> {
    /// Split instruction and return mnemonic and operands
    /// (mnemonic, operand1, operand2)
    pub fn split_instruction(self) -> Option<(&'a str, [Option<&'a str>; 2])> {
        if let Line::Instruction(s) = self {
            let mut s_split = s.trim().split(' ');

            let mnemonic = s_split.next().expect("unknown error");
            let operand1 = s_split.next();
            let operand2 = s_split.next();

            if s_split.next().is_none() {
                Some((mnemonic, [operand1, operand2]))
            } else {
                None
            }
        } else {
            None
        }
    }

    /// Get mneonic
    pub fn mnemonic(self) -> Option<&'a str> {
        Some(self.split_instruction()?.0)
    }

    /// Get operands
    pub fn operands(self) -> Option<[Option<&'a str>; 2]> {
        Some(self.split_instruction()?.1)
    }

    /// Get instruction information
    pub fn get_instruction(self) -> Option<Instruction> {
        for i in INSTRUCTION_LIST {
            if i.match_with(&self) {
                return Some(*i);
            }
        }
        None
    }

    fn get_operand_by_type(self, operand_type: OperandType) -> Option<&'a str> {
        let instruction = self.get_instruction()?;
        let operand_index = instruction
            .expression()
            .get_operand_index_by_type(operand_type)?;
        self.operands()?[operand_index]
    }

    fn reg_operand_helper(self, operand_type: OperandType) -> Option<Register> {
        if let Ok(r) = self.get_operand_by_type(operand_type)?.parse::<Register>() {
            Some(r)
        } else {
            None
        }
    }

    /// Get r8 operand
    pub fn r8_operand(self) -> Option<Register> {
        let register = self.reg_operand_helper(OperandType::R8)?;
        if register.is_8bit() {
            Some(register)
        } else {
            None
        }
    }

    /// Get r16 operand
    pub fn r16_operand(self) -> Option<Register> {
        let register = self.reg_operand_helper(OperandType::R16)?;
        if register.is_16bit() {
            Some(register)
        } else {
            None
        }
    }

    /// Get r32 operand
    pub fn r32_operand(self) -> Option<Register> {
        let register = self.reg_operand_helper(OperandType::R32)?;
        if register.is_32bit() {
            Some(register)
        } else {
            None
        }
    }

    /// Get r64 operand
    pub fn r64_operand(self) -> Option<Register> {
        let register = self.reg_operand_helper(OperandType::R64)?;
        if register.is_64bit() {
            Some(register)
        } else {
            None
        }
    }
}

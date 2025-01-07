use crate::{
    functions::parse_rm,
    instruction::{Instruction, OperandType, INSTRUCTION_LIST},
    register::Register,
};
use util::functions::result_to_option;

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

    /// Get register operand
    pub fn register_operand(self) -> Option<Register> {
        let expression: &str = self
            .get_operand_by_type(OperandType::R8)
            .or_else(|| self.get_operand_by_type(OperandType::R16))
            .or_else(|| {
                self.get_operand_by_type(OperandType::R32)
                    .or_else(|| self.get_operand_by_type(OperandType::R64))
            })
            .expect("invalid operation");
        result_to_option(expression.parse())
    }

    /// Get rm refering operand
    pub fn rm_ref_operand(self) -> Option<(i32, Register, Option<(Register, u8)>)> {
        let operand: &str = self
            .get_operand_by_type(OperandType::Rm8)
            .or_else(|| self.get_operand_by_type(OperandType::Rm16))
            .or_else(|| self.get_operand_by_type(OperandType::Rm32))
            .or_else(|| self.get_operand_by_type(OperandType::Rm64))?;

        parse_rm(operand)
    }

    /// Get rm register operand
    pub fn rm_register_operand(self) -> Option<Register> {
        let operand: &str = self
            .get_operand_by_type(OperandType::Rm8)
            .or_else(|| self.get_operand_by_type(OperandType::Rm16))
            .or_else(|| self.get_operand_by_type(OperandType::Rm32))
            .or_else(|| self.get_operand_by_type(OperandType::Rm64))
            .expect("invalid operation");

        result_to_option(operand.parse())
    }
}

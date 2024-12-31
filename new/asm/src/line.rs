use crate::instruction::{AddRegRule, Instruction, OperandType, INSTRUCTION_LIST};
use crate::register::Register;
use util::svec::SVec;

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

    pub fn get_instruction(self) -> Option<Instruction> {
        for i in INSTRUCTION_LIST {
            if i.match_with(&self) {
                return Some(*i);
            }
        }
        None
    }

    fn reg_operand_helper(self, operand_type: OperandType) -> Option<Register> {
        let instruction = self.get_instruction()?;
        let operand_index = instruction
            .expression()
            .get_operand_index_by_type(OperandType::R8)?;
        let operands = self.operands()?;

        if let Ok(r) = operands[operand_index]?.parse::<Register>() {
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

// Encode
impl<'a> Line<'a> {
    /// Get opecode in raw machine code
    pub fn opecode(self) -> Option<SVec<3, u8>> {
        let instruction = self.get_instruction()?;
        let mut opecode = instruction.encoding().opecode();

        let opecode_len = opecode.len();
        opecode[opecode_len - 1] += self
            .addreg_regcode()
            .or(Some((None, 0)))
            .expect("unknown error")
            .1;

        Some(opecode)
    }

    fn opecode_len(self) -> Option<usize> {
        Some(self.opecode()?.len())
    }

    fn addreg_regcode(self) -> Option<(Option<bool>, u8)> {
        let instruction = self.get_instruction()?;
        let addreg_rule = instruction.encoding().addreg();

        match addreg_rule {
            None => None,
            Some(AddRegRule::R8) => self.r8_operand()?.to_regcode8(),
            Some(AddRegRule::R16) => self.r16_operand()?.to_regcode16(),
            Some(AddRegRule::R32) => self.r32_operand()?.to_regcode32(),
            Some(AddRegRule::R64) => self.r64_operand()?.to_regcode64(),
        }
    }

    pub fn rex(self) -> Option<u8> {
        if let Some(addreg_regcode) = self.addreg_regcode() {
            todo!()
        } else {
            todo!()
        }
    }
}

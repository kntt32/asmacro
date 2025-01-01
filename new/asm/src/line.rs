use crate::instruction::{
    AddRegRule, ImmRule, Instruction, OperandType, RexRule, INSTRUCTION_LIST,
};
use crate::register::Register;
use std::mem::transmute;
use util::functions::stoi;
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

    fn imm_operand_helper(self, operand_type: OperandType) -> Option<i128> {
        stoi(self.get_operand_by_type(operand_type)?)
    }

    /// Get imm8 operand
    pub fn imm8_operand(self) -> Option<u8> {
        let value = self.imm_operand_helper(OperandType::Imm8)?;
        if i8::MIN as i128 <= value && value < 0 {
            Some(unsafe { transmute::<i8, u8>(value as i8) })
        } else if value <= u8::MAX as i128 {
            Some(value as u8)
        } else {
            None
        }
    }

    /// Get imm16 operand
    pub fn imm16_operand(self) -> Option<u16> {
        let value = self.imm_operand_helper(OperandType::Imm16)?;
        if i16::MIN as i128 <= value && value < 0 {
            Some(unsafe { transmute::<i16, u16>(value as i16) })
        } else if value <= u16::MAX as i128 {
            Some(value as u16)
        } else {
            None
        }
    }

    /// Get imm32 operand
    pub fn imm32_operand(self) -> Option<u32> {
        let value = self.imm_operand_helper(OperandType::Imm32)?;
        if i32::MIN as i128 <= value && value < 0 {
            Some(unsafe { transmute::<i32, u32>(value as i32) })
        } else if value <= u32::MAX as i128 {
            Some(value as u32)
        } else {
            None
        }
    }

    /// Get imm64 operand
    pub fn imm64_operand(self) -> Option<u64> {
        let value = self.imm_operand_helper(OperandType::Imm64)?;
        if i64::MIN as i128 <= value && value < 0 {
            Some(unsafe { transmute::<i64, u64>(value as i64) })
        } else if value <= u64::MAX as i128 {
            Some(value as u64)
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
        let instruction = self.get_instruction().expect("invalid operation");
        let addreg_rule = instruction.encoding().addreg_rule();

        match addreg_rule {
            None => None,
            Some(AddRegRule::R8) => self.r8_operand()?.to_regcode8(),
            Some(AddRegRule::R16) => self.r16_operand()?.to_regcode16(),
            Some(AddRegRule::R32) => self.r32_operand()?.to_regcode32(),
            Some(AddRegRule::R64) => self.r64_operand()?.to_regcode64(),
        }
    }

    fn rex_rule(self) -> Option<RexRule> {
        self.get_instruction()
            .expect("invalid operation")
            .encoding()
            .rex_rule()
    }

    fn imm_rule(self) -> Option<ImmRule> {
        self.get_instruction()
            .expect("invalid operation")
            .encoding()
            .imm_rule()
    }

    /// Get rex prefix in raw machine code
    pub fn rex(self) -> Option<u8> {
        let mut rex_w = false;
        let rex_r = false;
        let rex_x = false;
        let mut rex_b = false;
        let rexrule_existing_flag = match self.rex_rule() {
            None => false,
            Some(RexRule::Rex) => true,
            Some(RexRule::RexW) => {
                rex_w = true;
                true
            }
        };

        if let Some(addreg_regcode) = self.addreg_regcode() {
            if let Some(b) = addreg_regcode.0 {
                rex_b = b;
            } else {
                return None;
            }
        }

        todo!()
    }

    /// Get Imm in raw machine code
    pub fn imm(self) -> Option<SVec<8, u8>> {
        match self.imm_rule() {
            None => None,
            _ => todo!(),
        }
    }
}

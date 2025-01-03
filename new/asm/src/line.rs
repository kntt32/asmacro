use crate::instruction::{
    AddRegRule, Instruction, OperandSize, OperandType, INSTRUCTION_LIST, ModRmRule
};
use crate::register::Register;
use std::cmp::max;
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
}

// Encode
impl<'a> Line<'a> {
    /// Get raw machine code
    pub fn machine_code(self) -> SVec<13, u8> {
        let mut svec = SVec::new();
        svec += self.legacy_prefix();
        svec += self.rex_prefix();
        svec += self.opecode();
        // coding ...
        svec += self.imm();
        svec
    }

    /// Get opecode in raw machine code
    pub fn opecode(self) -> SVec<3, u8> {
        let instruction = self.get_instruction().expect("invalid operation");
        let mut opecode = instruction.encoding().opecode();

        let opecode_len = opecode.len();
        opecode[opecode_len - 1] += self
            .addreg_regcode()
            .or(Some((None, 0)))
            .expect("unknown error")
            .1;

        opecode
    }

    fn addreg_regcode(self) -> Option<(Option<bool>, u8)> {
        let instruction = self.get_instruction().expect("invalid operation");
        let addreg_rule = instruction.encoding().addreg_rule();

        match addreg_rule {
            None => None,
            Some(AddRegRule::Rb) => self.r8_operand()?.to_regcode8(),
            Some(AddRegRule::Rw) => self.r16_operand()?.to_regcode16(),
            Some(AddRegRule::Rd) => self.r32_operand()?.to_regcode32(),
            Some(AddRegRule::Rq) => self.r64_operand()?.to_regcode64(),
        }
    }

    fn modrm_register_regcode(self) -> Option<(Option<bool>, u8)> {
        let instruction = self.get_instruction().expect("invalid operation");
        let encoding = instruction.encoding();

        match encoding.modrm_rule() {
            None => None,
            Some(ModRmRule::R) => {
                let register = self.r8_operand().or_else(|| self.r16_operand()).or_else(|| self.r32_operand()).or_else(|| self.r64_operand()).expect("unknown error");
                Some(register.to_regcode8().or(register.to_regcode16()).or(register.to_regcode32()).or(register.to_regcode64()).expect("unknown error"))
            }
            Some(ModRmRule::Dight(i)) => Some((Some(false), i)),
        }
    }

    fn modrm_base_regcode(self) -> Option<(Option<bool>, u8)> {
        let instruction = self.get_instruction().expect("invalid operation");
        let encoding = instruction.encoding();
        
        todo!()
    }

    fn rex_prefix_is_required(self) -> bool {
        let instruction = self.get_instruction().expect("invalid operation");
        let encoding = instruction.encoding();
        let default_operand_size = encoding.default_operand_size();
        assert!(OperandSize::Od <= default_operand_size);

        if let Some(operand_size) = self.operand_size() {
            default_operand_size < operand_size
        } else {
            false
        }
    }

    fn prefix_x66_is_required(self) -> bool {
        if let Some(operand_size) = self.operand_size() {
            operand_size == OperandSize::Ob
        } else {
            false
        }
    }

    fn operand_size(self) -> Option<OperandSize> {
        let instruction = self.get_instruction().expect("invalid operation");
        let expression = instruction.expression();
        let operands_types = expression.operands();
        max(
            operands_types[0].map(OperandType::size),
            operands_types[1].map(OperandType::size),
        )
    }

    /// Get legacy prefix in raw machine code
    pub fn legacy_prefix(self) -> SVec<1, u8> {
        let mut svec = SVec::new();
        if self.prefix_x66_is_required() {
            svec.push(0x66);
        }
        svec
    }

    /// Get rex prefix in raw machine code
    pub fn rex_prefix(self) -> SVec<1, u8> {
        let mut rex_w = false;
        let mut rex_r = false;
        let rex_x = false;
        let rex_b = false;

        rex_w = if self.rex_prefix_is_required() {
            true
        } else {
            false
        };

        let addreg_regcode = self.addreg_regcode();
        let modrm_register_regcode = self.modrm_register_regcode();

        if let Some((Some(true), _)) = addreg_regcode {
            rex_r = true;
        }
        if let Some((Some(true), _)) = modrm_register_regcode {
            rex_r = true;
        }

        // coding ...

        let mut rex_prefix = SVec::new();

        if rex_w || rex_r || rex_x || rex_b {
            if let Some((None, _)) = addreg_regcode {
                panic!("unknown error");
            }
            if let Some((None, _)) = modrm_register_regcode {
                panic!("unknown error");
            }

            rex_prefix.push(0x40);
            if rex_w {
                rex_prefix[0] |= 0x08;
            }
            if rex_r {
                rex_prefix[0] |= 0x04;
            }
            if rex_x {
                rex_prefix[0] |= 0x02;
            }
            if rex_b {
                rex_prefix[0] |= 0x01;
            }
            rex_prefix
        } else {
            rex_prefix
        }
    }

    /// Get Imm in raw machine code
    pub fn imm(self) -> SVec<8, u8> {
        let imm_rule = self
            .get_instruction()
            .expect("invalid operation")
            .encoding()
            .imm_rule();
        match imm_rule {
            None => SVec::new(),
            Some(i) => {
                let operand_type = i.operand_type();
                let value = stoi(
                    self.get_operand_by_type(operand_type)
                        .expect("invalid operation"),
                )
                .expect("invalid operation");
                let value = unsafe { transmute::<i128, u128>(value) };
                SVec::from_value(value, operand_type.size().value())
            }
        }
    }
}

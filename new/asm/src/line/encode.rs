use super::Line;
use crate::{
    instruction::{ModRmRule, OperandSize, OperandType},
    register::{Register, RegisterCode},
};
use std::{cmp::max, mem::transmute};
use util::{functions::stoi, svec::SVec};

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
            .opecode_register_code()
            .or(Some((None, 0)))
            .expect("unknown error")
            .1;

        opecode
    }

    fn opecode_register_code(self) -> Option<RegisterCode> {
        let instruction = self.get_instruction().expect("invalid operation");
        let opecode_register_rule = instruction.encoding().opecode_register_rule();

        match opecode_register_rule {
            Some(_) => Some(
                self.register_operand()
                    .expect("invalid operation")
                    .register_code_for_opecode_register(),
            ),
            None => None,
        }
    }

    fn modrm_register_regcode(self) -> RegisterCode {
        let instruction = self.get_instruction().expect("invalid operation");
        let encoding = instruction.encoding();

        match encoding.modrm_rule() {
            None => panic!("modrm field doesn't exist"),
            Some(ModRmRule::R) => {
                let register = self.register_operand().expect("invalid operation");
                register.register_code_for_opecode_register()
            }
            Some(ModRmRule::Dight(i)) => (Some(false), i),
        }
    }

    fn modrm_ref_base(self) -> Option<Register> {
        let (_, base, _) = self.rm_ref_operand()?;
        Some(base)
    }

    fn modrm_base_regcode(self) -> RegisterCode {
        if let Some(r) = self.rm_register_operand() {
            r.register_code_for_opecode_register()
        } else {
            self.modrm_ref_base()
                .expect("invalid operation")
                .register_code_for_rm_ref_base()
        }
    }

    fn modrm_ref_index(self) -> Option<Register> {
        if let (_, _, Some((index, _))) = self.rm_ref_operand()? {
            Some(index)
        } else {
            None
        }
    }

    fn modrm_index_regcode(self) -> Option<RegisterCode> {
        Some(self.modrm_ref_index()?.register_code_for_rm_ref_index())
    }

    fn modrm_scale(self) -> Option<u8> {
        if let (_, _, Some((_, scale))) = self.rm_ref_operand().expect("invalid operation") {
            Some(scale)
        } else {
            None
        }
    }

    fn modrm_disp(self) -> i32 {
        let (disp, _, _) = self.rm_ref_operand().expect("invalid operation");
        disp
    }

    fn modrm_mode(self) -> u8 {
        let modrm_ref_base = self.modrm_ref_base();
        match modrm_ref_base {
            Some(Register::Rip) => 0b00,
            Some(r) => {
                let modrm_disp = self.modrm_disp();
                let disp_is_8bit = i8::MIN as i32 <= modrm_disp && modrm_disp <= i8::MAX as i32;
                let disp_isnt_exist = modrm_disp == 0
                    && r != Register::Rbp
                    && r != Register::R13
                    && !(self.sib_require() && (r == Register::Rbp || r == Register::R13));
                if disp_isnt_exist {
                    0b00
                } else if disp_is_8bit {
                    0b01
                } else {
                    0b10
                }
            }
            None => 0b11,
        }
    }

    fn modrm_exist(self) -> bool {
        let instruction = self.get_instruction().expect("invalid operation");
        let encoding = instruction.encoding();
        encoding.modrm_rule().is_some()
    }

    fn sib_require(self) -> bool {
        let modrm_ref_base = self.modrm_ref_base();
        self.modrm_ref_index().is_some()
            || modrm_ref_base == Some(Register::Rsp)
            || modrm_ref_base == Some(Register::R12)
    }

    pub fn modrm(self) -> SVec<1, u8> {
        if self.modrm_exist() {
            let mode = self.modrm_mode();
            let (_, reg) = self.modrm_register_regcode();
            let base = if self.sib_require() {
                0b100
            } else {
                self.modrm_base_regcode().1
            };
            let modrm = (mode << 6) | (reg << 3) | base;
            SVec::from([modrm])
        } else {
            return SVec::new();
        }
    }

    pub fn sib(self) -> SVec<1, u8> {
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
        let mut rex_x = false;
        let mut rex_b = false;

        if self.rex_prefix_is_required() {
            rex_w = true;
        }

        let modrm_exist = self.modrm_exist();

        let opecode_register_code = self.opecode_register_code();
        let modrm_register_regcode = if modrm_exist {
            Some(self.modrm_register_regcode())
        } else {
            None
        };
        if let Some((Some(true), _)) = modrm_register_regcode {
            rex_r = true;
        }
        if let Some((Some(true), _)) = opecode_register_code {
            rex_r = true;
        }
        let modrm_base_regcode = if modrm_exist {
            Some(self.modrm_base_regcode())
        } else {
            None
        };
        let modrm_index_regcode = self.modrm_index_regcode();
        if let Some((Some(true), _)) = modrm_base_regcode {
            rex_b = true;
        }
        if let Some((Some(true), _)) = modrm_index_regcode {
            rex_x = true;
        }

        let mut rex_prefix = SVec::new();

        if rex_w || rex_r || rex_x || rex_b {
            if let Some((None, _)) = opecode_register_code {
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

use super::Line;
use crate::{
    functions::parse_rm,
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

    fn modrm_register_regcode(self) -> Option<(Option<bool>, u8)> {
        let instruction = self.get_instruction().expect("invalid operation");
        let encoding = instruction.encoding();

        match encoding.modrm_rule() {
            None => None,
            Some(ModRmRule::R) => {
                todo!();
                /*let register = self
                    .r8_operand()
                    .or_else(|| self.r16_operand())
                    .or_else(|| self.r32_operand())
                    .or_else(|| self.r64_operand())
                    .expect("unknown error");
                Some(
                    register
                        .to_regcode8()
                        .or(register.to_regcode16())
                        .or(register.to_regcode32())
                        .or(register.to_regcode64())
                        .expect("unknown error"),
                )*/
            }
            Some(ModRmRule::Dight(i)) => Some((Some(false), i)),
        }
    }

    fn modrm_parse_rm(self) -> (i32, Register, Option<(Register, u8)>) {
        let operand: &str = self
            .get_operand_by_type(OperandType::Rm8)
            .or_else(|| self.get_operand_by_type(OperandType::Rm16))
            .or_else(|| self.get_operand_by_type(OperandType::Rm32))
            .or_else(|| self.get_operand_by_type(OperandType::Rm32))
            .expect("invalid operation");

        parse_rm(operand).expect("invalid operation")
    }

    fn modrm_base_regcode(self) -> (Option<bool>, u8) {
        let (_, base, _) = self.modrm_parse_rm();
        base.to_regcode()
    }

    fn modrm_index_regcode(self) -> Option<(Option<bool>, u8)> {
        if let (_, _, Some((index, _))) = self.modrm_parse_rm() {
            Some(index.to_regcode())
        } else {
            None
        }
    }

    fn modrm_scale(self) -> Option<u8> {
        if let (_, _, Some((_, scale))) = self.modrm_parse_rm() {
            Some(scale)
        } else {
            None
        }
    }

    fn modrm_disp(self) -> i32 {
        let (disp, _, _) = self.modrm_parse_rm();
        disp
    }
    /*
        fn modrm_mode(self) -> ModRmMode {

        }
    */
    /*
        pub fn modrm(self) -> SVec<1, u8> {
            let instruction = self.get_instruction().expect("invalid operation");
            let encoding = instruction.encoding();
            match encoding.modrm_rule() {
                None => SVec::new(),
                Some(ModRmRule::R) | Some(ModRmRule::Dight(_)) => {
                    let modrm =
                },
            }
        }

        pub fn sib(self) -> SVec<1, u8> {

        }
    */
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

        let opecode_register_code = self.opecode_register_code();
        let modrm_register_regcode = self.modrm_register_regcode();

        if let Some((Some(true), _)) = opecode_register_code {
            rex_r = true;
        }
        if let Some((Some(true), _)) = modrm_register_regcode {
            rex_r = true;
        }

        // coding ...

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

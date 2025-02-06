use super::Line;
use crate::{
    assembler::Label,
    functions::Relocation,
    instruction::{ImmRule, ModRmRule, OperandSize, OperandType},
    register::{Register, RegisterCode},
};
use std::{cmp::max, mem::transmute};
use util::svec::SVec;

impl<'a> Line<'a> {
    /// Get raw machine code
    pub fn machine_code(self, labels: &[Label<'a>], offset: usize) -> Result<SVec<19, u8>, String> {
        let mut svec = SVec::new();
        svec += self.legacy_prefix(); //1
        svec += self.rex_prefix(); //1
        svec += self.opecode(); //3
        svec += self.modrm(); //1
        svec += self.sib(); //1
        svec += self.disp(labels, offset)?; //4
        svec += self.imm(labels, offset)?; //8
        Ok(svec)
    }

    /// Get raw machine code length
    pub fn machine_code_len(self) -> usize {
        let mut len = 0;

        len += self.legacy_prefix_len();
        len += self.rex_prefix_len();
        len += self.opecode_len();
        len += self.modrm_len();
        len += self.sib_len();
        len += self.disp_len();
        len += self.imm_len();

        len
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

    fn opecode_len(self) -> usize {
        let instruction = self.get_instruction().expect("invalid operation");
        let opecode = instruction.encoding().opecode();
        opecode.len()
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

    fn modrm_disp(self) -> Relocation<'a, i32> {
        let (disp, _, _) = self.rm_ref_operand().expect("invalid operation");
        disp
    }

    fn modrm_mode(self) -> u8 {
        let modrm_ref_base = self.modrm_ref_base();
        match modrm_ref_base {
            Some(Register::Rip) => 0b00,
            Some(r) => {
                let modrm_disp = self.modrm_disp();
                let disp_is_8bit;
                let disp_isnt_exist;
                if let Relocation::Value(d) = modrm_disp {
                    disp_is_8bit = i8::MIN as i32 <= d && d <= i8::MAX as i32;
                    disp_isnt_exist = d == 0
                        && r != Register::Rbp
                        && r != Register::R13
                        && !(self.sib_exist() && (r == Register::Rbp || r == Register::R13));
                } else {
                    disp_is_8bit = false;
                    disp_isnt_exist = false;
                }

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

    fn disp_len(self) -> usize {
        if self.modrm_exist() {
            match self.modrm_mode() {
                0b00 | 0b11 => 0,
                0b01 => 1,
                0b10 => 4,
                _ => panic!("invalid output"),
            }
        } else {
            0
        }
    }

    fn modrm_exist(self) -> bool {
        let instruction = self.get_instruction().expect("invalid operation");
        let encoding = instruction.encoding();
        encoding.modrm_rule().is_some()
    }

    fn sib_exist(self) -> bool {
        let modrm_ref_base = self.modrm_ref_base();
        self.modrm_ref_index().is_some()
            || modrm_ref_base == Some(Register::Rsp)
            || modrm_ref_base == Some(Register::R12)
    }

    pub fn modrm(self) -> SVec<1, u8> {
        if self.modrm_exist() {
            let mode = self.modrm_mode();
            let (_, reg) = self.modrm_register_regcode();
            let base = if self.sib_exist() {
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

    fn modrm_len(self) -> usize {
        if self.modrm_exist() {
            1
        } else {
            0
        }
    }

    pub fn sib(self) -> SVec<1, u8> {
        if self.sib_exist() {
            let (_, base) = self.modrm_base_regcode();
            let (_, index) = self
                .modrm_index_regcode()
                .or(Some((None, 0b100)))
                .expect("invalid operation");
            let scale: u8 = match self.modrm_scale() {
                Some(1) => 0b00,
                Some(2) => 0b01,
                Some(4) => 0b10,
                Some(8) => 0b11,
                _ => 0b00,
            };

            let sib = (scale << 6) | (index << 3) | base;
            SVec::from([sib])
        } else {
            SVec::new()
        }
    }

    fn sib_len(self) -> usize {
        if self.sib_exist() {
            1
        } else {
            0
        }
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

    fn legacy_prefix_len(self) -> usize {
        self.legacy_prefix().len()
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

    fn rex_prefix_len(self) -> usize {
        self.rex_prefix().len()
    }

    fn imm_len(self) -> usize {
        let imm_rule = self
            .get_instruction()
            .expect("invalid operation")
            .encoding()
            .imm_rule();

        match imm_rule {
            None => 0,
            Some(i) => match i {
                ImmRule::Ib => 1,
                ImmRule::Iw => 2,
                ImmRule::Id => 4,
                ImmRule::Iq => 8,
            },
        }
    }

    /// Get Imm in raw machine code
    pub fn imm(self, labels: &[Label<'a>], offset: usize) -> Result<SVec<8, u8>, String> {
        let imm_rule = self
            .get_instruction()
            .expect("invalid operation")
            .encoding()
            .imm_rule();
        match imm_rule {
            None => Ok(SVec::new()),
            Some(_) => {
                let imm: i128 = self
                    .imm_operand()
                    .expect("invalid operation")
                    .relocate_imm(labels, offset + self.machine_code_len())?;
                let imm_usize: u128 = unsafe { transmute::<i128, u128>(imm) };
                let imm_len = self.imm_len();
                Ok(SVec::from_value(imm_usize, imm_len))
            }
        }
    }

    /// Get Disp in raw machine code
    pub fn disp(self, labels: &[Label<'a>], offset: usize) -> Result<SVec<4, u8>, String> {
        let disp_len = self.disp_len();
        if disp_len == 0 {
            Ok(SVec::new())
        } else {
            let disp = self.modrm_disp().relocate_disp(labels, offset)?;
            let disp_usize = unsafe { transmute::<i128, u128>(disp as i128) };
            Ok(SVec::from_value(disp_usize, disp_len))
        }
    }
}

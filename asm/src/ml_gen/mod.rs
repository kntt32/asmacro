use crate::encoder::*;
use util::svec::SVec;
use helper_types::RegCode;

mod helper_types;


/// 
#[derive(Clone, Copy, Debug)]
pub struct OpDescpritor {
    rex_prefix: bool,
    opecode: SVec<3, u8>, // opecode before add regcode
    operand: Operand, // operand
}

impl OpDescpritor {
    pub fn new(rex_prefix: bool, opecode: SVec<3, u8>, operand: Operand) -> Self {
        OpDescpritor { rex_prefix: rex_prefix, opecode: opecode, operand: operand }
    }

    pub fn encode(&self) -> Result<SVec<18, u8>, ()> {
        let mut encoder = Encoder::new();

        //rex
        if self.rex_prefix {
            encoder.rex_prefix.enable();
            encoder.rex_prefix.set_w(true);
        }

        //opecode
        encoder.opecode.set(self.opecode);

        //operand
        self.set_operand(&mut encoder)?;

        Ok(encoder.encode())
    }

    fn set_operand(&self, encoder: &mut Encoder) -> Result<(), ()> {
        match self.operand {
            Operand::None => {
                Ok(())
            },
            Operand::R64Rm64(reg64, rm64) => self.set_modrm(encoder, reg64, rm64),
            Operand::R64Imm64(reg64, imm64) => self.set_reg_imm(encoder, reg64, imm64),
        }
    }

    fn set_modrm(&self, encoder: &mut Encoder, reg64: Reg64, rm64: Rm64) -> Result<(), ()> {
        let mut mod_rm = ModRm::new();

        // reg64
        encoder.rex_prefix.enable();
        encoder.rex_prefix.set_w(true);
        let regcode = reg64.to_regcode_for_modrm_reg()?;
        encoder.rex_prefix.set_r(regcode.rex);
        mod_rm.set_reg(regcode.reg);

        // rm64
        match rm64 {
            Rm64::R64(reg64) => {
                let regcode = reg64.to_regcode_for_modrm_rm_reg_mod11()?;
                mod_rm.set_mod(0b11);
                mod_rm.set_rm(regcode.reg);
                encoder.rex_prefix.set_b(regcode.rex);
            },
            Rm64::M64(reg64, disp) => {
                match disp {
                    Disp::None => {
                        match reg64 {
                            Reg64::Rip => todo!("Todo"),
                            Reg64::Rsp => todo!("Todo"),
                            Reg64::Rbp => todo!("Todo"),
                            Reg64::R12 => todo!("Todo"),
                            Reg64::R13 => todo!("Todo"),
                            _ => {
                                let regcode = reg64.to_regcode_for_modrm_rm_reg_mod00()?;
                                mod_rm.set_mod(0b00);
                                mod_rm.set_rm(regcode.reg);
                                encoder.rex_prefix.set_b(regcode.rex);
                            },
                        }
                    },
                    Disp::Disp8(disp8) => {
                        match reg64 {
                            Reg64::Rsp => todo!("Todo"),
                            Reg64::R12 => todo!("Todo"),
                            _ => {
                                let regcode = reg64.to_regcode_for_modrm_rm_reg_mod01()?;
                                mod_rm.set_mod(0b01);
                                mod_rm.set_rm(regcode.reg);
                                encoder.rex_prefix.set_b(regcode.rex);
                                encoder.disp = Disp::Disp8(disp8);
                            },
                        }
                    },
                    Disp::Disp32(disp32) => {
                        match reg64 {
                            Reg64::Rsp => todo!("Todo"),
                            Reg64::R12 => todo!("Todo"),
                            _ => {
                                let regcode = reg64.to_regcode_for_modrm_rm_reg_mod10()?;
                                mod_rm.set_mod(0b10);
                                mod_rm.set_rm(regcode.reg);
                                encoder.rex_prefix.set_b(regcode.rex);
                                encoder.disp = Disp::Disp32(disp32);
                            },
                        }
                    },
                }
            },
            Rm64::M64Sib(sib, disp) => {
                todo!("Todo");
            },
        }

        encoder.mod_rm = Some(mod_rm);
        Ok(())
    }

    fn set_reg_imm(&self, encoder: &mut Encoder, reg64: Reg64, imm64: u64) -> Result<(), ()> {
        let regcode = reg64.to_regcode_for_opecode()?;

        // reg64
        let encode_opecode_length = encoder.opecode.len();
        encoder.opecode[encode_opecode_length - 1] += regcode.reg;
        encoder.rex_prefix.enable();
        encoder.rex_prefix.set_w(true);
        encoder.rex_prefix.set_r(regcode.rex);

        // imm64
        encoder.imm = Imm::Imm64(imm64);
        Ok(())
    }
}

#[derive(Clone, Copy, Debug)]
pub enum Operand {
    None,
    R64Rm64(Reg64, Rm64),
    R64Imm64(Reg64, u64),
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Reg64 {
    Rax, Rcx, Rdx, Rbx, Rsp, Rbp, Rsi, Rdi, R8, R9, R10, R11, R12, R13, R14, R15,
    Rip,
}

impl Reg64 {
    fn to_regcode(self) ->  Result<RegCode, ()> {
        match self {
            Reg64::Rax => Ok(RegCode::Rax),
            Reg64::Rcx => Ok(RegCode::Rcx),
            Reg64::Rdx => Ok(RegCode::Rdx),
            Reg64::Rbx => Ok(RegCode::Rbx),
            Reg64::Rsp => Ok(RegCode::Rsp),
            Reg64::Rbp => Ok(RegCode::Rbp),
            Reg64::Rsi => Ok(RegCode::Rsi),
            Reg64::Rdi => Ok(RegCode::Rdi),
            Reg64::R8 => Ok(RegCode::R8),
            Reg64::R9 => Ok(RegCode::R9),
            Reg64::R10 => Ok(RegCode::R10),
            Reg64::R11 => Ok(RegCode::R11),
            Reg64::R12 => Ok(RegCode::R12),
            Reg64::R13 => Ok(RegCode::R13),
            Reg64::R14 => Ok(RegCode::R14),
            Reg64::R15 => Ok(RegCode::R15),
            _ => Err(())
        }
    }

    fn to_regcode_for_opecode(self) -> Result<RegCode, ()> {
        self.to_regcode()
    }

    fn to_regcode_for_modrm_reg(self) -> Result<RegCode, ()> {
        self.to_regcode_for_opecode()
    }

    fn to_regcode_for_modrm_rm_reg_mod11(self) -> Result<RegCode, ()> {
        self.to_regcode_for_opecode()
    }

    fn to_regcode_for_modrm_rm_reg_mod00(self) -> Result<RegCode, ()> {
        match self {
            Reg64::Rsp => Err(()),
            Reg64::Rbp => Err(()),
            Reg64::R12 => Err(()),
            Reg64::R13 => Err(()),
            _ => self.to_regcode(),
        }
    }

    fn to_regcode_for_modrm_rm_reg_mod01(self) -> Result<RegCode, ()> {
        match self {
            Reg64::Rsp => Err(()),
            Reg64::R12 => Err(()),
            _ => self.to_regcode(),
        }
    }

    fn to_regcode_for_modrm_rm_reg_mod10(self) -> Result<RegCode, ()> {
        self.to_regcode_for_modrm_rm_reg_mod01()
    }
}

#[derive(Clone, Copy, Debug)]
pub enum Rm64 {
    R64(Reg64),
    M64(Reg64, Disp),
    M64Sib(Sib, Disp),
}

#[derive(Clone, Copy, Debug)]
pub struct Sib {
    pub scale: u8, // 1:0
    pub base: (Reg64, Disp),
    pub index: Reg64,
}

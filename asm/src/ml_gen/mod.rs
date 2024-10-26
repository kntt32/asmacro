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

    pub fn encode(&self) -> SVec<18, u8> {
        let mut encoder = Encoder::new();

        //rex
        if self.rex_prefix {
            encoder.rex_prefix.enable();
            encoder.rex_prefix.set_w(true);
        }

        //opecode
        encoder.opecode.set(self.opecode);

        //operand
        match self.operand {
            Operand::None => {
                // do nothing
            },
            Operand::R64Rm64(reg64, rm64) => {
                let mut mod_rm = ModRm::new();

                // reg64
                encoder.rex_prefix.enable();
                encoder.rex_prefix.set_w(true);
                let regcode = reg64.to_regcode();
                encoder.rex_prefix.set_r(regcode.rex);
                mod_rm.set_reg(regcode.reg);

                // rm64
                match rm64 {
                    Rm64::R64(reg64) => {
                        let regcode = reg64.to_regcode();
                        mod_rm.set_mod(0b11);
                        mod_rm.set_rm(regcode.reg);
                        encoder.rex_prefix.set_b(regcode.rex);
                    },
                    Rm64::M64(reg64, disp) => {
                        let regcode = reg64.to_regcode();
                        match disp {
                            Disp::None => {
                                mod_rm.set_mod(0b00);
                                if reg64 == Reg64::Rip {
                                    mod_rm.set_rm(0b101);
                                    encoder.rex_prefix.set_b(false);
                                }else {
                                    mod_rm.set_rm(regcode.reg);
                                    encoder.rex_prefix.set_b(regcode.rex);
                                }
                            },
                            Disp::Disp8(disp8) => {
                                todo!("Todo");
                            },
                            Disp::Disp32(disp32) => {
                                todo!("Todo");
                            },
                        }
                    },
                    Rm64::M64Sib(sib, disp) => {

                    },
                }

                encoder.mod_rm = Some(mod_rm);
            },
            Operand::R64Imm64(reg64, imm64) => {
                let regcode = reg64.to_regcode();

                // reg64
                let encode_opecode_length = encoder.opecode.len();
                encoder.opecode[encode_opecode_length - 1] += regcode.reg;
                encoder.rex_prefix.enable();
                encoder.rex_prefix.set_w(true);
                encoder.rex_prefix.set_r(regcode.rex);

                // imm64
                encoder.imm = Imm::Imm64(imm64);
            },
            _ => {
                panic!("Unknown Operand");
            },
        }

        encoder.encode()
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
    fn to_regcode(self) -> RegCode {
        match self {
            Reg64::Rax => RegCode::Rax,
            Reg64::Rcx => RegCode::Rcx,
            Reg64::Rdx => RegCode::Rdx,
            Reg64::Rbx => RegCode::Rbx,
            Reg64::Rsp => RegCode::Rsp,
            Reg64::Rbp => RegCode::Rbp,
            Reg64::Rsi => RegCode::Rsi,
            Reg64::Rdi => RegCode::Rdi,
            Reg64::R8 => RegCode::R8,
            Reg64::R9 => RegCode::R9,
            Reg64::R10 => RegCode::R10,
            Reg64::R11 => RegCode::R11,
            Reg64::R12 => RegCode::R12,
            Reg64::R13 => RegCode::R13,
            Reg64::R14 => RegCode::R14,
            Reg64::R15 => RegCode::R15,
            Reg64::Rip => {
                panic!("Invalid Register")
            }
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub enum Rm64 {
    R64(Reg64),
    M64(Reg64, Disp),
    M64Sib(Sib, Disp),
}

#[derive(Clone, Copy, Debug)]
pub enum Disp {
    None,
    Disp8(u8),
    Disp32(u32),
}

#[derive(Clone, Copy, Debug)]
pub struct Sib {
    pub scale: u8, // 1:0
    pub base: (Reg64, Disp),
    pub index: Reg64,
}

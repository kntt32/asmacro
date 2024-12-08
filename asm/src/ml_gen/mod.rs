use std::mem::transmute;
use util::svec::SVec;

pub use raw_encoder::*;

pub mod raw_encoder;

#[derive(Clone, Copy, Debug)]
pub struct MlGen {
    pub prefix_group3: bool,
    pub prefix_group4: bool,
    pub rex_prefix: RexPrefix,
    pub opecode: Opecode,
    pub rel: Rel,
    pub mod_rm: ModRM,
    pub sib: Sib,
    pub disp: Disp,
    pub imm: Imm,
}

#[derive(Clone, Copy, Debug)]
pub struct DynMlBin {
    mlbin: MlBin,
    index: usize,
    len: usize,
}

impl DynMlBin {
    pub fn build(mut self, disp: isize) -> MlBin {
        for i in 0..self.len {
            self.mlbin[i + self.index] =
                ((unsafe { transmute::<isize, usize>(disp) } >> (i * 8)) & 0xff) as u8;
        }
        self.mlbin
    }
}

pub type MlBin = SVec<22, u8>;

impl MlGen {
    pub fn new() -> Self {
        MlGen {
            prefix_group3: false,
            prefix_group4: false,
            rex_prefix: RexPrefix::None,
            opecode: Opecode::None,
            rel: Rel::None,
            mod_rm: ModRM::None,
            sib: Sib::None,
            disp: Disp::None,
            imm: Imm::None,
        }
    }

    pub fn build(self) -> MlBin {
        self.gen().build(
            self.disp
                .value()
                .or_else(|| Some(0))
                .expect("unknown error"),
        )
    }

    pub fn gen(self) -> DynMlBin {
        let mut dyn_mlbin = DynMlBin {
            mlbin: MlBin::new(),
            index: 0,
            len: 0,
        };
        let ml_svec = &mut dyn_mlbin.mlbin;

        if self.prefix_group3 {
            ml_svec.push(0x66);
        }
        if self.prefix_group4 {
            ml_svec.push(0x67);
        }

        if let RexPrefix::Field(field) = self.rex_prefix {
            ml_svec.push(field);
        }

        if let Opecode::Field(field) = self.opecode {
            if field.len() == 0 {
                panic!("invalid operation");
            }
            for value in field {
                ml_svec.push(value);
            }
        } else {
            panic!("invalid operation");
        }

        if let Rel::Rel32(field) = self.rel {
            for i in 0..4 {
                ml_svec.push(((unsafe { transmute::<i32, u32>(field) } >> (i * 8)) & 0xff) as u8);
            }
        }

        if let ModRM::Field(field) = self.mod_rm {
            ml_svec.push(field);
        }

        if let Sib::Field(field) = self.sib {
            ml_svec.push(field);
        }

        match self.disp {
            Disp::None => (),
            Disp::Disp8(field) => {
                dyn_mlbin.index = ml_svec.len();
                dyn_mlbin.len = 1;
                ml_svec.push(unsafe { transmute::<i8, u8>(field) });
            }
            Disp::Disp32(field) => {
                dyn_mlbin.index = ml_svec.len();
                dyn_mlbin.len = 4;
                for i in 0..4 {
                    ml_svec
                        .push(((unsafe { transmute::<i32, u32>(field) } >> (i * 8)) & 0xff) as u8);
                }
            }
        }

        match self.imm {
            Imm::None => (),
            Imm::Imm8(field) => {
                ml_svec.push(unsafe { transmute::<i8, u8>(field) });
            }
            Imm::Imm16(field) => {
                for i in 0..2 {
                    ml_svec
                        .push(((unsafe { transmute::<i16, u16>(field) } >> (i * 8)) & 0xff) as u8);
                }
            }
            Imm::Imm32(field) => {
                for i in 0..4 {
                    ml_svec
                        .push(((unsafe { transmute::<i32, u32>(field) } >> (i * 8)) & 0xff) as u8);
                }
            }
            Imm::Imm64(field) => {
                for i in 0..8 {
                    ml_svec
                        .push(((unsafe { transmute::<i64, u64>(field) } >> (i * 8)) & 0xff) as u8);
                }
            }
        }

        dyn_mlbin
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RexPrefix {
    None,
    Field(u8),
}

impl RexPrefix {
    pub fn enable(&mut self) {
        *self = Self::Field(0b0100_0000);
    }

    pub fn disable(&mut self) {
        *self = Self::None;
    }

    pub fn is_enabled(self) -> bool {
        if self == Self::None {
            false
        } else {
            true
        }
    }

    pub fn set_w(&mut self, value: bool) {
        if let Self::Field(ref mut field) = self {
            *field &= 0b11110111;
            if value {
                *field |= 0b00001000;
            }
        } else {
            panic!("invalid operation");
        }
    }

    pub fn set_r(&mut self, value: bool) {
        if let Self::Field(ref mut field) = self {
            *field &= 0b11111011;
            if value {
                *field |= 0b00000100;
            }
        } else {
            panic!("invalid operation");
        }
    }

    pub fn set_x(&mut self, value: bool) {
        if let Self::Field(ref mut field) = self {
            *field &= 0b11111101;
            if value {
                *field |= 0b00000010;
            }
        } else {
            panic!("invalid operation");
        }
    }

    pub fn set_b(&mut self, value: bool) {
        if let Self::Field(ref mut field) = self {
            *field &= 0b11111110;
            if value {
                *field |= 0b00000001;
            }
        } else {
            panic!("invalid operation");
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub enum Opecode {
    None,
    Field(SVec<3, u8>),
}

#[derive(Clone, Copy, Debug)]
pub enum Rel {
    None,
    Rel32(i32),
}

impl Opecode {
    pub fn set(&mut self, mut opecode: SVec<3, u8>, register: Option<u8>) {
        if let Some(value) = register {
            let length = opecode.len();
            opecode[length - 1] += value;
        }
        *self = Self::Field(opecode);
    }

    pub fn set_opecode(&mut self, opecode: SVec<3, u8>) {
        *self = Self::Field(opecode);
    }

    pub fn add_register(&mut self, register: u8) {
        if let Self::Field(ref mut field) = self {
            let len = field.len();
            field[len - 1] += register;
        } else {
            panic!("invalid operation")
        }
    }

    pub fn disable(&mut self) {
        *self = Self::None;
    }
}

#[derive(Clone, Copy, Debug)]
pub enum ModRM {
    None,
    Field(u8),
}

impl ModRM {
    pub fn enable(&mut self) {
        *self = Self::Field(0);
    }

    pub fn disable(&mut self) {
        *self = Self::None;
    }

    pub fn is_enabled(self) -> bool {
        match self {
            Self::None => false,
            Self::Field(_) => true,
        }
    }

    pub fn set_mod(&mut self, r#mod: u8) {
        if let Self::Field(ref mut field) = self {
            *field &= 0b00_111_111;
            *field |= (r#mod & 0b11) << 6;
        } else {
            panic!("invalid operation");
        }
    }

    pub fn set_reg(&mut self, reg: u8) {
        if let Self::Field(ref mut field) = self {
            *field &= 0b11_000_111;
            *field |= (reg & 0b111) << 3;
        } else {
            panic!("invalid operation");
        }
    }

    pub fn set_rm(&mut self, rm: u8) {
        if let Self::Field(ref mut field) = self {
            *field &= 0b11_111_000;
            *field |= rm & 0b111;
        } else {
            panic!("invalid operation");
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub enum Sib {
    None,
    Field(u8),
}

impl Sib {
    pub fn enable(&mut self) {
        *self = Self::Field(0);
    }

    pub fn disable(&mut self) {
        *self = Self::None;
    }

    pub fn set_scale(&mut self, scale: u8) {
        if let Self::Field(ref mut field) = self {
            *field &= 0b00_111_111;
            *field |= (scale & 0b11) << 6;
        } else {
            panic!("invalid operation");
        }
    }

    pub fn set_index(&mut self, index: u8) {
        if let Self::Field(ref mut field) = self {
            *field &= 0b11_000_111;
            *field |= (index & 0b111) << 3;
        } else {
            panic!("invalid operation");
        }
    }

    pub fn set_base(&mut self, base: u8) {
        if let Self::Field(ref mut field) = self {
            *field &= 0b11_111_000;
            *field |= base & 0b111;
        } else {
            panic!("invalid operation");
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub enum Disp {
    None,
    Disp8(i8),
    Disp32(i32),
}

impl Disp {
    pub fn value(self) -> Option<isize> {
        match self {
            Self::None => None,
            Self::Disp8(v) => Some(v as isize),
            Self::Disp32(v) => Some(v as isize),
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub enum Imm {
    None,
    Imm8(i8),
    Imm16(i16),
    Imm32(i32),
    Imm64(i64),
}

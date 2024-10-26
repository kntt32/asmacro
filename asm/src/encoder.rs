use std::ops::{Deref, DerefMut};
use util::svec::SVec;

/// Encoder is an encoder for x64 machine language.
/// This is the most low-layer module in asm crate.
/// # Examples
/// ```
/// use asm::encoder::Encoder;
/// let mut encoder = Encoder::new();
///
/// encoder.rex_prefix.enable();
/// encoder.rex_prefix.set_w(true);
/// encoder.rex_prefix.set_r(false);
/// encoder.opecode.push(0xb8 + 0);
/// encoder.imm = Imm::Imm64(123);
///
/// encoder.encode(); // return machine language code
/// ```
/// # Caution
/// - Input to Encoder struct will not be checked.
#[derive(Clone, Copy, Debug)]
pub struct Encoder {
    pub prefix: Option<()>,
    pub rex_prefix: Rex,
    pub opecode: Opecode,
    pub mod_rm: Option<ModRm>,
    pub sib: Option<Sib>,
    pub disp: Disp,
    pub imm: Imm,
}

impl Encoder {
    pub fn new() -> Self {
        Encoder {
            prefix: None,
            rex_prefix: Rex::new(),
            opecode: Opecode::new(),
            mod_rm: None,
            sib: None,
            disp: Disp::None,
            imm: Imm::None,
        }
    }

    pub fn encode(&self) -> SVec<18, u8> {
        let mut binary = SVec::new();

        // prefix
            // unsupported

        // rex prefix
        if let Some(rex_prefix) = self.rex_prefix.get() {
            binary.push(rex_prefix);
        }

        // opecode
        for i in self.opecode.get() {
            binary.push(i);
        }

        // mod_rm
        if let Some(ref mod_rm) = self.mod_rm {
            binary.push(mod_rm.get());
        }

        // sib
        if let Some(ref sib) = self.sib {
            binary.push(sib.get());
        }

        // disp
        match self.disp {
            Disp::None => {},
            Disp::Disp8(disp) => {
                binary.push(disp);
            },
            Disp::Disp16(disp) => {
                for i in 0 .. 2 { binary.push(((disp >> (i*8)) & 0xff) as u8); }
            },
            Disp::Disp32(disp) => {
                for i in 0 .. 4 { binary.push(((disp >> (i*8)) & 0xff) as u8); }
            },
        }

        // imm
        match self.imm {
            Imm::None => {},
            Imm::Imm8(imm) => {
                binary.push(imm);
            },
            Imm::Imm16(imm) => {
                for i in 0 .. 2 { binary.push(((imm >> (i*8)) & 0xff) as u8); }
            },
            Imm::Imm32(imm) => {
                for i in 0 .. 4 { binary.push(((imm >> (i*8)) & 0xff) as u8); }
            },
            Imm::Imm64(imm) => {
                for i in 0 .. 8 { binary.push(((imm >> (i*8)) & 0xff) as u8); }
            },
        }

        binary
    }
}

#[derive(Clone, Copy, Debug)]
pub struct Rex(u8, bool);

impl Rex {
    pub const fn new() -> Self {
        Rex(0b0100_0000, false)
    }

    pub fn enable(&mut self) {
        self.1 = true;
    }

    pub fn set_w(&mut self, w: bool) {
        self.0 &= !0b1000;
        if w {
            self.0 |= 0b1000;
        }
    }

    pub fn set_r(&mut self, r: bool) {
        self.0 &= !0b0100;
        if r {
            self.0 |= 0b0100;
        }
    }

    pub fn set_x(&mut self, x: bool) {
        self.0 &= !0b0010;
        if x {
            self.0 |= 0b0010;
        }
    }

    pub fn set_b(&mut self, b: bool) {
        self.0 &= !0b0001;
        if b {
            self.0 |= 0b0001;
        }
    }

    const fn get(&self) -> Option<u8> {
        if self.1 {
            Some(self.0)
        }else {
            None
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub struct Opecode(SVec<3, u8>);

impl Opecode {
    pub fn new() -> Self {
        Opecode(SVec::new())
    }

    pub fn set(&mut self, opecode: SVec<3, u8>) {
        self.0 = opecode;
    }

    pub fn get(&self) -> SVec<3, u8> {
        self.0
    }
}

impl Deref for Opecode {
    type Target = SVec<3, u8>;

    fn deref(&self) -> &SVec<3, u8> {
        &self.0
    }
}

impl DerefMut for Opecode {
    fn deref_mut(&mut self) -> &mut SVec<3, u8> {
        &mut self.0
    }
}

#[derive(Clone, Copy, Debug)]
pub struct ModRm(u8);

impl ModRm {
    pub fn new() -> ModRm {
        ModRm(0)
    }

    pub fn set_mod(&mut self, r#mod: u8) {
        self.0 &= !0b11000000;
        self.0 |= (r#mod & 0b11) << 6;
    }

    pub fn set_reg(&mut self, reg: u8) {
        self.0 &= !0b00111000;
        self.0 |= (reg & 0b111) << 3;
    }

    pub fn set_rm(&mut self, rm: u8) {
        self.0 &= !0b00000111;
        self.0 |= rm & 0b111;
    }

    pub fn get(&self) -> u8 {
        self.0
    }
}

#[derive(Clone, Copy, Debug)]
pub struct Sib(u8);

impl Sib {
    pub fn new() -> Self {
        Sib(0)
    }

    pub fn set_scale(&mut self, scale: u8) {
        self.0 &= !0b11000000;
        self.0 |= (scale & 0b11) << 6;
    }

    pub fn set_index(&mut self, index: u8) {
        self.0 &= !0b111000;
        self.0 |= (index & 0b111) << 3;
    }

    pub fn set_base(&mut self, base: u8) {
        self.0 &= !0b111;
        self.0 |= base & 0b111;
    }

    pub fn get(&self) -> u8 {
        self.0
    }
}

#[derive(Clone, Copy, Debug)]
pub enum Disp {
    None,
    Disp8(u8),
    Disp16(u16),
    Disp32(u32),
}

#[derive(Clone, Copy, Debug)]
pub enum Imm {
    None,
    Imm8(u8),
    Imm16(u16),
    Imm32(u32),
    Imm64(u64),
}

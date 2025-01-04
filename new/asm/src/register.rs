use std::str::FromStr;

/// Enum for Register
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Register {
    Rax,
    Rcx,
    Rdx,
    Rbx,
    Rsp,
    Rbp,
    Rsi,
    Rdi,
    R8,
    R9,
    R10,
    R11,
    R12,
    R13,
    R14,
    R15,
    Rip,

    Eax,
    Ecx,
    Edx,
    Ebx,
    Esp,
    Ebp,
    Esi,
    Edi,
    R8d,
    R9d,
    R10d,
    R11d,
    R12d,
    R13d,
    R14d,
    R15d,

    Ax,
    Cx,
    Dx,
    Bx,
    Sp,
    Bp,
    Si,
    Di,
    R8w,
    R9w,
    R10w,
    R11w,
    R12w,
    R13w,
    R14w,
    R15w,

    Al,
    Cl,
    Dl,
    Bl,
    Ah,
    Ch,
    Dh,
    Bh,
    Spl,
    Bpl,
    Sil,
    Dil,
    R8l,
    R9l,
    R10l,
    R11l,
    R12l,
    R13l,
    R14l,
    R15l,
}

impl Register {
    /// If this register is 64bit
    pub fn is_64bit(self) -> bool {
        const RAX_USIZE: usize = Register::Rax as usize;
        const RIP_USIZE: usize = Register::Rip as usize;

        let self_usize = self as usize;

        if RAX_USIZE <= self_usize && self_usize <= RIP_USIZE {
            true
        } else {
            false
        }
    }

    /// If this register is 32bit
    pub fn is_32bit(self) -> bool {
        const EAX_USIZE: usize = Register::Eax as usize;
        const R15D_USIZE: usize = Register::R15d as usize;

        let self_usize = self as usize;

        if EAX_USIZE <= self_usize && self_usize <= R15D_USIZE {
            true
        } else {
            false
        }
    }

    /// If this register is 16bit
    pub fn is_16bit(self) -> bool {
        const AX_USIZE: usize = Register::Ax as usize;
        const R15W_USIZE: usize = Register::R15w as usize;

        let self_usize = self as usize;

        if AX_USIZE <= self_usize && self_usize <= R15W_USIZE {
            true
        } else {
            false
        }
    }

    /// If this register is 8bit
    pub fn is_8bit(self) -> bool {
        const AL_USIZE: usize = Register::Al as usize;
        const R15L_USIZE: usize = Register::R15l as usize;

        let self_usize = self as usize;

        if AL_USIZE <= self_usize && self_usize <= R15L_USIZE {
            true
        } else {
            false
        }
    }

    /// Get 8bit register code
    /// Option<(rex.x, reg)
    pub const fn to_regcode8(self) -> Option<(Option<bool>, u8)> {
        match self {
            Self::Al => Some((Some(false), 0)),
            Self::Cl => Some((Some(false), 1)),
            Self::Dl => Some((Some(false), 2)),
            Self::Bl => Some((Some(false), 3)),
            Self::Ah => Some((None, 4)),
            Self::Ch => Some((None, 5)),
            Self::Dh => Some((None, 6)),
            Self::Bh => Some((None, 7)),
            Self::Spl => Some((Some(false), 4)),
            Self::Bpl => Some((Some(false), 5)),
            Self::Sil => Some((Some(false), 6)),
            Self::Dil => Some((Some(false), 7)),
            Self::R8l => Some((Some(true), 0)),
            Self::R9l => Some((Some(true), 1)),
            Self::R10l => Some((Some(true), 2)),
            Self::R11l => Some((Some(true), 3)),
            Self::R12l => Some((Some(true), 4)),
            Self::R13l => Some((Some(true), 5)),
            Self::R14l => Some((Some(true), 6)),
            Self::R15l => Some((Some(true), 7)),
            _ => None,
        }
    }

    pub const fn to_regcode16(self) -> Option<(Option<bool>, u8)> {
        match self {
            Self::Ax => Some((Some(false), 0)),
            Self::Cx => Some((Some(false), 1)),
            Self::Dx => Some((Some(false), 2)),
            Self::Bx => Some((Some(false), 3)),
            Self::Sp => Some((Some(false), 4)),
            Self::Bp => Some((Some(false), 5)),
            Self::Si => Some((Some(false), 6)),
            Self::Di => Some((Some(false), 7)),
            Self::R8w => Some((Some(true), 0)),
            Self::R9w => Some((Some(true), 1)),
            Self::R10w => Some((Some(true), 2)),
            Self::R11w => Some((Some(true), 3)),
            Self::R12w => Some((Some(true), 4)),
            Self::R13w => Some((Some(true), 5)),
            Self::R14w => Some((Some(true), 6)),
            Self::R15w => Some((Some(true), 7)),
            _ => None,
        }
    }

    pub const fn to_regcode32(self) -> Option<(Option<bool>, u8)> {
        match self {
            Self::Eax => Some((Some(false), 0)),
            Self::Ecx => Some((Some(false), 1)),
            Self::Edx => Some((Some(false), 2)),
            Self::Ebx => Some((Some(false), 3)),
            Self::Esp => Some((Some(false), 4)),
            Self::Ebp => Some((Some(false), 5)),
            Self::Esi => Some((Some(false), 6)),
            Self::Edi => Some((Some(false), 7)),
            Self::R8d => Some((Some(true), 0)),
            Self::R9d => Some((Some(true), 1)),
            Self::R10d => Some((Some(true), 2)),
            Self::R11d => Some((Some(true), 3)),
            Self::R12d => Some((Some(true), 4)),
            Self::R13d => Some((Some(true), 5)),
            Self::R14d => Some((Some(true), 6)),
            Self::R15d => Some((Some(true), 7)),
            _ => None,
        }
    }

    pub const fn to_regcode64(self) -> Option<(Option<bool>, u8)> {
        match self {
            Self::Rax => Some((Some(false), 0)),
            Self::Rcx => Some((Some(false), 1)),
            Self::Rdx => Some((Some(false), 2)),
            Self::Rbx => Some((Some(false), 3)),
            Self::Rsp => Some((Some(false), 4)),
            Self::Rbp => Some((Some(false), 5)),
            Self::Rsi => Some((Some(false), 6)),
            Self::Rdi => Some((Some(false), 7)),
            Self::R8 => Some((Some(true), 0)),
            Self::R9 => Some((Some(true), 1)),
            Self::R10 => Some((Some(true), 2)),
            Self::R11 => Some((Some(true), 3)),
            Self::R12 => Some((Some(true), 4)),
            Self::R13 => Some((Some(true), 5)),
            Self::R14 => Some((Some(true), 6)),
            Self::R15 => Some((Some(true), 7)),
            _ => None,
        }
    }

    pub fn to_regcode(self) -> (Option<bool>, u8) {
        self.to_regcode8()
            .or(self.to_regcode16())
            .or(self.to_regcode32())
            .or(self.to_regcode64())
            .expect("internal error")
    }
    /*
    /// Get 64bit register code
    pub fn to_regcode64(self) -> Result<u8, ()> {
        if self.is_64bit() {
            if self != Self::Rip {
                Ok((self as usize - Self::Rax as usize) as u8)
            } else {
                Err(())
            }
        } else {
            Err(())
        }
    }

    pub fn to_regcode(self) -> Result<u8, ()> {
        if self.is_64bit() {
            if self == Self::Rip {
                Err(())
            } else {
                Ok(self as usize as u8)
            }
        } else if self.is_32bit() {
            Ok((self as usize - Self::Eax as usize) as u8)
        } else if self.is_16bit() {
            Ok((self as usize - Self::Ax as usize) as u8)
        } else if self.is_8bit() {
            Ok((self as usize - Self::Al as usize) as u8)
        } else {
            Err(())
        }
    }*/
}

impl FromStr for Register {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s {
            "rax" => Self::Rax,
            "rcx" => Self::Rcx,
            "rdx" => Self::Rdx,
            "rbx" => Self::Rbp,
            "rsp" => Self::Rsp,
            "rbp" => Self::Rbp,
            "rsi" => Self::Rsi,
            "rdi" => Self::Rdi,
            "r8" => Self::R8,
            "r9" => Self::R9,
            "r10" => Self::R10,
            "r11" => Self::R11,
            "r12" => Self::R12,
            "r13" => Self::R13,
            "r14" => Self::R14,
            "r15" => Self::R15,
            "rip" => Self::Rip,
            "eax" => Self::Eax,
            "ecx" => Self::Ecx,
            "edx" => Self::Edx,
            "ebx" => Self::Ebx,
            "esp" => Self::Esp,
            "ebp" => Self::Ebp,
            "esi" => Self::Esi,
            "edi" => Self::Edi,
            "r8d" => Self::R8d,
            "r9d" => Self::R9d,
            "r10d" => Self::R10d,
            "r11d" => Self::R11d,
            "r12d" => Self::R12d,
            "r13d" => Self::R13d,
            "r14d" => Self::R14d,
            "r15d" => Self::R15d,
            "ax" => Self::Ax,
            "cx" => Self::Cx,
            "dx" => Self::Dx,
            "bx" => Self::Bx,
            "sp" => Self::Sp,
            "bp" => Self::Bp,
            "si" => Self::Si,
            "di" => Self::Di,
            "r8w" => Self::R8w,
            "r9w" => Self::R9w,
            "r10w" => Self::R10w,
            "r11w" => Self::R11w,
            "r12w" => Self::R12w,
            "r13w" => Self::R13w,
            "r14w" => Self::R14w,
            "r15w" => Self::R15w,
            "al" => Self::Al,
            "cl" => Self::Cl,
            "dl" => Self::Dl,
            "bl" => Self::Bl,
            "spl" => Self::Spl,
            "bpl" => Self::Bpl,
            "sil" => Self::Sil,
            "dil" => Self::Dil,
            "r8l" => Self::R8l,
            "r9l" => Self::R9l,
            "r10l" => Self::R10l,
            "r11l" => Self::R11l,
            "r12l" => Self::R12l,
            "r13l" => Self::R13l,
            "r14l" => Self::R14l,
            "r15l" => Self::R15l,
            _ => return Err(()),
        })
    }
}

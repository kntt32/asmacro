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
    }
}

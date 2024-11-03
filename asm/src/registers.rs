pub enum Register {
    Rax, Rcx, Rdx, Rbx, Rsp, Rbp, Rsi, Rdi, R8, R9, R10, R11, R12, R13, R14, R15,
    Rip,
    
    Eax, Ecx, Edx, Ebx, Esp, Ebp, Esi, Edi,

    Ax, Cx, Dx, Bx, Sp, Bp, Si, Di,
}

impl Register {
    pub const fn is_64bit(self) -> bool {
        const RAX_USIZE: usize = Register::Rax as usize;
        const RIP_USIZE: usize = Register::Rip as usize;

        let self_usize = self as usize;

        if RAX_USIZE <= self_usize && self_usize <= RIP_USIZE {
            true
        }else {
            false
        }
    }
}
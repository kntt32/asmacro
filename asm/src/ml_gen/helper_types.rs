pub struct RegCode {
    pub rex: bool,
    pub reg: u8, // 2:0
}

impl RegCode {
    pub const Rax: RegCode = RegCode {
        rex: false,
        reg: 0,
    };
    pub const Rcx: RegCode = RegCode {
        rex: false,
        reg: 1,
    };
    pub const Rdx: RegCode = RegCode {
        rex: false,
        reg: 2,
    };
    pub const Rbx: RegCode = RegCode {
        rex: false,
        reg: 3,
    };
    pub const Rsp: RegCode = RegCode {
        rex: false,
        reg: 4,
    };
    pub const Rbp: RegCode = RegCode {
        rex: false,
        reg: 5,
    };
    pub const Rsi: RegCode = RegCode {
        rex: false,
        reg: 6,
    };
    pub const Rdi: RegCode = RegCode {
        rex: false,
        reg: 7,
    };
    pub const R8: RegCode = RegCode {
        rex: true,
        reg: 0,
    };
    pub const R9: RegCode = RegCode {
        rex: true,
        reg: 1,
    };
    pub const R10: RegCode = RegCode {
        rex: true,
        reg: 2,
    };
    pub const R11: RegCode = RegCode {
        rex: true,
        reg: 3,
    };
    pub const R12: RegCode = RegCode {
        rex: true,
        reg: 4,
    };
    pub const R13: RegCode = RegCode {
        rex: true,
        reg: 5,
    };
    pub const R14: RegCode = RegCode {
        rex: true,
        reg: 6,
    };
    pub const R15: RegCode = RegCode {
        rex: true,
        reg: 7,
    };
}




pub type Elf64Addr = u64;
pub type Elf64Half = u16;
pub type Elf64Off = u64;
pub type Elf64Sword = i32;
pub type Elf64Word = u32;
pub type Elf64Xword = u64;
pub type Elf64Sxword = i64;

#[derive(Clone, Debug)]
pub struct Elf {
    elf_header: Elf64Ehdr,
    program_header: Elf64Phdr,
    program: Vec<u8>,
}

impl Elf {
    pub fn run(&self) -> isize {
        todo!()
    }
}

#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct Elf64Ehdr {
    pub e_ident: [u8; Self::EiNident],
    pub e_type: Elf64Half,
    pub e_machine: Elf64Half,
    pub e_version: Elf64Word,
    pub e_entry: Elf64Addr,
    pub e_phoff: Elf64Off,
    pub e_shoff: Elf64Off,
    pub e_flags: Elf64Word,
    pub e_ehsize: Elf64Half,
    pub e_phentsize: Elf64Half,
    pub e_phnum: Elf64Half,
    pub e_shentsize: Elf64Half,
    pub e_shnum: Elf64Half,
    pub e_shstrndx: Elf64Half,
}

impl Elf64Ehdr {
    pub const EiNident: usize = 16;
    pub const EiIdent: [u8; Self::EiNident] = [
        0x7f, 0x45, 0x4c, 0x46, 0x02, 0x01, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x00,
    ];

    pub const EtRel: Elf64Half = 1;
    pub const EtExec: Elf64Half = 2;
    pub const EtDyn: Elf64Half = 3;

    pub const EmAmd64: Elf64Half = 0x3e;

    pub const EfNone: Elf64Word = 0x00;

    pub const EshnUndef: Elf64Half = 0x00;
}

#[derive(Clone, Copy, Debug)]
pub struct Elf64Phdr {
    pub p_type: Elf64Word,
    pub p_flags: Elf64Word,
    pub p_offset: Elf64Off,
    pub p_vaddr: Elf64Addr,
    pub p_paddr: Elf64Addr,
    pub p_filesz: Elf64Xword,
    pub p_memsz: Elf64Xword,
    pub p_align: Elf64Xword,
}

impl Elf64Phdr {
    pub const PtNull: Elf64Word = 0;
    pub const PtLoad: Elf64Word = 1;
    pub const PtPhdr: Elf64Word = 6;

    pub const PfX: Elf64Word = 0x1;
    pub const PfW: Elf64Word = 0x2;
    pub const PfR: Elf64Word = 0x4;
}

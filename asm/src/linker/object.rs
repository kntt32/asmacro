use crate::{
    assembler::line::label::{Label, Location},
    functions::SResult,
};

pub struct Elf(Vec<u8>);

#[derive(Clone, Debug)]
pub struct Object {
    pub code: Vec<u8>,
    pub label: Vec<Label>,
    pub location: Vec<Location>,
}

impl Object {
    pub fn new() -> Self {
        Object {
            code: Vec::new(),
            label: Vec::new(),
            location: Vec::new(),
        }
    }

    pub fn code_len(&self) -> usize {
        self.code.len()
    }

    fn locate(&mut self) -> SResult<()> {
        let mut status: SResult<()> = Ok(());

        self.location.retain(|l| {
            if let Some(label) = l.get_label(&self.label) {
                let target: isize = l.rel_base as isize - label.value() as isize;
                for i in 0..l.size {
                    self.code[l.offset + i] = ((target >> (i * 8)) & 0xff) as u8;
                }
                if !label.is_public() {
                    false
                } else {
                    true
                }
            } else {
                status = Err(format!("unknown label \"{}\"", &l.label));
                true
            }
        });

        status
    }

    fn add_base(&mut self, base: usize) {
        for l in &mut self.label {
            l.add_base(base);
        }
        for l in &mut self.location {
            l.add_base(base);
        }
    }

    fn append(&mut self, mut other: Self) {
        let link_base: usize = (self.code.len() + 0x0f) & (!0xf);

        self.code.resize(link_base, 0x00);
        self.add_base(link_base);

        self.code.append(&mut other.code);
        self.label.append(&mut other.label);
        self.location.append(&mut other.location);
    }

    pub fn link(&mut self, mut other: Self) -> SResult<()> {
        self.locate()?;
        other.locate()?;
        self.append(other);
        self.locate()?;
        Ok(())
    }

    pub fn elf(self) -> SResult<Elf> {
        todo!();
    }
}

mod elf {
    pub type Elf64Addr = u64;
    pub type Elf64Half = u16;
    pub type Elf64Off = u64;
    pub type Elf64Sword = i32;
    pub type Elf64Word = u32;
    pub type Elf64Xword = u64;
    pub type Elf64Sxword = i64;

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
            0x7f, 0x45, 0x4c, 0x46, 0x02, 0x01, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00,
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
}

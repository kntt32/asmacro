use crate::{
    assembler::line::label::{Label, Location},
    functions::SResult,
};
use elf::{Elf64Ehdr, Elf64Phdr};
use std::{
    mem::{size_of, transmute},
    slice,
};

pub struct Elf(Vec<u8>);

impl Elf {
    pub fn as_vec(self) -> Vec<u8> {
        self.0
    }
}

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

    fn get_label<'a>(&'a self, name: &str) -> Option<&'a Label> {
        for i in &self.label {
            if i.name() == name {
                return Some(i);
            }
        }
        None
    }

    fn locate(&mut self) -> SResult<()> {
        let mut status: SResult<()> = Ok(());

        self.location.retain(|l| {
            if let Some(label) = l.get_label(&self.label) {
                let target = label.value() as isize - l.rel_base as isize;
                let target_usize = unsafe { transmute::<isize, usize>(target) };
                for i in 0..l.size {
                    self.code[l.offset + i] = ((target_usize >> (i * 8)) & 0xff) as u8;
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

    pub fn elf(mut self, entry_point: &str) -> SResult<Elf> {
        self.locate()?;

        let entry_point = if let Some(l) = self.get_label(entry_point) {
            l.value()
        } else {
            return Err(format!("entry point \"{}\" not found", entry_point));
        };

        let phdr_offset = size_of::<Elf64Ehdr>() as u64;
        let text_offset = phdr_offset + (size_of::<Elf64Phdr>() as u64) * 2;
        let load_base = 0x400000u64;

        let elf64ehdr = Elf64Ehdr {
            e_ident: Elf64Ehdr::EI_IDENT,
            e_type: Elf64Ehdr::ET_EXEC,
            e_machine: Elf64Ehdr::EM_AMD64,
            e_version: 0x01,
            e_entry: load_base + entry_point as u64 + text_offset,
            e_phoff: phdr_offset,
            e_shoff: 0x00,
            e_flags: Elf64Ehdr::EF_NONE,
            e_ehsize: size_of::<Elf64Ehdr>() as u16,
            e_phentsize: size_of::<Elf64Phdr>() as u16,
            e_phnum: 2,
            e_shentsize: 0x00,
            e_shnum: 0x00,
            e_shstrndx: Elf64Ehdr::ESHN_UNDEF,
        };

        let elf64phdr_phdr = Elf64Phdr {
            p_type: Elf64Phdr::PT_PHDR,
            p_flags: Elf64Phdr::PF_R,
            p_offset: phdr_offset,
            p_vaddr: load_base + phdr_offset,
            p_paddr: load_base + phdr_offset,
            p_filesz: size_of::<Elf64Phdr>() as u64 * 2,
            p_memsz: size_of::<Elf64Phdr>() as u64 * 2,
            p_align: 0x80,
        };

        let elf64phdr_load = Elf64Phdr {
            p_type: Elf64Phdr::PT_LOAD,
            p_flags: Elf64Phdr::PF_R | Elf64Phdr::PF_X,
            p_offset: 0,
            p_vaddr: load_base,
            p_paddr: load_base,
            p_filesz: (size_of::<Elf64Ehdr>() + size_of::<Elf64Phdr>() * 2 + self.code.len())
                as u64,
            p_memsz: (size_of::<Elf64Ehdr>() + size_of::<Elf64Phdr>() * 2 + self.code.len()) as u64,
            p_align: 0x200000,
        };

        let elf64ehdr_u8_ptr =
            unsafe { transmute::<*const Elf64Ehdr, *const u8>(&elf64ehdr as *const Elf64Ehdr) };
        let elf64phdr_phdr_u8_ptr = unsafe {
            transmute::<*const Elf64Phdr, *const u8>(&elf64phdr_phdr as *const Elf64Phdr)
        };
        let elf64phdr_load_u8_ptr = unsafe {
            transmute::<*const Elf64Phdr, *const u8>(&elf64phdr_load as *const Elf64Phdr)
        };

        let elf64ehdr_slice =
            unsafe { slice::from_raw_parts(elf64ehdr_u8_ptr, size_of::<Elf64Ehdr>()) };
        let elf64phdr_phdr_slice =
            unsafe { slice::from_raw_parts(elf64phdr_phdr_u8_ptr, size_of::<Elf64Phdr>()) };
        let elf64phdr_load_slice =
            unsafe { slice::from_raw_parts(elf64phdr_load_u8_ptr, size_of::<Elf64Phdr>()) };

        let mut binary = Vec::new();

        binary.extend_from_slice(elf64ehdr_slice);
        binary.extend_from_slice(elf64phdr_phdr_slice);
        binary.extend_from_slice(elf64phdr_load_slice);
        binary.extend_from_slice(&self.code);

        Ok(Elf(binary))
    }
}

#[allow(dead_code)]
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
        pub e_ident: [u8; Self::EI_NINDENT],
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

    #[allow(dead_code)]
    impl Elf64Ehdr {
        pub const EI_NINDENT: usize = 16;
        pub const EI_IDENT: [u8; Self::EI_NINDENT] = [
            0x7f, 0x45, 0x4c, 0x46, 0x02, 0x01, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00,
        ];

        pub const ET_REL: Elf64Half = 1;
        pub const ET_EXEC: Elf64Half = 2;
        pub const ET_DYN: Elf64Half = 3;

        pub const EM_AMD64: Elf64Half = 0x3e;

        pub const EF_NONE: Elf64Word = 0x00;

        pub const ESHN_UNDEF: Elf64Half = 0x00;
    }

    #[derive(Clone, Copy, Debug)]
    #[repr(C)]
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

    #[allow(dead_code)]
    impl Elf64Phdr {
        pub const PT_NULL: Elf64Word = 0;
        pub const PT_LOAD: Elf64Word = 1;
        pub const PT_PHDR: Elf64Word = 6;

        pub const PF_X: Elf64Word = 0x1;
        pub const PF_W: Elf64Word = 0x2;
        pub const PF_R: Elf64Word = 0x4;
    }
}

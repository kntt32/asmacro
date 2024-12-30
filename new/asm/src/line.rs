use crate::instruction::{Instruction, INSTRUCTION_LIST};
use util::svec::SVec;

/// Assembly line information
#[derive(Clone, Copy, Debug)]
pub enum Line<'a> {
    None,
    Label(&'a str),
    AsmCommand(&'a str),
    Instruction(&'a str),
    UnKnown(&'a str),
}

impl<'a> Line<'a> {
    /// Split instruction and return mnemonic and operands
    /// (mnemonic, operand1, operand2)
    pub fn split_instruction(self) -> Option<(&'a str, [Option<&'a str>; 2])> {
        if let Line::Instruction(s) = self {
            let mut s_split = s.trim().split(' ');

            let mnemonic = s_split.next().expect("unknown error");
            let operand1 = s_split.next();
            let operand2 = s_split.next();

            if s_split.next().is_none() {
                Some((mnemonic, [operand1, operand2]))
            } else {
                None
            }
        } else {
            None
        }
    }

    /// Get mneonic
    pub fn mnemonic(self) -> Option<&'a str> {
        Some(self.split_instruction()?.0)
    }

    /// Get operands
    pub fn operands(self) -> Option<[Option<&'a str>; 2]> {
        Some(self.split_instruction()?.1)
    }

    pub fn get_instruction(self) -> Option<Instruction> {
        for i in INSTRUCTION_LIST {
            if i.match_with(&self) {
                return Some(*i);
            }
        }
        None
    }
}

// Encode
impl<'a> Line<'a> {
    /// Get opecode in raw machine code
    pub fn opecode(self) -> Option<SVec<3, u8>> {
        let instruction = self.get_instruction()?;
        let mut opecode = instruction.opecode();
        todo!();
        //opecode[opecode.len() - 1] += ;
    }

    fn opecode_len(self) -> Option<usize> {
        todo!()
    }
}


use crate::instruction::{Instruction, OperandType, INSTRUCTION_LIST};
use pseudo::Pseudo;

/*
/// Methods related to machine code encoding
pub mod encode;
*/
/// Assembly line information
#[derive(Clone, Copy, Debug)]
pub enum Line<'a> {
    /// 空の行
    None,
    /// ラベル
    Label(&'a str),
    /// 疑似命令
    Pseudo(&'a str),
    /// CPU命令
    Instruction(&'a str),
    /// 不明な表現
    Unknown(&'a str),
}

impl<'a> Line<'a> {
    pub fn len(self, pseudo_commands: &[Pseudo]) -> Result<usize, String> {
        match self {
            Line::None => Ok(0),
            Line::Label(_) => Ok(0),
            Line::Pseudo(_) => self.pseudo_len(&pseudo_commands),
            Line::Instruction(_) => todo!(),
            Line::Unknown(s) => Err(format!("unknown expression \"{}\"", s)),
        }
    }
}

/// 疑似命令関連のモジュール
/// # サポートする疑似命令
/// - .db8  : 8bit数値列書き込み
/// - .db16 : 16bit数値列書き込み
/// - .db32 : 32bit数値列書き込み
/// - .db64 : 64bit数値列書き込み
/// - .utf8 : utf8文字列書き込み
/// - .align16 : 16バイトでアライメント
mod pseudo {
    use super::Line;

    pub struct Pseudo {
        name: String,
        bin: Box<dyn Fn(&str) -> Vec<u8>>,
        len: Box<dyn Fn(&str) -> Result<usize, String>>,
    }
    impl<'a> Line<'a> {
        pub fn pseudo_bin(self, pseudo_commands: &[Pseudo]) -> Vec<u8> {
            if let Line::Pseudo(s) = self {
                let name = pseudo_name(s);
                let arg = pseudo_arg(s);
                if let Some(p) = get_pseudo(name, pseudo_commands) {
                    (p.bin)(arg)
                } else {
                    panic!("internal error: undefined pseudo");
                }
            } else {
                panic!("internal error: input must be Line::Pseudo");
            }
        }

        pub fn pseudo_len(self, pseudo_commands: &[Pseudo]) -> Result<usize, String> {
            if let Line::Pseudo(s) = self {
                let name = pseudo_name(s);
                let arg = pseudo_arg(s);
                if let Some(p) = get_pseudo(name, pseudo_commands) {
                    (p.len)(arg)
                } else {
                    Err(format!("unknown pseudo instruction : \"{}\"", s))
                }
            } else {
                panic!("internal error: input must be Line::Pseudo");
            }
        }
    }

    fn get_pseudo<'a>(name: &str, pseudo_commands: &'a [Pseudo]) -> Option<&'a Pseudo> {
        pseudo_commands.iter().find(|&x| name == x.name)
    }

    fn pseudo_name(mut line: &str) -> &str {
        const error_message: &str = "internal error: input must be pseudo instruction";
        line = line.trim();
        if !line.starts_with('.') {
            panic!("{}", error_message);
        }
        line = &line['.'.len_utf8()..line.len()];
        line.split(' ').next().expect(error_message)
    }

    fn pseudo_arg(line: &str) -> &str {
        if let Some((_, right)) = line.trim().split_once(' ') {
            right
        } else {
            &""
        }
    }
}

/// Label関連のモジュール
mod label {
    pub struct Label {
        name: String,
        offset: usize,
    }
}

/// Instruction関連のモジュール
pub mod instruction;
impl<'a> Line<'a> {
    /// Split instruction and return mnemonic and operands
    /// (mnemonic, operand1, operand2)
    pub fn split_instruction(self) -> Option<(&'a str, Vec<&'a str>)> {
        if let Line::Instruction(s) = self {
            let mut s_split = s.trim().split(' ');

            let mnemonic = s_split.next().expect("unknown error");
            let operands = s_split.collect();
            Some((mnemonic, operands))
        } else {
            None
        }
    }

    // Is valid instruction
    pub fn is_valid_instruction(self) -> bool {
        match self {
            Line::Instruction(_) => self.get_instruction().is_some(),
            _ => false,
        }
    }

    /// Get mneonic
    pub fn mnemonic(self) -> Option<&'a str> {
        if let Line::Instruction(s) = self {
            let mut s_split = s.trim().split(' ');
            let mnemonic = s_split.next().expect("unknown error");
            Some(mnemonic)
        } else {
            None
        }
    }

    /// Get operands
    pub fn operands(self) -> Option<Vec<&'a str>> {
        Some(self.split_instruction()?.1)
    }

    /// Get instruction information
    pub fn get_instruction(self) -> Option<&'static Instruction> {
        for i in INSTRUCTION_LIST {
            if i.match_with(&self) {
                return Some(&i);
            }
        }
        None
    }

    fn get_operand_by_type(self, operand_type: OperandType) -> Option<&'a str> {
        let instruction = self.get_instruction()?;
        let operand_index = instruction
            .expression()
            .get_operand_index_by_type(operand_type)?;
        Some(self.operands()?[operand_index])
    }
}

use crate::assembler::register::Register;
use std::{mem::transmute, str::FromStr};
use util::functions::{result_to_option, stoi};

#[derive(Clone, Debug, PartialEq)]
pub enum Disp {
    Value(i32),
    Label(String),
}

#[derive(Clone, Debug, PartialEq)]
pub enum Imm {
    Value(u64),
    Label(String),
}

impl FromStr for Imm {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if let Some(v) = stoi(s) {
            if i64::MIN as i128 <= v && v <= u64::MAX as i128 {
                let value = (unsafe { transmute::<i128, u128>(v) } & 0xffff_ffff_ffff_ffff) as u64;
                Ok(Imm::Value(value))
            } else {
                Err("imm value is too large".to_string())
            }
        } else {
            Ok(Imm::Label(s.to_string()))
        }
    }
}

pub fn parse_rm_anysize(expr: &str) -> Option<(Disp, Register, Option<(Register, u8)>)> {
    for c in &['b', 'w', 'd', 'q'] {
        if let Some(v) = parse_rm(expr, *c) {
            return Some(v);
        }
    }
    None
}

pub fn parse_rm(
    mut expr: &str,
    address_size: char,
) -> Option<(Disp, Register, Option<(Register, u8)>)> {
    // disp[base, index, scale]
    let disp: Disp = if !expr.starts_with('[') {
        let disp_expr = expr.split_once('[')?.0;
        if let Some(value) = stoi(disp_expr) {
            if i32::MIN as i128 <= value && value <= i32::MAX as i128 {
                Disp::Value(value as i32)
            } else {
                return None;
            }
        } else {
            if is_keyword(disp_expr) {
                Disp::Label(disp_expr.to_string())
            } else {
                return None;
            }
        }
    } else {
        Disp::Value(0)
    };

    expr = expr.split_once('[')?.1.trim();
    if !expr.ends_with(address_size) {
        return None;
    }
    expr = &expr[..expr.len() - address_size.len_utf8()];
    if !expr.ends_with(']') {
        return None;
    }
    expr = &expr[..expr.len() - ']'.len_utf8()];
    let mut arguments_iter = expr.split(',');

    let base = result_to_option(arguments_iter.next()?.parse::<Register>())?;

    let index = if let Some(s) = arguments_iter.next() {
        result_to_option(s.parse::<Register>())?
    } else {
        return Some((disp, base, None));
    };

    let scale = if let Some(s) = arguments_iter.next() {
        let value = stoi(s)?;
        if value == 1 || value == 2 || value == 4 || value == 8 {
            value as u8
        } else {
            return None;
        }
    } else {
        return Some((disp, base, Some((index, 1))));
    };

    Some((disp, base, Some((index, scale))))
}

/// If this is a label
pub fn is_label(mut line: &str) -> bool {
    line = line.trim();
    if !line.ends_with(':') {
        return false;
    }
    line = &line[..line.len() - ':'.len_utf8()].trim();
    is_keyword(line)
}

pub fn is_keyword(mut word: &str) -> bool {
    word = word.trim();
    let mut word_chars = word.chars();

    let Some(first_char) = word_chars.next() else {
        return false;
    };
    if !first_char.is_ascii_alphabetic() {
        return false;
    }

    for c in word_chars {
        if !c.is_ascii_alphanumeric() {
            return false;
        }
    }

    true
}

/// If this is a assembler command
pub fn is_pseudo(mut line: &str) -> bool {
    line = line.trim();
    line.starts_with('.')
}

/// If this is a instruction
pub fn is_instruction(line: &str) -> bool {
    let mut line_split = line.split(' ');

    let Some(mnemonic) = line_split.next() else {
        return false;
    };
    is_keyword(mnemonic)
}

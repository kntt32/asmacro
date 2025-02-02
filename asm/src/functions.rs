use crate::register::Register;
use util::functions::{result_to_option, stoi};

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Disp<'a> {
    Value(i32),
    Label(&'a str),
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Imm<'a> {
    Value(i32),
    Label(&'a str),
}
/*
impl<'a> FromStr for Imm<'a> {
    let
}
*/
pub fn parse_rm_anysize(expr: &str) -> Option<(Disp<'_>, Register, Option<(Register, u8)>)> {
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
) -> Option<(Disp<'_>, Register, Option<(Register, u8)>)> {
    // disp[base, index, scale]
    let disp: Disp<'_> = if !expr.starts_with('[') {
        let disp_expr = expr.split_once('[')?.0;
        if let Some(value) = stoi(disp_expr) {
            if i32::MIN as i128 <= value && value <= i32::MAX as i128 {
                Disp::Value(value as i32)
            } else {
                return None;
            }
        } else {
            if is_keyword(disp_expr) {
                Disp::Label(disp_expr)
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
pub fn is_asm_command(mut line: &str) -> bool {
    line = line.trim();
    if !line.starts_with('.') {
        return false;
    }
    line = &line[1..].trim();
    is_keyword(line)
}

/// If this is a instruction
pub fn is_instruction(line: &str) -> bool {
    let mut line_split = line.split(' ');

    let Some(mnemonic) = line_split.next() else {
        return false;
    };
    is_keyword(mnemonic)
}

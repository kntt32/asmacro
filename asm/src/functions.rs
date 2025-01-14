use crate::register::Register;
use util::functions::{result_to_option, stoi};

pub enum Relocation<'a, T> {
    Value(T),
    Label(&'a str),
}

pub fn parse_rm(
    mut expr: &str,
    address_size: char,
) -> Option<(i32, Register, Option<(Register, u8)>)> {
    // disp[base, index, scale]
    let disp: i32 = if !expr.starts_with('[') {
        let value = stoi(expr.split_once('[')?.0)?;
        if i32::MIN as i128 <= value && value <= i32::MAX as i128 {
            value as i32
        } else {
            return None;
        }
    } else {
        0
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

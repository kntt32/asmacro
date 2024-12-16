use super::*;

pub fn get_reg64_str(expr: &str) -> Option<&str> {
    let trimed_expr = expr.trim();
    for s in [
        "rax", "rcx", "rdx", "rbx", "rsp", "rbp", "rdi", "rsi", "r8", "r9", "r10", "r11", "r12",
        "r13", "r14", "r15", "rip",
    ] {
        if trimed_expr == s {
            return Some(trimed_expr);
        }
    }
    None
}

pub fn get_rm64_ref_str(expr: &str) -> Option<(isize, Register, Option<(Register, u8)>)> {
    // disp[base], disp[base, index] or disp[base, index, scale]
    // Option<(disp, base, Option<(index, scale)>)>

    let mut operand = expr;

    // get disp
    let (left, right) = operand.split_once('[')?;
    let disp: isize = if left.trim().is_empty() {
        1
    } else {
        stoi(left.trim())?
    };
    operand = right.trim();

    // get base
    let base = if let Some((left, right)) = operand.split_once(',') {
        operand = right.trim();
        if let Ok(base_temp) = left.trim().parse() {
            base_temp
        } else {
            return None;
        }
    } else {
        return if operand.ends_with(']') {
            if let Ok(base_temp) = operand[..operand.len() - ']'.len_utf8()].trim().parse() {
                Some((disp, base_temp, None))
            } else {
                None
            }
        } else {
            None
        };
    };

    // get index
    let index = if let Some((left, right)) = operand.split_once(',') {
        operand = right.trim();
        if let Ok(index_temp) = left.trim().parse() {
            index_temp
        } else {
            return None;
        }
    } else {
        return if operand.ends_with(']') {
            if let Ok(index_temp) = operand[..operand.len() - ']'.len_utf8()].trim().parse() {
                Some((disp, base, Some((index_temp, 1))))
            } else {
                None
            }
        } else {
            None
        };
    };

    // get scale
    if operand.ends_with(']') {
        if let Ok(scale) = operand[..operand.len() - ']'.len_utf8()].trim().parse() {
            Some((disp, base, Some((index, scale))))
        } else {
            None
        }
    } else {
        None
    }
}

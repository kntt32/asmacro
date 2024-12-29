use super::*;

pub fn get_reg64(mut expr: &str) -> Option<Register> {
    let expr = expr.trim();
    if let Ok(r) = expr.parse::<Register>() {
        if r.is_64bit() {
            Some(r)
        }else {
            None
        }
    }else {
        None
    }
}

pub fn get_rm64_ref(expr: &str) -> Option<(isize, Register, Register, u8>)> {
    // disp[base], disp[base, index] or disp[base, index, scale]
    // Option<(disp, base, index, scale)>

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
                Some((disp, base_temp, Register::Rax, 0))
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
                Some((disp, base, index_temp, 1))
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
            Some((disp, base, index, scale))
        } else {
            None
        }
    } else {
        None
    }
}

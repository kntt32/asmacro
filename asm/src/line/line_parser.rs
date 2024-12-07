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

// [base, scale, index, disp]
pub fn get_rm64_ref_str(
    expr: &str,
) -> Option<(Option<&str>, Option<(isize, &str)>, Option<isize>)> {
    let inner_expr = get_inner_expr(expr, ['[', ']'])?;

    let mut iter = inner_expr.split(',').map(|s| s.trim());

    let mut result = (None, None, None);

    // get base
    if let Some(s) = iter.next() {
        result.0 = Some(s);
    } else {
        return None;
    }

    // get index
    if let Some(s) = iter.next() {
        result.1 = Some((1, s));
    }

    // get scale
    if let Some(s) = iter.next() {
        result.1?.0 = stoi(s)?;
    }

    // get disp
    if let Some(s) = iter.next() {
        result.2 = Some(stoi(s)?);
    }

    Some(result)
}

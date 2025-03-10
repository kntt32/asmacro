/// Result<T, String>
pub type SResult<T> = Result<T, String>;

/// Offset in source code
#[derive(Clone, Copy, PartialEq, Debug)]
pub struct Offset {
    pub column: usize,
    pub row: usize,
}

/// Convert str to Integer
/// # Example
/// ```
/// use util::functions::stoi;
/// assert_eq!(1328, stoi("1328").unwrap());
/// assert_eq!(0xa639f3e, stoi("0xa639f3e").unwrap());
/// assert_eq!(0b101101110101010, stoi("0b101101110101010").unwrap());
/// assert_eq!(0o116672, stoi("0o116672").unwrap());
/// ```
pub fn stoi(s: &str) -> Option<i128> {
    const STOI_FUNCTIONS: [fn(&str) -> Option<i128>; 5] =
        [stoi_minus, stoi_octal, stoi_decimal, stoi_hex, stoi_binary];

    for f in STOI_FUNCTIONS {
        if let Some(n) = f(s) {
            return Some(n);
        }
    }

    None
}

fn remove_prefix<'a>(s: &'a str, prefix: &str) -> Option<&'a str> {
    if let Some(v) = s.split_at_checked(prefix.len()) {
        if v.0 == prefix {
            Some(v.1)
        } else {
            None
        }
    } else {
        None
    }
}

fn stoi_helper(s: &str, n: &[char]) -> Option<i128> {
    let mut num: i128 = 0;

    for c in s.chars().map(|c| c.to_ascii_lowercase()) {
        let mut match_flag = false;
        if let Some(muln) = num.checked_mul(n.len() as i128) {
            num = muln;
        } else {
            return None;
        }

        for i in 0..n.len() {
            if c == n[i] {
                num += i as i128;
                match_flag = true;
                break;
            }
        }
        if !match_flag {
            return None;
        }
    }

    Some(num)
}

fn stoi_minus(s: &str) -> Option<i128> {
    stoi(remove_prefix(s, "-")?.trim()).map(|v| -v)
}

/// Binary to Integer
pub fn stoi_binary(s: &str) -> Option<i128> {
    stoi_helper(remove_prefix(s, "0b")?, &['0', '1'])
}

/// Octal to Integer
pub fn stoi_octal(s: &str) -> Option<i128> {
    stoi_helper(
        remove_prefix(s, "0o")?,
        &['0', '1', '2', '3', '4', '5', '6', '7'],
    )
}

/// Decimal to Integer
pub fn stoi_decimal(s: &str) -> Option<i128> {
    stoi_helper(s, &['0', '1', '2', '3', '4', '5', '6', '7', '8', '9'])
}

/// Hex to Integer
pub fn stoi_hex(s: &str) -> Option<i128> {
    stoi_hex_no_prefix(remove_prefix(s, "0x")?)
}

/// Hex with no prefix to Integer
pub fn stoi_hex_no_prefix(s: &str) -> Option<i128> {
    stoi_helper(
        s,
        &[
            '0', '1', '2', '3', '4', '5', '6', '7', '8', '9', 'a', 'b', 'c', 'd', 'e', 'f',
        ],
    )
}

/// Get string inner bracket.  Arguments bracket_start and bracket_end must be defferent each other.
pub fn get_inner_bracket(mut expr: &str, bracket_start: char, bracket_end: char) -> Option<&str> {
    if bracket_start == bracket_end {
        panic!("invalid input");
    }
    expr = expr.trim();
    if expr.starts_with(bracket_start) && expr.ends_with(bracket_end) {
        Some(&expr[bracket_start.len_utf8()..expr.len() - bracket_end.len_utf8()])
    } else {
        None
    }
}

/// Convert Result<T, E> to Option<T>
pub fn result_to_option<T, E>(result: Result<T, E>) -> Option<T> {
    match result {
        Ok(t) => Some(t),
        Err(_) => None,
    }
}

/// Get expression in brackets
/// # Example
/// ```
/// use util::functions::get_inner_expr;
///
/// assert_eq!("Hello", get_inner_expr(" [Hello ]", ['[', ']']).unwrap());
/// ```
pub fn get_inner_expr(expr: &str, bracket: [char; 2]) -> Option<&str> {
    let split_expr = expr.trim().split_at_checked(bracket[0].len_utf8())?;
    if split_expr.0.chars().next()? != bracket[0] {
        return None;
    }

    let start_removed_expr = split_expr.1;
    if start_removed_expr.len() <= bracket[1].len_utf8() {
        return None;
    }
    let split_expr = start_removed_expr
        .trim()
        .split_at_checked(start_removed_expr.len() - bracket[1].len_utf8())?;
    if split_expr.1.chars().next()? != bracket[1] {
        None
    } else {
        Some(split_expr.0.trim())
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    pub fn stoi_test() {
        assert_eq!(123, stoi("123").unwrap());
        assert_eq!(0xfe, stoi("0xfe").unwrap());
        assert_eq!(0b101, stoi("0b101").unwrap());
        assert_eq!(0o132, stoi("0o132").unwrap());
    }

    #[test]
    pub fn match_str_test() {
        fn is_reg64(s: &str) -> bool {
            s == "rbp" || s == "rdi"
        }
        assert_eq!(
            Some(vec!["[", "rbp", "+", "rdi", "*", "2", "]"]),
            match_str(
                &"[ rbp + rdi * 2 ]",
                &[
                    MatchStr::Char('['),
                    MatchStr::Str(&"rbp"),
                    MatchStr::Char('+'),
                    MatchStr::Str(&"rdi"),
                    MatchStr::Char('*'),
                    MatchStr::Number,
                    MatchStr::Char(']'),
                ],
            )
        );
    }

    #[test]
    pub fn get_inner_expr_test() {
        assert_eq!("Hello", get_inner_expr(" [Hello ]", ['[', ']']).unwrap());
        assert_eq!(
            "Inner Expr",
            get_inner_expr("[Inner Expr)", ['[', ')']).unwrap()
        );
    }
}

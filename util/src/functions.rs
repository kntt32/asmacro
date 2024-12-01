/// Convert str to Integer
/// # Example
/// ```
/// assert_eq!(1328, stoi("1328").unwrap());
/// assert_eq!(0xa639f3e, stoi("0xa639f3e").unwrap());
/// assert_eq!(0b101101110101010, stoi("0b101101110101010").unwrap());
/// assert_eq!(0o116672, stoi("0o116672").unwrap());
/// ```
pub fn stoi(s: &str) -> Option<usize> {
    const stoi_func: [fn(&str) -> Option<usize>; 5] =
        [stoi_minus, stoi_octal, stoi_decimal, stoi_hex, stoi_binary];

    for f in stoi_func {
        if let Some(n) = f(s) {
            return Some(n);
        }
    }

    None
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

fn stoi_helper(s: &str, n: &[char]) -> Option<usize> {
    let mut num: usize = 0;

    for c in s.chars().map(|c| c.to_ascii_lowercase()) {
        let mut match_flag = false;
        if let Some(muln) = num.checked_mul(n.len()) {
            num = muln;
        } else {
            return None;
        }

        for i in 0..n.len() {
            if c == n[i] {
                num += i;
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

fn stoi_minus(s: &str) -> Option<usize> {
    stoi(remove_prefix(s, "-")?.trim())
}

/// Binary to Integer
pub fn stoi_binary(s: &str) -> Option<usize> {
    stoi_helper(remove_prefix(s, "0b")?, &['0', '1'])
}

/// Octal to Integer
pub fn stoi_octal(s: &str) -> Option<usize> {
    stoi_helper(
        remove_prefix(s, "0o")?,
        &['0', '1', '2', '3', '4', '5', '6', '7'],
    )
}

/// Decimal to Integer
pub fn stoi_decimal(s: &str) -> Option<usize> {
    stoi_helper(s, &['0', '1', '2', '3', '4', '5', '6', '7', '8', '9'])
}

/// Hex to Integer
pub fn stoi_hex(s: &str) -> Option<usize> {
    stoi_helper(
        remove_prefix(s, "0x")?,
        &[
            '0', '1', '2', '3', '4', '5', '6', '7', '8', '9', 'a', 'b', 'c', 'd', 'e', 'f',
        ],
    )
}

/*
/// Matching string
pub fn match_str(mut s: &str, rule: &[StrMatch]) -> Option<[&str]> {
    for m in rule {
        match m {
            WhiteSpace => s = s.trim_start(),
            Number => {
                if !stoi(s).is_some() {  }
            }
        }
    }
}*/

enum MatchStr<'a> {
    WhiteSpace,
    Number,
    Char(char),
    Str(&'a str),
    Custom(fn(&str) -> bool),
}

use std::cmp::{Ord, Ordering, PartialOrd};

/// Result<T, String>
pub type SResult<T> = Result<T, String>;

/// Result<(), ErrorMessage>
pub type EResult = Result<(), ErrorMessage>;

/// ソースコード中の位置を表現するデータ型
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct Offset {
    pub row: usize,
    pub column: usize,
}

impl Offset {
    pub fn seek(&mut self, s: &str) {
        for c in s.chars() {
            if c == '\n' {
                self.row += 1;
                self.column = 0;
            } else {
                self.column += 1;
            }
        }
    }
}

impl PartialOrd for Offset {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(match self.row.cmp(&other.row) {
            Ordering::Equal => self.column.cmp(&other.column),
            o => o,
        })
    }
}

impl Ord for Offset {
    fn cmp(&self, other: &Self) -> Ordering {
        self.partial_cmp(other).expect("internal error")
    }
}

/// エラーメッセージを表現するデータ型
#[derive(Clone, PartialEq, Debug)]
pub struct ErrorMessage {
    pub msg: String,
    pub offset: Offset,
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

/// パーサーに役立つ関数をまとめたモジュール
pub mod parser {
    use super::Offset;

    /// 識別子
    pub fn parse_identifier(mut src: &str, mut offset: Offset) -> Option<(&str, &str, Offset)> {
        (src, offset) = skip_whitespace(src, offset);

        let mut count: usize = 0;
        for c in src.chars() {
            if count == 0 {
                if c.is_ascii_alphabetic() {
                    count += 1;
                } else {
                    return None;
                }
            } else {
                if c.is_ascii_alphanumeric() {
                    count += 1;
                } else {
                    break;
                }
            }
        }

        let (left, right) = src.split_at(count);
        if right.starts_with(separator) {
            offset.seek(left);
            Some((left, right, offset))
        } else {
            None
        }
    }

    /// キーワード
    pub fn parse_keyword<'a>(
        mut src: &'a str,
        keyword: &str,
        mut offset: Offset,
    ) -> Option<(&'a str, &'a str, Offset)> {
        (src, offset) = skip_whitespace(src, offset);

        if src.starts_with(keyword) {
            let (left, right) = src.split_at(keyword.len());

            if right.starts_with(separator) {
                offset.seek(left);
                Some((left, right, offset))
            } else {
                None
            }
        } else {
            None
        }
    }

    /// シンボル
    pub fn parse_symbol<'a>(
        mut src: &'a str,
        symbol: &str,
        mut offset: Offset,
    ) -> Option<(&'a str, &'a str, Offset)> {
        (src, offset) = skip_whitespace(src, offset);
        if src.starts_with(symbol) {
            let (left, right) = src.split_at(symbol.len());
            offset.seek(left);
            Some((left, right, offset))
        } else {
            None
        }
    }

    /// 数値リテラル
    pub fn parse_number_literal(mut src: &str, mut offset: Offset) -> Option<(&str, &str, Offset)> {
        (src, offset) = skip_whitespace(src, offset);
        if let Some(t) = parse_number_literal_bin(src, offset) {
            Some(t)
        } else if let Some(t) = parse_number_literal_dight(src, offset) {
            Some(t)
        } else if let Some(t) = parse_number_literal_hex(src, offset) {
            Some(t)
        } else {
            None
        }
    }

    fn parse_number_literal_bin(src: &str, mut offset: Offset) -> Option<(&str, &str, Offset)> {
        if src.starts_with("0b") || src.starts_with("0B") {
            let mut count: usize = 0;
            for c in src[2..].chars() {
                if c == '0' || c == '1' {
                    break;
                }
                count += c.len_utf8();
            }
            if count != 0 {
                let (left, right) = src.split_at(count + 2);
                if right.starts_with(separator) {
                    offset.seek(left);
                    Some((left, right, offset))
                } else {
                    None
                }
            } else {
                None
            }
        } else {
            None
        }
    }

    fn parse_number_literal_dight(src: &str, mut offset: Offset) -> Option<(&str, &str, Offset)> {
        let mut count: usize = 0;
        for c in src.chars() {
            if !c.is_ascii_digit() {
                break;
            }
            count += c.len_utf8();
        }
        if count != 0 {
            let (left, right) = src.split_at(count);
            if right.starts_with(separator) {
                offset.seek(left);
                Some((left, right, offset))
            } else {
                None
            }
        } else {
            None
        }
    }

    fn parse_number_literal_hex(src: &str, mut offset: Offset) -> Option<(&str, &str, Offset)> {
        if src.starts_with("0x") || src.starts_with("0X") {
            let mut count: usize = 0;
            for c in src[2..].chars() {
                if !c.is_ascii_hexdigit() {
                    break;
                }
                count += c.len_utf8();
            }
            if count != 0 {
                let (left, right) = src.split_at(count + 2);
                if right.starts_with(separator) {
                    offset.seek(left);
                    Some((left, right, offset))
                } else {
                    None
                }
            } else {
                None
            }
        } else {
            None
        }
    }

    /// 文字列リテラル
    pub fn parse_string_literal(mut src: &str, mut offset: Offset) -> Option<(&str, &str, Offset)> {
        (src, offset) = skip_whitespace(src, offset);
        if src.starts_with('"') {
            let mut count: usize = 0;
            let mut prefix_flag = false;
            for c in src[1..].chars() {
                if prefix_flag {
                    if !(c == 'n' || c == 'r' || c == '0' || c == '\\' || c == '"') {
                        return None;
                    }
                    prefix_flag = false;
                } else {
                    match c {
                        '\\' => prefix_flag = true,
                        '\"' => break,
                        _ => (),
                    }
                }
                count += c.len_utf8();
            }
            let (left, right) = src.split_at(count + 2);
            if right.starts_with(separator) {
                offset.seek(left);
                Some((left, right, offset))
            } else {
                None
            }
        } else {
            None
        }
    }

    /// 文字リテラル
    pub fn parse_char_literal(mut src: &str, mut offset: Offset) -> Option<(&str, &str, Offset)> {
        (src, offset) = skip_whitespace(src, offset);
        let mut count: usize = 2;
        let mut chars = src.chars();

        if chars.next() != Some('\'') {
            return None;
        }

        match chars.next()? {
            '\\' => {
                let c = chars.next()?;
                if !(c == 'n' || c == 'r' || c == '0' || c == '\\' || c == '"') {
                    return None;
                }
                count += 1 + c.len_utf8();
            }
            '\'' => return None,
            c => count += c.len_utf8(),
        }

        if chars.next() != Some('\'') {
            return None;
        }

        let (left, right) = src.split_at(count);
        if right.starts_with(separator) {
            offset.seek(left);
            Some((left, right, offset))
        } else {
            None
        }
    }

    // 空白を読み飛ばす
    fn skip_whitespace(src: &str, mut offset: Offset) -> (&str, Offset) {
        let count = src.len() - src.trim_start().len();
        let (left, right) = src.split_at(count);
        offset.seek(left);
        (right, offset)
    }

    /// 波カッコ
    pub fn parse_proc_block(mut src: &str, mut offset: Offset) -> Option<(&str, &str, Offset)> {
        (src, offset) = skip_whitespace(src, offset);
        let mut count: usize = 1;
        let (_, mut s, mut offset) = parse_symbol(src, "{", offset)?;
        loop {
            if let Some((left, right, o)) = skip(src, offset) {
                s = right;
                offset = o;
                count += left.len();
            } else {
                (_, _, offset) = parse_symbol(s, "}", offset)?;
                let (left, right) = src.split_at(count + 1);
                offset.seek(left);
                break Some((left, right, offset));
            }
        }
    }

    /// カッコ
    pub fn parse_expr_block(mut src: &str, mut offset: Offset) -> Option<(&str, &str, Offset)> {
        (src, offset) = skip_whitespace(src, offset);
        let mut count: usize = 1;
        let (_, mut s, mut offset) = parse_symbol(src, "(", offset)?;
        loop {
            if let Some((left, right, o)) = skip(src, offset) {
                s = right;
                offset = o;
                count += left.len();
            } else {
                (_, _, offset) = parse_symbol(s, ")", offset)?;
                let (left, right) = src.split_at(count + 1);
                offset.seek(left);
                break Some((left, right, offset));
            }
        }
    }

    /// カギカッコ
    pub fn parse_index_block(mut src: &str, mut offset: Offset) -> Option<(&str, &str, Offset)> {
        (src, offset) = skip_whitespace(src, offset);
        let mut count: usize = 1;
        let (_, mut s, mut offset) = parse_symbol(src, "[", offset)?;
        loop {
            if let Some((left, right, o)) = skip(src, offset) {
                s = right;
                offset = o;
                count += left.len();
            } else {
                (_, _, offset) = parse_symbol(s, "]", offset)?;
                let (left, right) = src.split_at(count + 1);
                offset.seek(left);
                break Some((left, right, offset));
            }
        }
    }

    pub fn parse_generics_block(mut src: &str, mut offset: Offset) -> Option<(&str, &str, Offset)> {
        (src, offset) = skip_whitespace(src, offset);
        let mut count: usize = 1;
        let (_, mut s, mut offset) = parse_symbol(src, "<", offset)?;
        loop {
            if let Some((left, right, o)) = skip(src, offset) {
                s = right;
                offset = o;
                count += left.len();
            } else {
                (_, _, offset) = parse_symbol(s, ">", offset)?;
                let (left, right) = src.split_at(count + 1);
                offset.seek(left);
                break Some((left, right, offset));
            }
        }
    }

    pub fn skip(src: &str, offset: Offset) -> Option<(&str, &str, Offset)> {
        const PARSERS: &[fn(&str, Offset) -> Option<(&str, &str, Offset)>] = &[
            parse_identifier,
            parse_number_literal,
            parse_string_literal,
            parse_char_literal,
            parse_proc_block,
            parse_expr_block,
            parse_index_block,
            parse_generics_block,
        ];
        for p in PARSERS {
            if let Some(t) = p(src, offset) {
                return Some(t);
            }
        }
        // parse_keyword
        const KEYWORDS: &[&str] = &["fn", "let", "mut"];
        for k in KEYWORDS {
            if let Some(t) = parse_keyword(src, k, offset) {
                return Some(t);
            }
        }

        // parse_symbol
        const SYMBOLS: &[&str] = &["@", ":", "=", ";"];
        for s in SYMBOLS {
            if let Some(t) = parse_symbol(src, s, offset) {
                return Some(t);
            }
        }
        None
    }

    fn separator(c: char) -> bool {
        c.is_ascii_whitespace() || c.is_ascii_punctuation()
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

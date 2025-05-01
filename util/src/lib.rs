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

    #[derive(Clone, Copy, Debug)]
    pub struct Parser<'a> {
        src: &'a str,
        offset: Offset,
    }

    impl<'a> Parser<'a> {
        pub fn new(src: &'a str) -> Self {
            Parser {
                src: src,
                offset: Offset { row: 1, column: 1 },
            }
        }

        pub fn build(s: (Offset, &'a str)) -> Self {
            Parser {
                src: s.1,
                offset: s.0,
            }
        }

        pub fn is_empty(&self) -> bool {
            self.src.trim().is_empty()
        }

        pub fn offset(&self) -> Offset {
            self.offset
        }

        pub fn parse_identifier(&mut self) -> Option<(Offset, &'a str)> {
            let mut self_copy = *self;
            let a = self_copy.parse_identifier_()?;
            *self = self_copy;
            Some(a)
        }

        fn parse_identifier_(&mut self) -> Option<(Offset, &'a str)> {
            self.skip_whitespace();

            let mut count: usize = 0;
            for c in self.src.chars() {
                if !(c.is_ascii_alphabetic() || (count != 0 && c.is_ascii_digit())) {
                    break;
                }
                count += c.len_utf8();
            }

            if count != 0 {
                let ident: &str;
                let ident_offset = self.offset;
                (ident, self.src) = self.src.split_at(count);
                self.offset.seek(ident);

                if self.exist_separator() {
                    Some((ident_offset, ident))
                } else {
                    None
                }
            } else {
                None
            }
        }

        pub fn parse_keyword(&mut self, keyword: &str) -> Option<(Offset, &'a str)> {
            let mut self_copy = *self;
            let a = self_copy.parse_keyword_(keyword)?;
            *self = self_copy;
            Some(a)
        }

        fn parse_keyword_(&mut self, keyword: &str) -> Option<(Offset, &'a str)> {
            let (o, k) = self.parse_identifier_()?;
            if k == keyword {
                Some((o, k))
            } else {
                None
            }
        }

        pub fn parse_symbol(&mut self, symbol: &str) -> Option<(Offset, &'a str)> {
            let mut self_copy = *self;
            let a = self_copy.parse_symbol_(symbol)?;
            *self = self_copy;
            Some(a)
        }

        fn parse_symbol_(&mut self, symbol: &str) -> Option<(Offset, &'a str)> {
            self.skip_whitespace();
            if self.src.starts_with(symbol) {
                let symbol_offset = self.offset;
                let s: &str;
                (s, self.src) = self.src.split_at(symbol.len());
                self.offset.seek(s);
                Some((symbol_offset, s))
            } else {
                None
            }
        }

        pub fn parse_number_literal(&mut self) -> Option<(Offset, &'a str)> {
            let mut self_copy = *self;
            let a = self_copy.parse_number_literal_()?;
            *self = self_copy;
            Some(a)
        }

        fn parse_number_literal_(&mut self) -> Option<(Offset, &'a str)> {
            self.parse_number_literal_bin_()
                .or(self.parse_number_literal_digit_())
                .or(self.parse_number_literal_hex_())
        }

        fn parse_number_literal_bin_(&mut self) -> Option<(Offset, &'a str)> {
            self.skip_whitespace();
            let value_offset = self.offset;
            if !(self.src.starts_with("0b") || self.src.starts_with("0B")) {
                return None;
            }
            self.src = &self.src[2..];
            self.offset.column += 2;

            let mut count: usize = 0;
            for c in self.src.chars() {
                if !(c == '0' || c == '1') {
                    break;
                }
                count += c.len_utf8();
            }

            if count != 0 {
                let value: &str;
                (value, self.src) = self.src.split_at(count);
                if self.exist_separator() {
                    self.offset.seek(value);
                    Some((value_offset, value))
                } else {
                    None
                }
            } else {
                None
            }
        }

        fn parse_number_literal_digit_(&mut self) -> Option<(Offset, &'a str)> {
            self.skip_whitespace();
            let value_offset = self.offset;

            let mut count: usize = 0;
            for c in self.src.chars() {
                if !c.is_ascii_digit() {
                    break;
                }
                count += c.len_utf8();
            }

            if count != 0 {
                let value: &str;
                (value, self.src) = self.src.split_at(count);
                if self.exist_separator() {
                    self.offset.seek(value);
                    Some((value_offset, value))
                } else {
                    None
                }
            } else {
                None
            }
        }

        fn parse_number_literal_hex_(&mut self) -> Option<(Offset, &'a str)> {
            self.skip_whitespace();
            let value_offset = self.offset;
            if !(self.src.starts_with("0x") || self.src.starts_with("0X")) {
                return None;
            }
            self.src = &self.src[2..];
            self.offset.column += 2;

            let mut count: usize = 0;
            for c in self.src.chars() {
                if !c.is_ascii_hexdigit() {
                    break;
                }
                count += c.len_utf8();
            }

            if count != 0 {
                let value: &str;
                (value, self.src) = self.src.split_at(count);
                if self.exist_separator() {
                    self.offset.seek(value);
                    Some((value_offset, value))
                } else {
                    None
                }
            } else {
                None
            }
        }

        pub fn parse_string_literal(&mut self) -> Option<(Offset, &'a str)> {
            let mut self_copy = *self;
            let a = self_copy.parse_string_literal_()?;
            *self = self_copy;
            Some(a)
        }

        fn parse_string_literal_(&mut self) -> Option<(Offset, &'a str)> {
            self.parse_symbol_("\"")?;
            let value_offset = self.offset;
            let mut prefix_flag = false;
            let mut count: usize = 0;
            let mut chars = self.src.chars();

            loop {
                let c = chars.next()?;

                if prefix_flag {
                    prefix_flag = false;
                    match c {
                        'n' | 'r' | '0' | 'a' | '\\' | '\"' | '\'' => (),
                        _ => return None,
                    }
                } else {
                    match c {
                        '\\' => prefix_flag = true,
                        '\"' => break,
                        _ => (),
                    }
                }

                count += c.len_utf8();
            }

            let value = &self.src[0..count];
            self.src = &self.src[count..];
            self.parse_symbol_("\"")?;
            Some((value_offset, value))
        }

        pub fn parse_char_literal(&mut self) -> Option<(Offset, &'a str)> {
            let mut self_copy = *self;
            let a = self_copy.parse_string_literal_()?;
            *self = self_copy;
            Some(a)
        }

        fn parse_char_literal_(&mut self) -> Option<(Offset, &'a str)> {
            self.parse_symbol_("\'")?;
            let value_offset = self.offset;
            let mut chars = self.src.chars();
            let count: usize = match chars.next()? {
                '\\' => match chars.next()? {
                    'n' | 'r' | '0' | 'a' | '\\' | '\"' | '\'' => 2,
                    _ => return None,
                },
                '\'' => return None,
                c => c.len_utf8(),
            };

            let value = &self.src[0..count];
            self.src = &self.src[count..];
            self.parse_symbol_("\'")?;
            Some((value_offset, value))
        }

        pub fn parse_proc_block(&mut self) -> Option<(Offset, &'a str)> {
            let mut self_copy = *self;
            let a = self_copy.parse_proc_block_()?;
            *self = self_copy;
            Some(a)
        }

        fn parse_proc_block_(&mut self) -> Option<(Offset, &'a str)> {
            self.parse_block_("{", "}")
        }

        pub fn parse_expr_block(&mut self) -> Option<(Offset, &'a str)> {
            let mut self_copy = *self;
            let a = self_copy.parse_expr_block_()?;
            *self = self_copy;
            Some(a)
        }

        fn parse_expr_block_(&mut self) -> Option<(Offset, &'a str)> {
            self.parse_block_("(", ")")
        }

        pub fn parse_index_block(&mut self) -> Option<(Offset, &'a str)> {
            let mut self_copy = *self;
            let a = self_copy.parse_index_block_()?;
            *self = self_copy;
            Some(a)
        }

        fn parse_index_block_(&mut self) -> Option<(Offset, &'a str)> {
            self.parse_block_("[", "]")
        }

        fn parse_block_(&mut self, start: &str, end: &str) -> Option<(Offset, &'a str)> {
            self.parse_symbol(start)?;
            let offset = self.offset;
            let src = self.src;
            let mut count: usize = 0;

            loop {
                if self.src.starts_with(end) && self.parse_symbol(end).is_some() {
                    return Some((offset, &src[0..count]));
                }
                let (_, token) = self.skip()?;
                count += token.len();
            }
        }

        pub fn skip(&mut self) -> Option<(Offset, &'a str)> {
            let len = self.src.len() - self.src.trim_start().len();
            if len != 0 {
                let offset = self.offset;
                let value = &self.src[0..len];
                self.src = &self.src[len..];
                self.offset.seek(value);
                return Some((offset, value));
            }

            let parsers: &[fn(&mut Parser<'a>) -> Option<(Offset, &'a str)>] = &[
                Parser::parse_identifier,
                Parser::parse_number_literal,
                Parser::parse_string_literal,
                Parser::parse_char_literal,
            ];
            for p in parsers {
                if let Some(r) = p(self) {
                    return Some(r);
                }
            }

            if self.src.starts_with(|c: char| c.is_ascii_punctuation()) {
                let value = &self.src[0..1];
                let offset = self.offset;
                self.src = &self.src[1..];
                self.offset.seek(value);
                return Some((offset, value));
            }

            None
        }

        fn exist_separator(&self) -> bool {
            if let Some(c) = self.src.chars().next() {
                c.is_ascii_whitespace() || c.is_ascii_punctuation()
            } else {
                true
            }
        }

        fn skip_whitespace(&mut self) {
            let count: usize = self.src.len() - self.src.trim_start().len();
            let (left, right) = self.src.split_at(count);
            self.offset.seek(left);
            self.src = right;
        }
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

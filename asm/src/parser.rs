use super::line::Line;
use std::iter::Iterator;
use std::str::Lines;
use util::svec::SVec;

#[derive(Clone)]
pub struct Parser<'a> {
    line_iter: Lines<'a>,
    line: usize,
}

impl<'a> Parser<'a> {
    pub fn new(code: &'a str) -> Self {
        Parser {
            line_iter: code.lines(),
            line: 0,
        }
    }
}

impl<'a> Parser<'a> {
    fn remove_comment(code: &str) -> &str {
        if let Some((left, _)) = code.split_once("//") {
            left
        } else {
            code
        }
    }

    fn get_label_mnemonic_helper(code: &str, split: char) -> (Option<&str>, &str) {
        let code = code.trim();
        if let Some((left, right)) = code.split_once(split) {
            let left = left.trim();

            if is_valid_keyword(left) {
                (Some(left), right)
            } else {
                (None, code)
            }
        } else {
            (None, code)
        }
    }

    fn get_label(code: &str) -> (Option<&str>, &str) {
        Self::get_label_mnemonic_helper(code, ':')
    }

    fn get_mnemonic(code: &str) -> (Option<&str>, &str) {
        let code = code.trim();
        if code.contains(' ') {
            Self::get_label_mnemonic_helper(code, ' ')
        } else if !code.is_empty() {
            (Some(code), "")
        } else {
            (None, "")
        }
    }

    fn get_operands(code: &str) -> Result<SVec<2, &str>, ()> {
        let code = code.trim();
        let mut iter = code.split(',');
        let mut svec = SVec::new();

        for i in 0..2 {
            if let Some(s) = iter.next() {
                let trimed_s = s.trim();
                if !trimed_s.is_empty() {
                    svec.push(trimed_s);
                }
            } else {
                break;
            }
        }

        if let None = iter.next() {
            Ok(svec)
        } else {
            Err(())
        }
    }
}

impl<'a> Iterator for Parser<'a> {
    type Item = Result<(usize, Line<'a>), usize>;

    fn next(&mut self) -> Option<Self::Item> {
        self.line += 1;
        let Some(mut code) = self.line_iter.next() else {
            return None;
        };

        code = Self::remove_comment(code);
        let (label, code) = Self::get_label(code);
        let (mnemonic, code) = Self::get_mnemonic(code);
        let Ok(operands) = Self::get_operands(code) else {
            return Some(Err(self.line));
        };

        Some(Ok((self.line, Line::new(label, mnemonic, operands))))
    }
}

fn is_valid_keyword(word: &str) -> bool {
    let mut chars = word.chars();
    let Some(c) = chars.next() else {
        return false;
    };

    if !c.is_ascii_alphabetic() {
        return false;
    }

    for i in chars {
        if !i.is_ascii_alphanumeric() {
            return false;
        }
    }

    true
}

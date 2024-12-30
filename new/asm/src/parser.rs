use crate::line::Line;
use std::iter::Iterator;
use std::str::Lines;

/// Simple parser for assembly
/// # Usage
/// ```
/// use asm::parser::Parser;
/// let source = "
///     .text
///     main:
///     push rbp
///     mov rbp, rsp
///
///     mov rax, 0
///     mov rsp, rbp
///     pop rbp
///     ret";
///
/// let parser = Parser::new(source);
///
/// for line in parser {
///     println!("{:?}", line);
/// }
/// ```
#[derive(Clone, Debug)]
pub struct Parser<'a> {
    lines: Lines<'a>,
}

impl<'a> Parser<'a> {
    /// Create new Parser
    pub fn new(source: &'a str) -> Self {
        Parser {
            lines: source.lines(),
        }
    }
}

impl<'a> Iterator for Parser<'a> {
    type Item = Line<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        let line = self.lines.next()?.trim();

        if line.is_empty() {
            return Some(Line::None);
        }
        if parser_helper::is_label(line) {
            return Some(Line::Label(line));
        }
        if parser_helper::is_asm_command(line) {
            return Some(Line::AsmCommand(line));
        }
        if parser_helper::is_instruction(line) {
            return Some(Line::Instruction(line));
        }
        Some(Line::UnKnown(line))
    }
}

mod parser_helper {
    fn is_keyword(mut word: &str) -> bool {
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

    pub fn is_label(mut line: &str) -> bool {
        line = line.trim();
        if !line.ends_with(':') {
            return false;
        }
        line = &line[..line.len() - ':'.len_utf8()].trim();
        is_keyword(line)
    }

    pub fn is_asm_command(mut line: &str) -> bool {
        line = line.trim();
        if !line.starts_with('.') {
            return false;
        }
        line = &line[1..].trim();
        is_keyword(line)
    }

    pub fn is_instruction(line: &str) -> bool {
        let mut line_split = line.split(' ');

        let Some(mnemonic) = line_split.next() else {
            return false;
        };
        if !is_keyword(mnemonic) {
            return false;
        }

        let Some(_) = line_split.next() else {
            return true;
        };
        let Some(_) = line_split.next() else {
            return true;
        };

        line_split.next().is_none()
    }
}

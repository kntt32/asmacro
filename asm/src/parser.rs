use crate::{
    functions::{is_asm_command, is_instruction, is_label},
    line::Line,
};
use std::{iter::Iterator, str::Lines};

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
        if is_label(line) {
            return Some(Line::Label(line));
        }
        if is_asm_command(line) {
            return Some(Line::AsmCommand(line));
        }
        if is_instruction(line) {
            return Some(Line::Instruction(line));
        }
        Some(Line::Unknown(line))
    }
}

use util::{Offset, stoi};

#[derive(Clone, Debug, PartialEq)]
pub struct TokenTree<'a> {
    tree: Vec<Token<'a>>,
}

impl<'a> TokenTree<'a> {
    pub fn new(src: &'a str) -> Self {
        TokenTree {
            tree: Parser::new(src).collect(),
        }
    }
}

#[derive(Clone, PartialEq, Debug)]
pub enum Token<'a> {
    Token {
        r#type: TokenType,
        src: &'a str,
        offset: Offset,
    },
    Block {
        r#type: BracketType,
        parser: Parser<'a>,
        offset: Offset,
    },
    Error {
        msg: String,
        offset: Offset,
    },
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TokenType {
    Keyword,
    Symbol,
    StringLiteral,
    NumberLiteral,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BracketType {
    CurlyBracket,
    Bracket,
    SquareBracket,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Parser<'a> {
    src: &'a str,
    offset: Offset,
}

impl<'a> Parser<'a> {
    pub fn new(src: &'a str) -> Self {
        Parser {
            src: src,
            offset: Offset { column: 1, row: 1 },
        }
    }

    fn parse_keyword(&mut self) -> Option<Token<'a>> {
        fn is_keyword(c: char, is_first: bool) -> bool {
            if is_first {
                c.is_ascii_alphabetic()
            } else {
                c.is_ascii_alphanumeric()
            }
        }

        let mut offset = self.offset;

        let mut len = 0;
        for c in self.src.chars() {
            if !is_keyword(c, len == 0) {
                if len == 0 {
                    return None;
                } else {
                    break;
                }
            }
            len += c.len_utf8();
            offset.column += 1;
        }

        let token = Token::Token {
            r#type: TokenType::Keyword,
            src: &self.src[..len],
            offset: self.offset,
        };
        self.src = &self.src[len..];
        self.offset = offset;
        Some(token)
    }

    fn parse_symbol(&mut self) -> Option<Token<'a>> {
        const SYMBOLS: &[&str] = &[
            "fn", "mut", "let", "->", ":", ",", "@", ";", "+", "-", "*", "/", "<", ">", "=",
        ];
        let offset = self.offset;

        for s in SYMBOLS {
            if self.src.starts_with(s) {
                self.offset.column += s.chars().count();
                self.src = &self.src[s.len()..];
                return Some(Token::Token {
                    r#type: TokenType::Symbol,
                    src: s,
                    offset: offset,
                });
            }
        }
        None
    }

    fn parse_block(&mut self) -> Option<Token<'a>> {
        self.parse_brackets_helper('{', '}', BracketType::CurlyBracket)
    }

    fn parse_bracket(&mut self) -> Option<Token<'a>> {
        self.parse_brackets_helper('(', ')', BracketType::Bracket)
    }

    fn parse_square(&mut self) -> Option<Token<'a>> {
        self.parse_brackets_helper('[', ']', BracketType::SquareBracket)
    }

    fn parse_brackets_helper(
        &mut self,
        start: char,
        end: char,
        r#type: BracketType,
    ) -> Option<Token<'a>> {
        let offset = self.offset;

        if !self.src.starts_with(start) {
            return None;
        }
        self.src = &self.src[start.len_utf8()..];
        self.offset.column += 1;

        let init_src = self.src;
        let init_offset = self.offset;
        let len = self.src.len();
        let parser = loop {
            self.parse_whitespace();
            if self.src.starts_with(end) {
                let parser = Parser {
                    src: &init_src[..len - self.src.len()],
                    offset: init_offset,
                };
                self.offset.column += 1;
                self.src = &self.src[end.len_utf8()..];
                break parser;
            }
            let Some(_) = self.next() else {
                return Some(Token::Error {
                    msg: "unclosed bracket".to_string(),
                    offset: self.offset,
                });
            };
        };

        Some(Token::Block {
            r#type: r#type,
            parser: parser,
            offset: offset,
        })
    }

    fn parse_string_literal(&mut self) -> Option<Token<'a>> {
        let offset = self.offset;

        let mut chars = self.src.chars();

        let mut len = 0;
        let mut escape_flag = false;
        let mut token = None;

        match chars.next() {
            Some('\"') => self.offset.column += 1,
            _ => return None,
        }

        loop {
            let Some(c) = chars.next() else {
                return Some(Token::Error {
                    msg: "mismatch double quate".to_string(),
                    offset: self.offset,
                });
            };

            len += c.len_utf8();
            self.offset.column += 1;

            if !escape_flag {
                match c {
                    '\\' => escape_flag = true,
                    '\n' => {
                        self.offset.row = 0;
                        self.offset.column += 1;
                    }
                    '\"' => break,
                    _ => (),
                }
            } else {
                escape_flag = false;
                match c {
                    '\\' | 'r' | 'n' | '\"' | '\'' => (),
                    _ => {
                        token = Some(Token::Error {
                            msg: "unknown character escape".to_string(),
                            offset: self.offset,
                        })
                    }
                }
            }
        }
        if let Some(t) = token {
            Some(t)
        } else {
            let src = &self.src[..len];
            self.src = &self.src[..len];
            Some(Token::Token {
                r#type: TokenType::StringLiteral,
                src: src,
                offset: offset,
            })
        }
    }

    fn parse_number_literal(&mut self) -> Option<Token<'a>> {
        fn is_number_literal_element(c: char) -> bool {
            c.is_ascii_hexdigit()
                || c == 'b'
                || c == 'o'
                || c == 'x'
                || c == 'B'
                || c == 'O'
                || c == 'X'
        }
        let offset = self.offset;

        let mut len = 0;
        for c in self.src.chars() {
            if !is_number_literal_element(c) {
                if c.is_ascii_punctuation() || c.is_ascii_whitespace() {
                    break;
                } else {
                    return None;
                }
            }
            len += c.len_utf8();
            self.offset.column += 1;
        }

        let src = &self.src[..len];
        if len != 0 && stoi(src).is_some() {
            let token = Token::Token {
                r#type: TokenType::NumberLiteral,
                src: src,
                offset: offset,
            };
            self.src = &self.src[len..];
            Some(token)
        } else {
            None
        }
    }

    fn parse_whitespace(&mut self) {
        for c in self.src.chars() {
            if c.is_ascii_whitespace() {
                if c == '\n' {
                    self.offset.column = 0;
                    self.offset.row += 1;
                } else {
                    self.offset.column += 1;
                }
                self.src = &self.src[c.len_utf8()..];
            } else {
                break;
            }
        }
    }
}

impl<'a> Iterator for Parser<'a> {
    type Item = Token<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        self.parse_whitespace();
        if self.src.len() == 0 {
            return None;
        }
        let token = self
            .parse_symbol()
            .or_else(|| self.parse_block())
            .or_else(|| self.parse_keyword())
            .or_else(|| self.parse_bracket())
            .or_else(|| self.parse_square())
            .or_else(|| self.parse_string_literal())
            .or_else(|| self.parse_number_literal())
            .or_else(|| {
                let token = Token::Error {
                    msg: "unknown expresssion".to_string(),
                    offset: self.offset,
                };
                let start_char = self.src.chars().next().expect("internal error");
                self.src = &self.src[start_char.len_utf8()..];
                self.offset.column += 1;
                Some(token)
            })
            .expect("internal error");
        Some(token)
    }
}

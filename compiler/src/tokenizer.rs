use util::functions::stoi;

#[derive(Clone, Debug, PartialEq)]
pub struct TokenTree<'a> {
    tree: Vec<Token<'a>>,
}

impl<'a> TokenTree<'a> {
    pub fn new(src: &'a str) -> Self {
        TokenTree {
            tree: Tokenizer::new(src).collect(),
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum Token<'a> {
    Keyword {
        src: &'a str,
        row: usize,
        column: usize,
    },
    Symbol {
        src: &'a str,
        column: usize,
        row: usize,
    },
    Block {
        tree: TokenTree<'a>,
        column: usize,
        row: usize,
    },
    Bracket {
        tree: TokenTree<'a>,
        column: usize,
        row: usize,
    },
    Square {
        tree: TokenTree<'a>,
        column: usize,
        row: usize,
    },
    StringLiteral {
        src: &'a str,
        column: usize,
        row: usize,
    },
    NumberLiteral {
        src: &'a str,
        column: usize,
        row: usize,
    },
    Err {
        msg: String,
        column: usize,
        row: usize,
    },
}

#[derive(Clone, Copy, Debug, PartialEq)]
struct Tokenizer<'a> {
    src: &'a str,
    column: usize,
    row: usize,
}

impl<'a> Tokenizer<'a> {
    pub fn new(src: &'a str) -> Self {
        Tokenizer {
            src: src,
            column: 1,
            row: 1,
        }
    }

    fn tokenize_keyword(&mut self) -> Option<Token<'a>> {
        fn is_keyword(c: char, is_first: bool) -> bool {
            if is_first {
                c.is_ascii_alphabetic()
            } else {
                c.is_ascii_alphanumeric()
            }
        }

        let mut column = self.column;

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
            column += 1;
        }

        let token = Token::Keyword {
            src: &self.src[..len],
            column: self.column,
            row: self.row,
        };
        self.src = &self.src[len..];
        self.column = column;
        Some(token)
    }

    fn tokenize_symbol(&mut self) -> Option<Token<'a>> {
        const symbols: &[&str] = &["->", ":", ",", "@", ";", "+", "-", "*", "/"];
        let column = self.column;
        let row = self.row;

        for s in symbols {
            if self.src.starts_with(s) {
                self.column += s.chars().count();
                self.src = &self.src[s.len()..];
                return Some(Token::Symbol {
                    src: s,
                    column: column,
                    row: row,
                });
            }
        }
        None
    }

    fn tokenize_block(&mut self) -> Option<Token<'a>> {
        self.tokenize_brackets_helper('{', '}')
    }

    fn tokenize_bracket(&mut self) -> Option<Token<'a>> {
        self.tokenize_brackets_helper('(', ')')
    }

    fn tokenize_square(&mut self) -> Option<Token<'a>> {
        self.tokenize_brackets_helper('[', ']')
    }

    fn tokenize_brackets_helper(&mut self, start: char, end: char) -> Option<Token<'a>> {
        let mut tree = Vec::new();
        let column = self.column;
        let row = self.row;

        if !self.src.starts_with(start) {
            return None;
        }
        self.src = &self.src[start.len_utf8()..];
        self.column += 1;

        loop {
            self.tokenize_whitespace();
            if self.src.starts_with(end) {
                self.column += 1;
                self.src = &self.src[end.len_utf8()..];
                break;
            }
            let child_token = self.next()?;
            tree.push(child_token);
        }

        Some(Token::Block {
            tree: TokenTree { tree: tree },
            column: column,
            row: row,
        })
    }

    fn tokenize_string_literal(&mut self) -> Option<Token<'a>> {
        let mut column = self.column;
        let mut row = self.row;

        let mut chars = self.src.chars();

        let mut len = 0;
        let mut escape_flag = false;
        let mut token = None;

        match chars.next() {
            Some('\"') => column += 1,
            _ => return None,
        }

        loop {
            let Some(c) = chars.next() else {
                return Some(Token::Err {
                    msg: "mismatch double quate".to_string(),
                    column: self.column,
                    row: self.row,
                });
            };

            len += c.len_utf8();
            column += 1;

            if !escape_flag {
                match c {
                    '\\' => escape_flag = true,
                    '\n' => {
                        row = 0;
                        column += 1;
                    }
                    '\"' => break,
                    _ => (),
                }
            } else {
                escape_flag = false;
                match c {
                    '\\' | 'r' | 'n' | '\"' | '\'' => (),
                    _ => {
                        token = Some(Token::Err {
                            msg: "unknown character escape".to_string(),
                            column: self.column,
                            row: self.row,
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
            self.column = column;
            self.row = row;
            Some(Token::StringLiteral {
                src: src,
                column: self.column,
                row: self.row,
            })
        }
    }

    fn tokenize_number_literal(&mut self) -> Option<Token<'a>> {
        fn is_number_literal_element(c: char) -> bool {
            c.is_ascii_hexdigit()
                || c == 'b'
                || c == 'o'
                || c == 'x'
                || c == 'B'
                || c == 'O'
                || c == 'X'
        }

        let mut column = self.column;

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
            column += 1;
        }

        let src = &self.src[..len];
        if len != 0 && stoi(src).is_some() {
            let token = Token::NumberLiteral {
                src: src,
                column: self.column,
                row: self.row,
            };
            self.src = &self.src[len..];
            self.column = column;
            Some(token)
        } else {
            None
        }
    }

    fn tokenize_whitespace(&mut self) {
        for c in self.src.chars() {
            if c.is_ascii_whitespace() {
                if c == '\n' {
                    self.column = 0;
                    self.row += 1;
                } else {
                    self.column += 1;
                }
                self.src = &self.src[c.len_utf8()..];
            } else {
                break;
            }
        }
    }
}

impl<'a> Iterator for Tokenizer<'a> {
    type Item = Token<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        self.tokenize_whitespace();
        if self.src.len() == 0 {
            return None;
        }
        let token = self
            .tokenize_keyword()
            .or_else(|| self.tokenize_block())
            .or_else(|| self.tokenize_symbol())
            .or_else(|| self.tokenize_bracket())
            .or_else(|| self.tokenize_square())
            .or_else(|| self.tokenize_string_literal())
            .or_else(|| self.tokenize_number_literal())
            .or_else(|| {
                let token = Token::Err {
                    msg: "unknown expresssion".to_string(),
                    column: self.column,
                    row: self.row,
                };
                let start_char = self.src.chars().next().expect("internal error");
                self.src = &self.src[start_char.len_utf8()..];
                self.column += 1;
                Some(token)
            })
            .expect("internal error");
        Some(token)
    }
}

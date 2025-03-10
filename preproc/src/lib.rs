use std::ops::{Deref, DerefMut};
use tokenizer::Tokenizer;
use util::stoi;

/// トークン化イテレータ
mod tokenizer;

pub type SystemMacro = Box<dyn Fn(&mut TokenTree, usize, &mut Vec<UserMacro>) -> bool>;
pub type UserMacro = Box<dyn Fn(&mut TokenTree, usize) -> bool>;

#[derive(Clone, Debug, PartialEq)]
pub struct TokenTree {
    tree: Vec<Token>,
}

#[derive(Clone, Debug, PartialEq)]
pub enum Token {
    Literal(String),
    Block(TokenTree),
}

impl Token {
    pub fn cat(tokens: &[Token]) -> Option<String> {
        let mut string = String::new();
        for i in tokens {
            match i {
                Token::Literal(ref s) => string += s,
                Token::Block(ref b) => string += &Self::cat(&*b)?,
            }
        }
        Some(string)
    }

    pub fn is_literal(&self) -> bool {
        match self {
            Self::Literal(_) => true,
            Self::Block(_) => false,
        }
    }

    pub fn is_block(&self) -> bool {
        match self {
            Self::Literal(_) => false,
            Self::Block(_) => true,
        }
    }
}

impl TokenTree {
    pub fn new(src: &str) -> Self {
        let tokenizer = Tokenizer::new(src);
        Self::from_tokenizer(tokenizer)
    }

    fn from_tokenizer(tokenizer: Tokenizer) -> Self {
        let mut tree = Vec::new();

        for t in tokenizer {
            let token = match t {
                tokenizer::Token::Literal(l) => Token::Literal(l.to_string()),
                tokenizer::Token::Block(b) => Token::Block(Self::from_tokenizer(b)),
            };
            tree.push(token);
        }

        TokenTree { tree: tree }
    }

    pub fn process(&mut self, system_macros: &[SystemMacro]) {
        let mut user_macros: Vec<UserMacro> = Vec::new();
        self.process_(system_macros, &mut user_macros);
    }

    fn process_(&mut self, system_macros: &[SystemMacro], user_macros: &mut Vec<UserMacro>) {
        let mut index = 0;
        while index < self.len() {
            self.process_index_full(index, system_macros, user_macros);
            index += 1;
        }
    }

    fn process_index_full(
        &mut self,
        index: usize,
        system_macros: &[SystemMacro],
        user_macros: &mut Vec<UserMacro>,
    ) {
        let mut i = index + 1;
        while 0 < i {
            i -= 1;
            if i < self.len() {
                self.process_index_until(i, system_macros, user_macros);
            }
        }
    }

    fn process_index_until(
        &mut self,
        index: usize,
        system_macros: &[SystemMacro],
        user_macros: &mut Vec<UserMacro>,
    ) {
        if let Token::Block(ref mut b) = self[index] {
            b.process_(system_macros, user_macros);
        }

        while index < self.len() && self.process_index(index, system_macros, user_macros) {}
    }

    fn process_index(
        &mut self,
        index: usize,
        system_macros: &[SystemMacro],
        user_macros: &mut Vec<UserMacro>,
    ) -> bool {
        for m in system_macros {
            if m(self, index, user_macros) {
                return true;
            }
        }

        for m in user_macros {
            if m(self, index) {
                return true;
            }
        }

        false
    }

    fn match_with(&self, index: usize, with: &TokenTree) -> Option<Vec<Token>> {
        if index < with.len() - 1 || self.len() <= index {
            return None;
        }

        let mut args = Vec::new();

        let index_base = index + 1 - with.len();
        for i in 0..with.len() {
            match with[i] {
                Token::Literal(ref l) => {
                    if l == "$" {
                        args.push(self[index_base + i].clone());
                    } else if self[index_base + i] != with[i] {
                        return None;
                    }
                }
                Token::Block(ref b) => {
                    if let Token::Block(ref t) = self[index_base + i] {
                        let mut block_args = t.match_with(b.len() - 1, b)?;
                        args.append(&mut block_args);
                    } else {
                        return None;
                    }
                }
            }
        }

        Some(args)
    }

    pub fn cat() -> SystemMacro {
        Box::new(
            |tt: &mut TokenTree, index: usize, _user_macros: &mut Vec<UserMacro>| {
                let is_match = if let Token::Literal(ref name) = tt[index] {
                    name == "#cat" && 1 <= index
                } else {
                    false
                };

                if is_match {
                    let Token::Block(ref input) = tt[index - 1] else {
                        return false;
                    };
                    let new_token = if let Some(s) = Token::cat(&*input) {
                        Token::Literal(s)
                    } else {
                        return false;
                    };
                    tt.remove(index - 1);
                    tt.remove(index - 1);
                    tt.insert(index - 1, new_token);
                    true
                } else {
                    false
                }
            },
        )
    }

    fn replace_args(&mut self, args: &[Token]) {
        for t in &mut self.tree {
            match t {
                Token::Literal(l) => {
                    if l.starts_with("$") {
                        let arg_number = &l['$'.len_utf8()..];
                        if let Some(n) = stoi(arg_number) {
                            if 0 <= n && n <= args.len() as i128 {
                                *t = args[n as usize].clone();
                            }
                        }
                    }
                }
                Token::Block(b) => b.replace_args(args),
            }
        }
    }

    pub fn def() -> SystemMacro {
        Box::new(
            |tt: &mut TokenTree, index: usize, user_macros: &mut Vec<UserMacro>| {
                let is_match = if let Token::Literal(ref name) = tt[index] {
                    name == "#def" && 2 <= index
                } else {
                    false
                };

                if is_match {
                    let Token::Block(ref matching_rule) = tt[index - 1] else {
                        return false;
                    };
                    let Token::Block(ref inner_block) = tt[index - 2] else {
                        return false;
                    };
                    let matching_rule: TokenTree = matching_rule.clone();
                    let inner_block: TokenTree = inner_block.clone();

                    let new_macro = Box::new(move |tt: &mut TokenTree, index: usize| {
                        let matching_rule = &matching_rule;
                        let inner_block = &inner_block;

                        if let Some(args) = tt.match_with(index, matching_rule) {
                            let index_base = index - matching_rule.len() + 1;
                            for _ in 0..matching_rule.len() {
                                tt.remove(index_base);
                            }
                            let mut inner = inner_block.clone();
                            inner.replace_args(&args);
                            for _ in 0..inner.tree.len() {
                                tt.insert(index_base, inner.pop().expect("unknon error"));
                            }
                            true
                        } else {
                            false
                        }
                    });
                    for _ in 0..3 {
                        tt.remove(index - 2);
                    }
                    user_macros.push(new_macro);
                    true
                } else {
                    false
                }
            },
        )
    }

    pub fn standard_macros() -> Vec<SystemMacro> {
        vec![Self::cat(), Self::def()]
    }
}

impl Deref for TokenTree {
    type Target = Vec<Token>;

    fn deref(&self) -> &Self::Target {
        &self.tree
    }
}

impl DerefMut for TokenTree {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.tree
    }
}

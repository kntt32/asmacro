use crate::{tokenizer, tokenizer::Tokenizer};
use std::ops::{Deref, DerefMut};
use util::types::SResult;

pub type SystemMacro =
    Box<dyn Fn(&mut TokenTree, usize, &mut Vec<UserMacro>) -> Option<SResult<()>>>;
pub type UserMacro = Box<dyn Fn(&mut TokenTree, usize) -> Option<SResult<()>>>;

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

    pub fn process(&mut self, system_macros: &[SystemMacro]) -> SResult<()> {
        let mut user_macros: Vec<UserMacro> = Vec::new();

        let mut index = 0;
        while index < self.tree.len() {
            self.process_helper(index, system_macros, &mut user_macros)?;
            index += 1;
        }

        Ok(())
    }

    fn process_helper(
        &mut self,
        index: usize,
        system_macros: &[SystemMacro],
        user_macros: &mut Vec<UserMacro>,
    ) -> SResult<()> {
        let mut i = 0;
        while i <= index && i < self.len() {
            self.process_index_until(i, system_macros, user_macros)?;
            i += 1;
        }
        Ok(())
    }

    fn process_index_until(
        &mut self,
        index: usize,
        system_macros: &[SystemMacro],
        user_macros: &mut Vec<UserMacro>,
    ) -> SResult<()> {
        if let Token::Block(ref mut b) = self[index] {
            for i in 0..b.len() {
                b.process_index_until(i, system_macros, user_macros)?;
            }
        }

        loop {
            match self.process_index(index, system_macros, user_macros) {
                Some(Ok(())) => (),
                Some(Err(e)) => break Err(e),
                None => break Ok(()),
            }
        }
    }

    fn process_index(
        &mut self,
        index: usize,
        system_macros: &[SystemMacro],
        user_macros: &mut Vec<UserMacro>,
    ) -> Option<SResult<()>> {
        for m in system_macros {
            match m(self, index, user_macros) {
                Some(r) => return Some(r),
                None => (),
            }
        }

        for m in user_macros {
            match m(self, index) {
                Some(r) => return Some(r),
                None => (),
            }
        }

        None
    }
    /*
    pub fn def() -> SystemMacro {
        //Box<dyn Fn(&mut TokenTree<'_>, usize, &mut Vec<UserMacro>) -> Option<SResult<()>>>
        Box::new(| tt: &mut TokenTree<'_>, index: usize, user_macros: &mut Vec<UserMacro> | {
            if tt[index] == Token::Literal("#def") {
                let Token::Literal(macro_name) = tt[index + 1] else {
                    return None;
                };
                let macro_name_string = macro_name.to_string();
                let Token::Block(macro_inner) =

                let new_macro = Box::new(move | tt: &mut TokenTree<'_>, index: usize | {
                    let name = macro_name_string();
                    let sharp_len_utf8 = '#'.len_utf8();
                    if &tt[index][0 .. sharp_len_utf8] == "#" && &tt[index][sharp_len_utf8 .. ] == name {
                    }else {
                        None
                    }
                });
                user_macros.push(new_macro);
                Some(Ok(()))
            }else {
                None
            }
        })
    }*/

    pub fn cat() -> SystemMacro {
        Box::new(
            |tt: &mut TokenTree, index: usize, _user_macros: &mut Vec<UserMacro>| {
                let is_match_name = if let Token::Literal(ref name) = tt[index] {
                    name == "#cat"
                } else {
                    false
                };
                if is_match_name && index + 1 < tt.len() {
                    let Token::Block(ref input) = tt[index + 1] else {
                        return None;
                    };
                    let new_token = Token::Literal(Token::cat(&*input)?);
                    tt.remove(index);
                    tt.remove(index);
                    tt.insert(index, new_token);
                    Some(Ok(()))
                } else {
                    None
                }
            },
        )
    }

    pub fn standard_macros() -> Vec<SystemMacro> {
        vec![Self::cat()]
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

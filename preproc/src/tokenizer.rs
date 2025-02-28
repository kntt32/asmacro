#[derive(Clone, Debug)]
pub enum Token<'a> {
    Literal(&'a str),
    Block(Tokenizer<'a>),
}

#[derive(Clone, Debug)]
pub struct Tokenizer<'a>(RawTokenizer<'a>);

impl<'a> Tokenizer<'a> {
    pub fn new(src: &'a str) -> Self {
        Tokenizer(RawTokenizer::new(src))
    }

    fn iter_helper_block(&mut self) -> Self {
        let new_self = self.clone();
        let mut level = 1;

        loop {
            if let Some(t) = self.0.next() {
                if t == "#" {
                    match self.0.next() {
                        Some("end") => level -= 1,
                        Some("start") => level += 1,
                        None => break,
                        Some(_) => (),
                    }
                    if level == 0 {
                        break;
                    }
                }
            } else {
                break;
            }
        }

        new_self
    }
}

impl<'a> Iterator for Tokenizer<'a> {
    type Item = Token<'a>;

    fn next(&mut self) -> Option<Token<'a>> {
        if let Some(t) = self.0.next() {
            match t {
                "#start" => Some(Token::Block(self.iter_helper_block())),
                "#end" => None,
                t => Some(Token::Literal(t)),
            }
        } else {
            None
        }
    }
}

#[derive(Clone, Copy, Debug)]
struct RawTokenizer<'a>(&'a str);

impl<'a> RawTokenizer<'a> {
    fn new(src: &'a str) -> Self {
        RawTokenizer(src)
    }
}

impl<'a> Iterator for RawTokenizer<'a> {
    type Item = &'a str;

    fn next(&mut self) -> Option<Self::Item> {
        self.0 = self
            .0
            .trim_matches(|c: char| c.is_whitespace() && c != '\n');
        if !self.0.is_empty() {
            let mut count = 0;
            let mut str_flag = false;
            let mut char_flag = false;

            for c in self.0.chars() {
                let is_first = count == 0;
                let is_whitespace = c.is_whitespace() && c != '\n';
                let is_double_quat = c == '\"';
                let is_single_quat = c == '\'';

                let is_sharp = c == '#';
                let is_ascii_punctuation = (c.is_ascii_punctuation() || c == '\n')
                    && !is_double_quat
                    && !is_single_quat
                    && !is_sharp;
                if is_first {
                    str_flag = is_double_quat;
                    char_flag = is_single_quat;
                }
                if is_whitespace || (!is_first && is_ascii_punctuation) {
                    break;
                }
                if !str_flag && is_double_quat {
                    break;
                }
                if !char_flag && is_single_quat {
                    break;
                }
                count += c.len_utf8();
                if is_ascii_punctuation {
                    break;
                }
                if !is_first && ((str_flag && is_double_quat) || (char_flag && is_single_quat)) {
                    break;
                }
            }

            let token = &self.0[0..count];
            self.0 = &self.0[count..];

            Some(token)
        } else {
            None
        }
    }
}

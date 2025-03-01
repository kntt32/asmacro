#[derive(Clone, Debug)]
pub enum Token<'a> {
    Literal(&'a str),
    Block(Tokenizer<'a>),
}

#[derive(Clone, Copy, Debug)]
pub struct Tokenizer<'a> {
    level: usize,
    raw_tokenizer: RawTokenizer<'a>,
}

impl<'a> Tokenizer<'a> {
    pub fn new(src: &'a str) -> Self {
        Tokenizer {
            level: 1,
            raw_tokenizer: RawTokenizer::new(src),
        }
    }

    fn seek_for_end_block(&mut self) {
        let level = self.level;
        self.level = 1;
        loop {
            if self.next().is_none() {
                break;
            }
        }
        self.level = level;
    }
}

impl<'a> Iterator for Tokenizer<'a> {
    type Item = Token<'a>;

    fn next(&mut self) -> Option<Token<'a>> {
        match self.raw_tokenizer.next() {
            Some("#start") => {
                let mut block_tokenizer = *self;
                block_tokenizer.level = 1;
                self.seek_for_end_block();
                Some(Token::Block(block_tokenizer))
            }
            Some("#end") => {
                self.level -= 1;
                if self.level == 0 {
                    None
                } else {
                    self.next()
                }
            }
            Some(t) => Some(Token::Literal(t)),
            None => None,
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
        self.0 = self.0.trim_matches(|c: char| c.is_whitespace());
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

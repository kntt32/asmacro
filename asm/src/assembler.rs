use crate::{line::Line, parser::Parser};

#[derive(Clone, Copy, Debug)]
pub struct Asm<'a> {
    source: &'a str,
}

impl<'a> Asm<'a> {
    pub fn new(source: &'a str) -> Self {
        Asm { source: source }
    }

    pub fn labels(&self) -> Result<Vec<Label<'_>>, usize> {
        let mut labels = Vec::new();
        let parser = Parser::new(self.source);
        let mut offset: usize = 0;
        let mut line_count: usize = 0;

        for line in parser {
            line_count += 1;
            match line {
                Line::None => (),
                Line::Label(s) => labels.push(Label {
                    name: s,
                    offset: offset,
                }),
                Line::Pseudo(_) => todo!(),
                Line::Instruction(_) => {
                    if line.is_valid_instruction() {
                        offset += line.machine_code_len();
                    } else {
                        return Err(line_count);
                    }
                }
                Line::Unknown(_) => return Err(line_count),
            }
        }

        Ok(labels)
    }

    pub fn assemble(&self) -> Result<Vec<u8>, String> {
        let parser = Parser::new(self.source);
        let mut vec = Vec::new();

        let labels = match self.labels() {
            Ok(v) => v,
            Err(n) => return Err(format!("{}: unknown expression", n)),
        };

        for line in parser {
            match line {
                Line::None => (),
                Line::Label(_) => (),
                Line::Pseudo(_) => todo!(),
                Line::Instruction(_) => {
                    line.machine_code(&labels, vec.len())?.push_to(&mut vec);
                }
                Line::Unknown(_) => panic!("unknown error"),
            }
        }

        Ok(vec)
    }
}

#[derive(Clone, Copy, Debug)]
pub struct Label<'a> {
    name: &'a str,
    offset: usize,
}

impl<'a> Label<'a> {
    pub fn name(&self) -> &'a str {
        self.name
    }

    pub fn offset(&self) -> usize {
        self.offset
    }
}

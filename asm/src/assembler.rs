use crate::{line::Line, parser::Parser};

#[derive(Clone, Copy, Debug)]
pub struct Label<'a> {
    name: &'a str,
    offset: usize,
}

pub fn labels(source: &str) -> Result<Vec<Label<'_>>, usize> {
    let mut labels = Vec::new();
    let parser = Parser::new(source);
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
            Line::AsmCommand(_) => todo!(),
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

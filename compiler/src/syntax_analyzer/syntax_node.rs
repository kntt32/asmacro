use super::*;

mod parser {
    pub fn parse_identifier(src: &str) -> Option<(&str, &str)> {
        let mut count: usize = 0;
        for c in src.chars() {
            if count == 0 {
                if c.is_ascii_alphabetic() {
                    count += 1;
                } else {
                    return None;
                }
            } else {
                if c.is_ascii_alphanumeric() {
                    count += 1;
                } else {
                    break;
                }
            }
        }

        let (left, right) = src.split_at(count);
        if right.starts_with(separator) {
            Some(src.split_at(count))
        } else {
            None
        }
    }

    pub fn parse_keyword<'a>(src: &'a str, keyword: &str) -> Option<(&'a str, &'a str)> {
        if src.starts_with(keyword) {
            let (left, right) = src.split_at(keyword.len());

            if right.starts_with(separator) {
                Some((left, right))
            } else {
                None
            }
        } else {
            None
        }
    }

    pub fn parse_symbol<'a>(src: &'a str, symbol: &str) -> Option<(&'a str, &'a str)> {
        if src.starts_with(symbol) {
            let (left, right) = src.split_at(symbol.len());
            Some((left, right))
        } else {
            None
        }
    }

    pub fn parse_number_literal(src: &str) -> Option<(&str, &str)> {
        todo!()
    }

    pub fn parse_string_literal(src: &str) -> Option<(&str, &str)> {
        todo!()
    }

    pub fn parse_proc_block(src: &str) -> Option<(&str, &str)> {
        todo!()
    }

    pub fn parse_expr_block(src: &str) -> Option<(&str, &str)> {
        todo!()
    }

    pub fn parse_index_block(src: &str) -> Option<(&str, &str)> {
        todo!()
    }

    pub fn parse_generics_block(src: &str) -> Option<(&str, &str)> {
        todo!()
    }

    fn separator(c: char) -> bool {
        c.is_ascii_whitespace() || c.is_ascii_punctuation()
    }
}

pub struct VariableDeclaration {
    name: String,
    mutable: bool,
    expr: Box<dyn SyntaxNode>,
    expr_state: Option<Rc<dyn CompilerState>>,
    offset: Offset,
}

impl VariableDeclaration {
    pub fn parse(src: &str) -> Option<(Box<dyn SyntaxNode>, &str)> {
        None
    }
}

impl SyntaxNode for VariableDeclaration {
    fn look_ahead(&mut self, state: Rc<dyn CompilerState>) {
        match state.clone().child_for_proc() {
            Ok(expr_state) => {
                self.expr.look_ahead(expr_state.clone());
                self.expr_state = Some(expr_state);
            }
            Err(msg) => state.add_error(self.offset, msg),
        }
    }

    fn data(&self, state: Rc<dyn CompilerState>) -> SResult<Option<Data>> {
        Ok(None)
    }

    fn compile(&self, state: Rc<dyn CompilerState>) {
        let expr_state = self.expr_state.clone().expect("internal error: The method \"compile\" must be used after the method \"look_ahead\" is called");
        self.expr.compile(expr_state.clone());

        let Ok(Some(expr_data)) = self.expr.data(expr_state) else {
            state.add_error(self.offset, format!("mismatch data"));
            return;
        };
        if state.clone().get_type(&expr_data.r#type).is_some() {
            let object = Object {
                name: Some(self.name.clone()),
                mutable: self.mutable,
                data: expr_data,
            };
            state.add_object(object);
        } else {
            state.add_error(
                self.offset,
                format!("type \"{}\" is undefined", expr_data.r#type),
            );
        }
    }
}

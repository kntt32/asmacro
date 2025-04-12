use super::*;
use util::parser;

pub fn parse(src: &str, offset: Offset) -> Option<(Box<dyn SyntaxNode>, &str, Offset)> {
    const PARSERS: &[fn(&str, Offset) -> Option<(Box<dyn SyntaxNode>, &str, Offset)>] =
        &[VariableDeclaration::parse, NumberLiteral::parse];
    for p in PARSERS {
        if let Some(t) = p(src, offset) {
            return Some(t);
        }
    }
    None
}

#[derive(Debug)]
pub struct VariableDeclaration {
    name: String,
    mutable: bool,
    data: Option<Data>,
    expr: Box<dyn SyntaxNode>,
    offset: Offset,
}

impl VariableDeclaration {
    pub fn parse(src: &str, mut offset: Offset) -> Option<(Box<dyn SyntaxNode>, &str, Offset)> {
        // let ( mut ) $ident (: $ident @ $register ) = $expr ;
        fn parse_let_keyword(s: &mut &str, offset: &mut Offset) -> Option<()> {
            (_, *s, *offset) = parser::parse_keyword(*s, "let", *offset)?;
            Some(())
        }
        fn parse_mut_keyword(s: &mut &str, offset: &mut Offset) -> Option<bool> {
            if let Some(i) = parser::parse_keyword(*s, "mut", *offset) {
                (_, *s, *offset) = i;
                Some(true)
            } else {
                Some(false)
            }
        }
        fn parse_name(s: &mut &str, offset: &mut Offset) -> Option<String> {
            let name;
            (name, *s, *offset) = parser::parse_identifier(*s, *offset)?;
            Some(name.to_string())
        }
        fn parse_colon_symbol(s: &mut &str, offset: &mut Offset) -> Option<()> {
            (_, *s, *offset) = parser::parse_symbol(s, ":", *offset)?;
            Some(())
        }
        fn parse_data(s: &mut &str, offset: &mut Offset) -> Option<Option<Data>> {
            if parse_colon_symbol(s, offset).is_some() {
                let data: Data;
                (data, *s, *offset) = Data::parse(s, *offset)?;
                Some(Some(data))
            } else {
                Some(None)
            }
        }
        fn parse_equal_symbol(s: &mut &str, offset: &mut Offset) -> Option<()> {
            (_, *s, *offset) = parser::parse_symbol(s, "=", *offset)?;
            Some(())
        }
        fn parse_expr(s: &mut &str, offset: &mut Offset) -> Option<Box<dyn SyntaxNode>> {
            let node;
            (node, *s, *offset) = parse(*s, *offset)?;
            Some(node)
        }

        let node_name: String;
        let node_data: Option<Data>;
        let node_mutable: bool;
        let node_expr: Box<dyn SyntaxNode>;
        let node_offset = offset;

        let mut s = src;
        parse_let_keyword(&mut s, &mut offset)?;
        node_mutable = parse_mut_keyword(&mut s, &mut offset)?;
        node_name = parse_name(&mut s, &mut offset)?;
        node_data = parse_data(&mut s, &mut offset)?;
        parse_equal_symbol(&mut s, &mut offset)?;
        node_expr = parse_expr(&mut s, &mut offset)?;

        let syntax_node: Box<dyn SyntaxNode> = Box::new(VariableDeclaration {
            name: node_name,
            mutable: node_mutable,
            data: node_data,
            expr: node_expr,
            offset: node_offset,
        });

        Some((syntax_node, s, offset))
    }
}

impl SyntaxNode for VariableDeclaration {
    fn look_ahead(&self, state: Rc<dyn CompilerState>) {
        // do nothing
    }

    fn data(&self, state: Rc<dyn CompilerState>) -> Option<Data> {
        None
    }

    fn compile(&self, state: Rc<dyn CompilerState>) {
        let expr_state;
        match state.clone().child_for_proc() {
            Ok(es) => {
                expr_state = es;
            }
            Err(msg) => {
                state.add_error(self.offset, msg);
                return;
            }
        }
        self.expr.look_ahead(expr_state.clone());
        self.expr.compile(expr_state.clone());

        let Some(expr_data) = self.expr.data(expr_state) else {
            state.add_error(self.offset, format!("mismatching data, found \"None\""));
            return;
        };
        if state.clone().get_type(&expr_data.r#type).is_some() {
            let match_data = if let Some(ref self_data) = self.data {
                self_data == &expr_data
            } else {
                true
            };
            if match_data {
                let object = Object {
                    name: Some(self.name.clone()),
                    mutable: self.mutable,
                    data: expr_data,
                };
                if let Err(msg) = state.clone().add_object(object) {
                    state.add_error(self.offset, msg);
                }
            } else {
                state.add_error(self.offset, format!("mismatching data"));
            }
        } else {
            state.add_error(
                self.offset,
                format!("type \"{}\" is undefined", expr_data.r#type),
            );
        }
    }
}

#[derive(Debug)]
pub struct FunctionDeclaration {
    function: Function,
    procedure: Vec<Box<dyn SyntaxNode>>,
    offset: Offset,
}

impl FunctionDeclaration {
    pub fn parse(mut src: &str, mut offset: Offset) -> Option<(Box<dyn SyntaxNode>, &str, Offset)> {
        fn parse_fn_keyword(s: &mut &str, offset: &mut Offset) -> Option<()> {
            (_, *s, *offset) = parser::parse_keyword(*s, "fn", *offset)?;
            Some(())
        }
        fn parse_name_identifier(s: &mut &str, offset: &mut Offset) -> Option<String> {
            let name: &str;
            (name, *s, *offset) = parser::parse_identifier(*s, *offset)?;
            Some(name.to_string())
        }
        fn parse_arguments(s: &mut &str, offset: &mut Offset) -> Option<Vec<Object>> {
            todo!()
        }
        fn parse_data(s: &mut &str, offset: &mut Offset) -> Option<Option<Data>> {
            todo!()
        }
        fn parse_proc(s: &mut &str, offset: &mut Offset) -> Option<Vec<Box<dyn SyntaxNode>>> {
            todo!()
        }

        let node_offset = offset;
        parse_fn_keyword(&mut src, &mut offset)?;
        let name = parse_name_identifier(&mut src, &mut offset)?;
        let arguments = parse_arguments(&mut src, &mut offset)?;
        let data = parse_data(&mut src, &mut offset)?;
        let procedure = parse_proc(&mut src, &mut offset)?;

        Some((
            Box::new(FunctionDeclaration {
                function: Function {
                    name: name.to_string(),
                    arguments: arguments,
                    data: data,
                },
                procedure: procedure,
                offset: node_offset,
            }),
            src,
            offset,
        ))
    }
}

impl SyntaxNode for FunctionDeclaration {
    fn look_ahead(&self, state: Rc<dyn CompilerState>) {
        if let Err(msg) = state.clone().add_function(self.function.clone()) {
            state.add_error(self.offset, msg);
        }
    }

    fn data(&self, state: Rc<dyn CompilerState>) -> Option<Data> {
        None
    }

    fn compile(&self, state: Rc<dyn CompilerState>) {
        match state.clone().child_for_proc() {
            Ok(child_state) => {
                for node in &self.procedure {
                    node.look_ahead(child_state.clone());
                }
                let code = format!("{}:\n", &self.function.name);
                state.clone().add_asm(&code);
                for node in &self.procedure {
                    node.compile(child_state.clone());
                }
                let proc_len = self.procedure.len();
                let proc_data = if proc_len != 0 {
                    self.procedure[proc_len - 1].data(child_state.clone())
                } else {
                    None
                };
                if &proc_data != &self.function.data {
                    state.add_error(self.offset, format!("expected data"));
                }
            }
            Err(msg) => {
                state.add_error(self.offset, msg);
            }
        }
    }
}

#[derive(Clone, Debug)]
pub struct NumberLiteral {
    value: String,
    r#type: Option<String>,
    register: Option<Register>,
    offset: Offset,
}

impl NumberLiteral {
    pub fn parse(src: &str, mut offset: Offset) -> Option<(Box<dyn SyntaxNode>, &str, Offset)> {
        // $num (: $type) (@ $register)
        fn parse_value(s: &mut &str, offset: &mut Offset) -> Option<String> {
            let value;
            (value, *s, *offset) = parser::parse_number_literal(*s, *offset)?;
            Some(value.to_string())
        }
        fn parse_type(s: &mut &str, offset: &mut Offset) -> Option<Option<String>> {
            let Some(i) = parser::parse_symbol(*s, ":", *offset) else {
                return Some(None);
            };
            (_, *s, *offset) = i;
            let r#type: &str;
            (r#type, *s, *offset) = parser::parse_identifier(*s, *offset)?;
            Some(Some(r#type.to_string()))
        }
        fn parse_register(s: &mut &str, offset: &mut Offset) -> Option<Option<Register>> {
            let Some(i) = parser::parse_symbol(*s, "@", *offset) else {
                return Some(None);
            };
            (_, *s, *offset) = i;
            let register: &str;
            (register, *s, *offset) = parser::parse_identifier(*s, *offset)?;

            if let Ok(r) = register.parse() {
                Some(Some(r))
            } else {
                None
            }
        }

        let mut s = src;
        let node_offset = offset;
        let node_value = parse_value(&mut s, &mut offset)?;
        let node_type = parse_type(&mut s, &mut offset)?;
        let node_register = parse_register(&mut s, &mut offset)?;

        let syntax_node = Box::new(NumberLiteral {
            value: node_value,
            r#type: node_type,
            register: node_register,
            offset: node_offset,
        });

        Some((syntax_node, s, offset))
    }
}

impl SyntaxNode for NumberLiteral {
    fn look_ahead(&self, state: Rc<dyn CompilerState>) {
        // do nothing
    }

    fn data(&self, state: Rc<dyn CompilerState>) -> Option<Data> {
        let r#type = if let Some(ref t) = self.r#type {
            t.clone()
        } else {
            "i32".to_string()
        };
        let register = if let Some(r) = self.register {
            r
        } else {
            Register::Rax
        };
        Some(Data {
            r#type: r#type,
            register: register,
        })
    }

    fn compile(&self, state: Rc<dyn CompilerState>) {
        let data = self.data(state.clone()).expect("internal error");
        let data_register = data.register;
        let object = Object {
            name: None,
            mutable: false,
            data: data,
        };
        if let Err(msg) = state.clone().add_object(object) {
            state.add_error(self.offset, msg);
            return;
        }
        let code = format!("mov {} {}\n", data_register, &self.value);
        state.clone().add_asm(&code);
    }
}

use super::{CompilerState, SyntaxNode};
use crate::types::{Data, Function, Object, Type};
use asm::assembler::register::Register;
use std::rc::Rc;
use util::{Offset, parser, parser::Parser};

pub fn parse(parser: &mut Parser<'_>) -> Option<Box<dyn SyntaxNode>> {
    const PARSERS: &[fn(p: &mut Parser<'_>) -> Option<Box<dyn SyntaxNode>>] = &[
        VariableDeclaration::parse,
        VariableAssign::parse,
        FunctionDeclaration::parse,
        CallingFunction::parse,
        NumberLiteral::parse,
        ReferVariableExpr::parse,
        AtExpr::parse,
    ];

    if parser.is_empty() {
        return None;
    }
    for p in PARSERS {
        if let Some(t) = p(parser) {
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
    pub fn parse(p: &mut Parser<'_>) -> Option<Box<dyn SyntaxNode>> {
        // let (object) = $expr;
        let mut p_copy = *p;
        let a = Self::parse_(&mut p_copy)?;
        *p = p_copy;
        Some(a)
    }

    fn parse_(p: &mut Parser<'_>) -> Option<Box<dyn SyntaxNode>> {
        let offset = p.offset();
        p.parse_keyword("let")?;
        let mutable = p.parse_keyword("mut").is_some();
        let name = p.parse_identifier()?;
        let data = if p.parse_symbol(":").is_some() {
            Some(Data::parse(p)?)
        } else {
            None
        };
        p.parse_symbol("=")?;
        let expr = parse(p)?;
        let node = VariableDeclaration {
            name: name.to_string(),
            mutable: mutable,
            data: data,
            expr: expr,
            offset: offset,
        };
        Some(Box::new(node))
    }
}

impl SyntaxNode for VariableDeclaration {
    fn look_ahead(&self, state: Rc<dyn CompilerState>) {
        self.expr.look_ahead(state.clone());
    }

    fn data(&self, state: Rc<dyn CompilerState>) -> Option<Data> {
        None
    }

    fn compile(&self, state: Rc<dyn CompilerState>) {
        self.expr.compile(state.clone());

        let Some(expr_data) = self.expr.data(state.clone()) else {
            state.add_error(
                self.offset,
                "right expression doesn't return any value".to_string(),
            );
            return;
        };
        let expr_data_register = expr_data.register;

        let data = if let Some(ref d) = self.data {
            d.clone()
        } else {
            expr_data
        };
        let object = Object {
            name: Some(self.name.clone()),
            mutable: self.mutable,
            data: data,
        };

        state.clone().move_object(expr_data_register, object);
    }
}

#[derive(Debug)]
pub struct FunctionDeclaration {
    function: Function,
    procedure: Vec<Box<dyn SyntaxNode>>,
    offset: Offset,
}

impl FunctionDeclaration {
    /// パース
    pub fn parse(p: &mut Parser<'_>) -> Option<Box<dyn SyntaxNode>> {
        let mut p_copy = *p;
        let a = Self::parse_(&mut p_copy)?;
        *p = p_copy;
        Some(a)
    }

    fn parse_(p: &mut Parser<'_>) -> Option<Box<dyn SyntaxNode>> {
        fn get_arguments_(p: &mut Parser<'_>) -> Option<Vec<Object>> {
            let mut arguments = Vec::new();
            p.parse_symbol("(")?;

            loop {
                if p.parse_symbol(")").is_some() {
                    if p.is_empty() {
                        break;
                    } else {
                        return None;
                    }
                }
                let object = Object::parse(p)?;
                arguments.push(object);
                if p.parse_symbol(",").is_none() {
                    if p.parse_symbol(")").is_some() {
                        if p.is_empty() {
                            break;
                        } else {
                            return None;
                        }
                    }
                }
            }

            Some(arguments)
        }
        fn get_procedure_(p: &mut Parser<'_>) -> Option<Vec<Box<dyn SyntaxNode>>> {
            let mut procedure = Vec::new();

            p.parse_symbol("{")?;
            loop {
                if p.parse_symbol("}").is_some() {
                    if p.is_empty() {
                        procedure.push(Box::new(UnitExpr()) as Box<dyn SyntaxNode>);
                        break;
                    } else {
                        return None;
                    }
                }
                let Some(expr) = parse(p) else {
                    return None;
                };
                procedure.push(expr);

                if p.parse_symbol(";").is_none() {
                    if p.parse_symbol("}").is_some() {
                        if p.is_empty() {
                            break;
                        } else {
                            return None;
                        }
                    }
                }
            }

            Some(procedure)
        }

        let offset = p.offset();
        let name = p.parse_identifier()?;
        let mut args_parser = Parser::build(p.offset(), p.parse_expr_block()?);
        let data = if p.parse_symbol("->").is_some() {
            Some(Data::parse(p)?)
        } else {
            None
        };
        let mut proc_parser = Parser::build(p.offset(), p.parse_proc_block()?);
        let arguments: Vec<Object> = get_arguments_(&mut args_parser)?;
        let procedure: Vec<Box<dyn SyntaxNode>> = get_procedure_(&mut proc_parser)?;

        let node = FunctionDeclaration {
            function: Function {
                name: name.to_string(),
                arguments: arguments,
                data: data,
            },
            procedure: procedure,
            offset: offset,
        };

        Some(Box::new(node))
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
    r#type: String,
    register: Register,
    offset: Offset,
}

impl NumberLiteral {
    pub fn parse(p: &mut Parser<'_>) -> Option<Box<dyn SyntaxNode>> {
        let mut p_copy = *p;
        let a = Self::parse_(&mut p_copy)?;
        *p = p_copy;
        Some(a)
    }

    fn parse_(p: &mut Parser<'_>) -> Option<Box<dyn SyntaxNode>> {
        // $value (:$type) (@$register)
        let offset = p.offset();
        let value = p.parse_number_literal()?;
        let r#type = if p.parse_symbol(":").is_some() {
            p.parse_identifier()?.to_string()
        } else {
            "i32".to_string()
        };
        let register = if p.parse_symbol("@").is_some() {
            let Ok(r) = p.parse_identifier()?.parse() else {
                return None;
            };
            r
        } else {
            Register::Eax
        };

        let node = NumberLiteral {
            value: value.to_string(),
            r#type: r#type,
            register: register,
            offset: offset,
        };
        Some(Box::new(node))
    }
}

impl SyntaxNode for NumberLiteral {
    fn look_ahead(&self, state: Rc<dyn CompilerState>) {
        // do nothing
    }

    fn data(&self, state: Rc<dyn CompilerState>) -> Option<Data> {
        Some(Data {
            r#type: self.r#type.clone(),
            register: self.register,
        })
    }

    fn compile(&self, state: Rc<dyn CompilerState>) {
        let data = self.data(state.clone()).expect("internal error");
        let object = Object {
            name: None,
            mutable: false,
            data: data,
        };
        if let Err(msg) = state.clone().add_object(object) {
            state.add_error(self.offset, msg);
            return;
        }
        let code = format!("mov {} {}\n", self.register, &self.value);
        state.clone().add_asm(&code);
    }
}

/// 変数参照を表す構造体
#[derive(Clone, Debug)]
pub struct ReferVariableExpr {
    name: String,
    offset: Offset,
}

impl ReferVariableExpr {
    pub fn parse(p: &mut Parser<'_>) -> Option<Box<dyn SyntaxNode>> {
        let offset = p.offset();
        let name = p.parse_identifier()?;
        let node = ReferVariableExpr {
            name: name.to_string(),
            offset: offset,
        };
        Some(Box::new(node))
    }
}

impl SyntaxNode for ReferVariableExpr {
    fn look_ahead(&self, state: Rc<dyn CompilerState>) {
        // do nothing
    }

    fn data(&self, state: Rc<dyn CompilerState>) -> Option<Data> {
        Some(state.get_object_by_name(&self.name)?.data)
    }

    fn compile(&self, state: Rc<dyn CompilerState>) {
        // do nothing
    }
}

/// 代入式を表す構造体
#[derive(Debug)]
pub struct VariableAssign {
    left: String,
    right: Box<dyn SyntaxNode>,
    offset: Offset,
}

impl VariableAssign {
    pub fn parse(p: &mut Parser<'_>) -> Option<Box<dyn SyntaxNode>> {
        let mut copy_p = *p;
        let a = Self::parse_(&mut copy_p)?;
        *p = copy_p;
        Some(a)
    }

    fn parse_(p: &mut Parser<'_>) -> Option<Box<dyn SyntaxNode>> {
        // $name = $expr
        let offset = p.offset();
        let left = p.parse_identifier()?;
        p.parse_symbol("=")?;
        let right_expr = parse(p)?;
        let node = VariableAssign {
            left: left.to_string(),
            right: right_expr,
            offset: offset,
        };
        Some(Box::new(node))
    }
}

impl SyntaxNode for VariableAssign {
    fn look_ahead(&self, state: Rc<dyn CompilerState>) {
        self.right.look_ahead(state.clone());
    }

    fn data(&self, state: Rc<dyn CompilerState>) -> Option<Data> {
        None
    }

    fn compile(&self, state: Rc<dyn CompilerState>) {
        let Some(left_object) = state.clone().get_object_by_name(&self.left) else {
            state.add_error(
                self.offset,
                format!("variable \"{}\" is undefined", self.left),
            );
            return;
        };
        let Some(right_data) = self.right.data(state.clone()) else {
            state.add_error(self.offset, format!("right expression returns no value"));
            return;
        };
        if left_object.data == right_data {
            if left_object.mutable {
                self.right.compile(state.clone());
            } else {
                state.add_error(
                    self.offset,
                    format!("variable \"{}\" is immutable", &self.left),
                );
            }
        } else {
            state.add_error(
                self.offset,
                format!("mismatching data of left and right expression"),
            );
        }
    }
}

#[derive(Debug)]
pub struct CallingFunction {
    name: String,
    arguments: Vec<Box<dyn SyntaxNode>>,
    offset: Offset,
}

impl CallingFunction {
    pub fn parse(p: &mut Parser<'_>) -> Option<Box<dyn SyntaxNode>> {
        let mut p_copy = *p;
        let a = Self::parse_(&mut p_copy)?;
        *p = p_copy;
        Some(a)
    }

    fn parse_(p: &mut Parser<'_>) -> Option<Box<dyn SyntaxNode>> {
        let offset = p.offset();
        let name = p.parse_identifier()?;
        let mut args_parser = Parser::build(p.offset(), p.parse_expr_block()?);
        let mut arguments = Vec::new();
        args_parser.parse_symbol("(")?;
        loop {
            if args_parser.parse_symbol(")").is_some() {
                if args_parser.is_empty() {
                    break;
                } else {
                    return None;
                }
            }

            let node = parse(&mut args_parser)?;
            arguments.push(node);
            if args_parser.parse_symbol(",").is_none() {
                if args_parser.parse_symbol(")").is_some() {
                    if args_parser.is_empty() {
                        break;
                    } else {
                        return None;
                    }
                }
            }
        }

        let node = CallingFunction {
            name: name.to_string(),
            arguments: arguments,
            offset: offset,
        };
        Some(Box::new(node))
    }
}

impl SyntaxNode for CallingFunction {
    fn look_ahead(&self, state: Rc<dyn CompilerState>) {
        for i in &self.arguments {
            i.look_ahead(state.clone());
        }
    }

    fn data(&self, state: Rc<dyn CompilerState>) -> Option<Data> {
        state.get_function(&self.name)?.data
    }

    fn compile(&self, state: Rc<dyn CompilerState>) {
        let Some(function) = state.clone().get_function(&self.name) else {
            return;
        };

        // 引数が正当かチェック
        if self.arguments.len() == function.arguments.len() {
            for i in 0..self.arguments.len() {
                let Some(argument_data) = self.arguments[i].data(state.clone()) else {
                    state
                        .clone()
                        .add_error(self.offset, format!("mismatching argument[{}]", i));
                    return;
                };
                if &argument_data != &function.arguments[i].data {
                    state
                        .clone()
                        .add_error(self.offset, format!("mismatching argument[{}]", i));
                    return;
                }
            }
        } else {
            state
                .clone()
                .add_error(self.offset, format!("mismatching arguments length"));
            return;
        }

        for i in &self.arguments {
            let i_data = i.data(state.clone());
            i.compile(state.clone());
            if let Some(k) = i_data {
                state.clone().drop_object_by_register(k.register);
            }
        }

        if let Some(data) = function.data {
            let object = Object {
                name: None,
                mutable: false,
                data: data,
            };
            state.clone().add_object(object);
        }

        state.clone().add_asm(&format!("call \"{}\"\n", self.name));
    }
}

/// 何も返さない式を表す構造体
#[derive(Clone, Debug)]
pub struct UnitExpr();

impl SyntaxNode for UnitExpr {
    fn look_ahead(&self, state: Rc<dyn CompilerState>) {
        // do nothing
    }

    fn data(&self, state: Rc<dyn CompilerState>) -> Option<Data> {
        None
    }

    fn compile(&self, state: Rc<dyn CompilerState>) {
        // do nothing
    }
}

#[derive(Debug)]
pub struct AtExpr {
    expr: Box<dyn SyntaxNode>,
    register: Register,
    offset: Offset,
}

impl AtExpr {
    pub fn parse(p: &mut Parser<'_>) -> Option<Box<dyn SyntaxNode>> {
        let mut p_copy = *p;
        let a = Self::parse_(&mut p_copy)?;
        *p = p_copy;
        Some(a)
    }

    fn parse_(p: &mut Parser<'_>) -> Option<Box<dyn SyntaxNode>> {
        let offset = p.offset();

        let mut expr_parser = Parser::build(p.offset(), p.parse_expr_block()?);
        expr_parser.parse_symbol("(")?;
        let expr = parse(&mut expr_parser)?;
        expr_parser.parse_symbol(")")?;
        if !expr_parser.is_empty() {
            return None;
        }

        p.parse_symbol("@")?;
        let register_string = p.parse_identifier()?;
        let Ok(register) = register_string.parse::<Register>() else {
            return None;
        };
        let node = AtExpr {
            expr: expr,
            register: register,
            offset: offset,
        };
        Some(Box::new(node))
    }
}

impl SyntaxNode for AtExpr {
    fn look_ahead(&self, state: Rc<dyn CompilerState>) {
        self.expr.look_ahead(state);
    }

    fn data(&self, state: Rc<dyn CompilerState>) -> Option<Data> {
        let mut data = self.expr.data(state)?;
        data.register = self.register;
        Some(data)
    }

    fn compile(&self, state: Rc<dyn CompilerState>) {
        let Some(origin_data) = self.expr.data(state.clone()) else {
            state
                .clone()
                .add_error(self.offset, "expected data".to_string());
            return;
        };
        let Some(data) = self.data(state.clone()) else {
            state
                .clone()
                .add_error(self.offset, "expected data".to_string());
            return;
        };

        self.expr.compile(state.clone());

        let Some(r#type) = state.clone().get_type(&origin_data.r#type) else {
            return;
        };

        let object = Object {
            name: None,
            mutable: false,
            data: data,
        };
        if r#type.copy {
            if let Err(e) = state.clone().copy_object(origin_data.register, object) {
                state.clone().add_error(self.offset, e);
                return;
            }
        } else {
            if let Err(e) = state.clone().move_object(origin_data.register, object) {
                state.clone().add_error(self.offset, e);
                return;
            }
        }
    }
}

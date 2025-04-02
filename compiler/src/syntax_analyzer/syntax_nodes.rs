use super::{Data, Function, Lifetime, Object, State, SyntaxNode, Type};
use asm::assembler::register::Register;
use std::{cell::RefCell, rc::Rc};
use util::{Offset, SResult};

/// 変数宣言を表すSyntaxNode
pub struct VariableDeclaration {
    name: String,
    mutable: bool,
    data: Option<Data>,
    expr: Box<dyn SyntaxNode>,
    node_start_offset: Offset,
    node_end_offset: Offset,
}

impl VariableDeclaration {
    fn look_ahead(&self, state: &mut State) -> SResult<Option<Rc<RefCell<Object>>>> {
        if let Some(object) = self.expr.look_ahead(state)? {
            let mut object_borrow = object.borrow();
            if let Some(ref t) = self.data {
                if &object_borrow.data != t {
                    return Err(format!("mismatch data"));
                }
            }

            let mut new_object = object_borrow.clone();
            new_object.mutable = self.mutable;
            new_object.name = Some(self.name.clone());
            new_object.lifetime.start = self.node_end_offset;
            new_object.lifetime.end = None;

            state.add_object(Rc::new(RefCell::new(new_object)));

            Ok(None)
        } else {
            Ok(None)
        }
    }

    fn compile(&self, state: &mut State) -> SResult<()> {
        self.expr.compile(state)
    }
}

/// i32数値リテラルを表すSyntaxNode
pub struct NumberLiteralExpr {
    value: String,
    register: Register,
    offset: Offset,
}

impl SyntaxNode for NumberLiteralExpr {
    fn look_ahead(&self, state: &mut State) -> SResult<Option<Rc<RefCell<Object>>>> {
        let object = Object {
            name: None,
            mutable: false,
            data: Data {
                r#type: "i32".to_string(),
                register: self.register,
            },
            lifetime: Lifetime {
                start: self.offset,
                end: None,
            },
        };
        let rc_refcell_object = Rc::new(RefCell::new(object));
        state.add_object(rc_refcell_object.clone());
        Ok(Some(rc_refcell_object))
    }

    fn compile(&self, state: &mut State) -> SResult<()> {
        state.assembly += &format!("mov {} {}", self.register.as_str(), &self.value);
        Ok(())
    }
}

/// 変数参照を表すSyntaxNode
pub struct ReferVariableExpr {
    name: String,
    offset: Offset,
}

impl SyntaxNode for ReferVariableExpr {
    fn look_ahead(&self, state: &mut State) -> SResult<Option<Rc<RefCell<Object>>>> {
        if let Some(object) = state.get_object_by_name(&self.name, self.offset) {
            Ok(Some(object))
        } else {
            Err(format!("variable \"{}\" is undefined", &self.name))
        }
    }

    fn compile(&self, state: &mut State) -> SResult<()> {
        Ok(())
    }
}

/// 代入式を表すSyntaxNode
pub struct AssginmentExpr {
    left_expr: Box<dyn SyntaxNode>,
    right_expr: Box<dyn SyntaxNode>,
    node_start_offset: Offset,
    node_end_offset: Offset,
}

impl SyntaxNode for AssginmentExpr {
    fn look_ahead(&self, state: &mut State) -> SResult<Option<Rc<RefCell<Object>>>> {
        let Some(left_object) = self.left_expr.look_ahead(state)? else {
            return Err("invalid left expression".to_string());
        };
        let Some(right_object) = self.right_expr.look_ahead(state)? else {
            return Err("invalid right expression".to_string());
        };
        let left_object_borrow = left_object.borrow();
        let right_object_borrow = right_object.borrow();

        if left_object_borrow.data != right_object_borrow.data {
            return Err("mismatching data".to_string());
        }
        todo!()
    }

    fn compile(&self, state: &mut State) -> SResult<()> {
        todo!()
    }
}

use super::{Data, Function, Lifetime, Object, State, SyntaxNode, Type, Variable};
use std::{cell::RefCell, rc::Rc};
use util::{Offset, SResult};

/// 変数定義を行うSyntaxNode
pub struct VariableDeclaration {
    name: String,
    object: Rc<RefCell<Object>>,
    offset: Offset,
}

impl VariableDeclaration {
    fn offset(&self) -> Offset {
        self.offset
    }

    fn look_ahead(&self, state: &mut State) -> SResult<Object> {
        todo!()
    }

    fn compile(&self, state: &mut State) -> SResult<()> {
        todo!()
    }
}

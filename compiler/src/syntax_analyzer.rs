use asm::assembler::register::Register;
use util::{EResult, ErrorMessage, Offset};

pub struct State {
    variable_list: Vec<Variable>,
    assembly: String,
}

pub struct SyntaxTree {
    tree: Vec<Box<dyn SyntaxNode>>,
    state: State,
}

pub trait SyntaxNode {
    fn offset(&self) -> Offset;
    fn as_data(&self) -> Data;
    fn look_ahead(&self, state: &mut State);
    fn compile(&self, state: &mut State) -> EResult;
}

#[derive(Clone, Debug, PartialEq)]
pub enum Data {
    Some {
        r#type: String,
        storage: Vec<Storage>,
    },
    None,
}

impl Data {
    pub fn doubling(&self, other: &Self) -> bool {
        match self {
            Self::Some { storage: s1, .. } => match other {
                Self::Some { storage: s2, .. } => {
                    for i in s1 {
                        for k in s2 {
                            if i == k {
                                return true;
                            }
                        }
                    }
                    false
                }
                Self::None => false,
            },
            Self::None => false,
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum Storage {
    Register(Register),
    Stack { offset: usize, size: usize },
}

#[derive(Clone, Debug, PartialEq)]
pub struct Lifetime {
    start: Offset,
    end: Option<Offset>,
}

impl Lifetime {
    pub fn alive(&self, offset: Offset) -> bool {
        if self.start <= offset {
            if let Some(self_end) = self.end {
                offset < self_end
            } else {
                true
            }
        } else {
            false
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct Variable {
    name: String,
    data: Data,
    mutable: bool,
    lifetime: Lifetime,
}

pub struct VariableDeclaration {
    variable: Variable,
    expr: Box<dyn SyntaxNode>,
    offset: Offset,
}

impl SyntaxNode for VariableDeclaration {
    fn offset(&self) -> Offset {
        self.offset
    }

    fn as_data(&self) -> Data {
        Data::None
    }

    fn look_ahead(&self, state: &mut State) {
        self.expr.look_ahead(state);
        for i in &mut state.variable_list {
            let alive = i.lifetime.alive(self.variable.lifetime.start);
            if alive && (i.name == self.variable.name || i.data.doubling(&self.variable.data)) {
                i.lifetime.end = Some(self.variable.lifetime.start);
            }
        }
        state.variable_list.push(self.variable.clone());
    }

    fn compile(&self, state: &mut State) -> EResult {
        let expr_data = self.expr.as_data();
        if expr_data == self.variable.data {
            self.expr.compile(state)
        } else {
            Err(ErrorMessage {
                msg: format!("mismatching data"),
                offset: self.offset,
            })
        }
    }
}

pub struct VariableAssignment {
    name: String,
    expr: Box<dyn SyntaxNode>,
    offset: Offset,
}

impl SyntaxNode for VariableAssignment {
    fn offset(&self) -> Offset {
        self.offset
    }

    fn as_data(&self) -> Data {
        Data::None
    }

    fn look_ahead(&self, state: &mut State) {}

    fn compile(&self, state: &mut State) -> EResult {
        for i in &state.variable_list {
            if i.name == self.name && i.lifetime.alive(self.offset) {
                let variable = i;
                let expr_data = self.expr.as_data();
                let variable_data = &variable.data;
                if &expr_data == variable_data {
                    todo!()
                    // return Ok();
                } else {
                    todo!()
                    // return
                }
            }
        }
        Err(ErrorMessage {
            msg: format!("Variable \"{}\" is undefined.", self.name),
            offset: self.offset,
        })
    }
}

use crate::types::{Data, Function, Object, Type};
use asm::assembler::register::Register;
use std::{fmt::Debug, rc::Rc};
use syntax_nodes::parse;
use util::{Offset, SResult, parser::Parser};

mod syntax_nodes;

mod compiler_states;

/// 構文ツリーを表現する構造体
pub struct SyntaxTree {
    tree: Vec<Box<dyn SyntaxNode>>,
    state: Rc<dyn CompilerState>,
}

impl SyntaxTree {
    pub fn new(src: &str) -> Self {
        let mut tree = Vec::new();
        let mut state = Rc::new(compiler_states::GlobalState::new());

        let mut parser = Parser::new(src);
        loop {
            if let Some(node) = parse(&mut parser) {
                tree.push(node);
            } else {
                if !parser.is_empty() {
                    state
                        .clone()
                        .add_error(parser.offset(), format!("unknown token found"));
                }
                break;
            }
        }

        SyntaxTree {
            tree: tree,
            state: state,
        }
    }

    pub fn compile(self) -> Result<String, Vec<(Offset, String)>> {
        for node in &self.tree {
            node.look_ahead(self.state.clone());
        }
        for node in &self.tree {
            node.compile(self.state.clone());
        }
        self.state.clone().status()
    }
}

/// 構文ノードのトレイト
trait SyntaxNode: Debug {
    fn look_ahead(&self, state: Rc<dyn CompilerState>);
    fn data(&self, state: Rc<dyn CompilerState>) -> Option<Data>;
    fn compile(&self, state: Rc<dyn CompilerState>);
}

/// コンパイル中に情報を記録するためのデータ型のトレイト
trait CompilerState {
    fn child_for_proc(self: Rc<Self>) -> SResult<Rc<dyn CompilerState>>;
    // fn child_for_branch_proc(self: Rc<Self>) -> SResult<(Rc<dyn CompilerState>, Rc<dyn CompilerState>)>;

    fn add_function(self: Rc<Self>, function: Function) -> SResult<()>;
    fn get_function(self: Rc<Self>, name: &str) -> Option<Function>;

    fn add_type(self: Rc<Self>, r#type: Type) -> SResult<()>;
    fn get_type(self: Rc<Self>, name: &str) -> Option<Type>;

    fn clean_object(self: Rc<Self>);
    fn add_object(self: Rc<Self>, object: Object) -> SResult<()>;
    fn get_object_by_name(self: Rc<Self>, name: &str) -> Option<Object>;
    fn get_object_by_register(self: Rc<Self>, register: Register) -> Option<Object>;
    fn copy_object(self: Rc<Self>, from: Register, to: Object) -> SResult<()>;
    fn move_object(self: Rc<Self>, from: Register, to: Object) -> SResult<()>;
    fn assgin_object(self: Rc<Self>, to: Register, object: Object) -> SResult<()>;
    fn kill_object_by_name(self: Rc<Self>, name: &str);
    fn kill_object_by_register(self: Rc<Self>, register: Register);
    fn kill_object_without(self: Rc<Self>, without: Register);
    fn kill_object_all(self: Rc<Self>);

    fn consume_object(self: Rc<Self>, register: Register);

    fn add_asm(self: Rc<Self>, code: &str);
    fn add_error(self: Rc<Self>, offset: Offset, msg: String);
    fn status(self: Rc<Self>) -> Result<String, Vec<(Offset, String)>>;
}

use crate::types::{Type, Data, Object, Function};
use std::{rc::Rc, fmt::Debug};
use util::{SResult, Offset, parser::Parser};
use asm::assembler::register::Register;

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

        let parsers = &[
            syntax_nodes::VariableDeclaration::parse,
            syntax_nodes::FunctionDeclaration::parse,
            syntax_nodes::CallingFunction::parse,
            syntax_nodes::NumberLiteral::parse,
            syntax_nodes::ReferVariableExpr::parse,
        ];
        let mut parser = Parser::new(src);
        'a: loop {
            for p in parsers {
                if let Some(node) = p(&mut parser) {
                    tree.push(node);
                    continue 'a;
                }
            }
            if !parser.is_empty() {
                state.clone().add_error(parser.offset(), format!("unknown token found"));
            }
            break;
        }

        SyntaxTree { tree: tree, state: state }
    }

    pub fn compile(&mut self) -> Result<String, Vec<(Offset, String)>> {
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

    fn add_function(self: Rc<Self>, function: Function) -> SResult<()>;
    fn get_function(self: Rc<Self>, name: &str) -> Option<Function>;

    fn add_type(self: Rc<Self>, r#type: Type) -> SResult<()>;
    fn get_type(self: Rc<Self>, name: &str) -> Option<Type>;

    fn clean_object(self: Rc<Self>);
    fn add_object(self: Rc<Self>, object: Object) -> SResult<()>;
    fn get_object_by_name(self: Rc<Self>, name: &str) -> Option<Object>;
    fn get_object_by_register(self: Rc<Self>, register: Register) -> Option<Object>;
    fn map_object_by_name(
        self: Rc<Self>,
        name: &str,
        p: &dyn Fn(Option<&mut Object>) -> SResult<()>,
    ) -> SResult<()>;
    fn map_object_by_register(
        self: Rc<Self>,
        register: Register,
        p: &dyn Fn(Option<&mut Object>) -> SResult<()>,
    ) -> SResult<()>;
    fn drop_object_by_name(self: Rc<Self>, name: &str);
    fn drop_object_by_register(self: Rc<Self>, register: Register);

    fn add_asm(self: Rc<Self>, code: &str);
    fn add_error(self: Rc<Self>, offset: Offset, msg: String);
    fn status(self: Rc<Self>) -> Result<String, Vec<(Offset, String)>>;
}


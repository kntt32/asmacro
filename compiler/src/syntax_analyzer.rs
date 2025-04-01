use asm::assembler::register::Register;
use std::{cell::Ref, cell::RefCell, cmp::min, rc::Rc};
use util::{ErrorMessage, Offset, SResult};

pub mod syntax_nodes;

/// 構文ツリーの要素となるためのトレイト
pub trait SyntaxNode {
    /// ソース中での場所を返す関数
    fn offset(&self) -> Offset;

    /// コンパイル時の先読みを行い、式として返すObjectを返却する関数
    fn look_ahead(&self, state: &mut State) -> SResult<Rc<RefCell<Object>>>;

    /// コンパイルし、アセンブリをState.assemblyに追記する
    fn compile(&self, state: &mut State) -> SResult<()>;
}

/// コンパイル中に情報を記録するための型
#[derive(Clone, Debug)]
pub struct State {
    variable_list: Vec<Rc<RefCell<Variable>>>,
    function_list: Vec<Rc<RefCell<Function>>>,
    type_list: Vec<Rc<RefCell<Type>>>,
    object_list: Vec<Rc<RefCell<Object>>>,
    assembly: String,
}

/// 構文ツリー
pub struct SyntaxTree {
    tree: Vec<Box<dyn SyntaxNode>>,
    state: State,
}

/// データや変数のライフタイムを表す構造体
#[derive(Clone, Debug, PartialEq)]
pub struct Lifetime {
    pub start: Offset,
    pub end: Option<Offset>,
}

/// データ型を表す構造体
#[derive(Clone, Debug, PartialEq)]
pub struct Type {
    name: String,
    avaiable_registers: Vec<Register>,
}

/// データを表す構造体
#[derive(Clone, Debug, PartialEq)]
pub struct Data {
    r#type: String,
    register: Register,
}

/// オブジェクトを表す構造体
#[derive(Clone, Debug, PartialEq)]
pub struct Object {
    data: Data,
    lifetime: Lifetime,
}

impl State {
    pub fn new() -> Self {
        State {
            variable_list: Vec::new(),
            function_list: Vec::new(),
            type_list: Type::primitive_types(),
            object_list: Vec::new(),
            assembly: String::new(),
        }
    }

    pub fn add_variable(&mut self, variable: Rc<RefCell<Variable>>) {
        todo!()
    }
}

impl SyntaxTree {
    pub fn new(tree: Vec<Box<dyn SyntaxNode>>) -> Self {
        SyntaxTree {
            tree: tree,
            state: State::new(),
        }
    }

    pub fn compile(mut self) -> SResult<String> {
        println!("debug0: {:?}\n", self.state);
        for i in &mut self.tree {
            println!("debuga: {:?}\n", self.state);
            i.look_ahead(&mut self.state)?;
        }
        println!("debug1: {:?}\n", self.state);
        for i in &mut self.tree {
            i.compile(&mut self.state)?;
        }
        println!("debug2: {:?}\n", self.state);
        Ok(self.state.assembly)
    }
}

impl Lifetime {
    /// Lifetime型のコンストラクタ
    pub fn new(start: Offset, end: Option<Offset>) -> Self {
        Lifetime {
            start: start,
            end: end,
        }
    }

    /// ライフタイムが存在しているか判定する関数
    pub fn exist(&self) -> bool {
        if let Some(self_end) = self.end {
            self.start < self_end
        } else {
            true
        }
    }

    /// ライフタイムが重複しているか判定する関数
    pub fn doubling(&self, other: &Self) -> bool {
        match (self.end, other.end) {
            (Some(self_end), Some(other_end)) => other.start < self_end && self.start < other_end,
            (Some(self_end), None) => other.start < self_end,
            (None, Some(other_end)) => self.start < other_end,
            (None, None) => true,
        }
    }
}

impl Data {
    /// データの存在が重複しているか判定する関数
    pub fn doubling(&self, other: &Self) -> bool {
        self.register.parent() == other.register.parent()
    }

    /// データ型を取得
    pub fn r#type(&self) -> &str {
        &self.r#type
    }

    /// レジスタを取得
    pub fn register(&self) -> Register {
        self.register
    }
}

impl Type {
    /// プリミティブなデータ型のリストを返す
    pub fn primitive_types() -> Vec<Rc<RefCell<Self>>> {
        let u32 = Type {
            name: "u32".to_string(),
            avaiable_registers: vec![Register::Eax, Register::Ecx, Register::Edx, Register::Ebx],
        };
        let i32 = Type {
            name: "i32".to_string(),
            avaiable_registers: vec![Register::Eax, Register::Ecx, Register::Edx, Register::Ebx],
        };
        vec![Rc::new(RefCell::new(u32)), Rc::new(RefCell::new(i32))]
    }

    /// 使用可能なレジスタを取得
    pub fn avaiable_registers(&self) -> &[Register] {
        &self.avaiable_registers
    }

    /// 名前を取得
    pub fn name(&self) -> &str {
        &self.name
    }
}

/// 変数を表す構造体
#[derive(Clone, Debug, PartialEq)]
pub struct Variable {
    name: String,
    object: Rc<RefCell<Object>>,
    mutable: bool,
}

impl Variable {
    /// Variable型のコンストラクタ
    pub fn new(name: String, object: Rc<RefCell<Object>>, mutable: bool) -> Self {
        Variable {
            name: name,
            object: object,
            mutable: mutable,
        }
    }
}

/// 関数を表す構造体
#[derive(Clone, Debug, PartialEq)]
pub struct Function {
    name: String,
    arguments: Vec<Rc<RefCell<Variable>>>,
    data: Data,
}

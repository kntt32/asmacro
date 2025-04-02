use asm::assembler::register::Register;
use std::{cell::Ref, cell::RefCell, cmp::min, rc::Rc};
use util::{ErrorMessage, Offset, SResult};

pub mod syntax_nodes;

/// 構文ツリーの要素となるためのトレイト
pub trait SyntaxNode {
    /// コンパイル時の先読みを行い、式として返すObjectを返却する関数
    fn look_ahead(&self, state: &mut State) -> SResult<Option<Rc<RefCell<Object>>>>;

    /// コンパイルし、アセンブリをState.assemblyに追記する
    fn compile(&self, state: &mut State) -> SResult<()>;
}

/// コンパイル中に情報を記録するための型
#[derive(Clone, Debug)]
pub struct State {
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
    copy: bool,
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
    name: Option<String>,
    mutable: bool,
    data: Data,
    lifetime: Lifetime,
}

impl State {
    pub fn new() -> Self {
        State {
            function_list: Vec::new(),
            type_list: Type::primitive_types(),
            object_list: Vec::new(),
            assembly: String::new(),
        }
    }

    pub fn add_object(&mut self, object: Rc<RefCell<Object>>) {
        let mut object_borrow_mut = object.borrow_mut();
        for i in &mut self.object_list {
            let mut i_borrow_mut = i.borrow_mut();
            i_borrow_mut.resolve(&mut object_borrow_mut);
        }
        std::mem::drop(object_borrow_mut);
        self.object_list.push(object);
    }

    pub fn get_object_by_name(&self, name: &str, offset: Offset) -> Option<Rc<RefCell<Object>>> {
        for i in &self.object_list {
            let i_borrow = i.borrow();

            let match_name = if let Some(ref i_name) = i_borrow.name {
                i_name == name
            } else {
                false
            };
            let alive = i_borrow.lifetime.alive(offset);

            if match_name && alive {
                return Some(i.clone());
            }
        }
        None
    }

    pub fn get_type_by_name(&self, name: &str) -> Option<Rc<RefCell<Type>>> {
        for i in &self.type_list {
            let i_borrow = i.borrow();
            if &i_borrow.name == name {
                return Some(i.clone());
            }
        }
        None
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

    /// offsetのとき生存しているか判定する関数
    pub fn alive(&self, offset: Offset) -> bool {
        if let Some(self_end) = self.end {
            self.start <= offset && offset < self_end
        } else {
            self.start <= offset
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

    /// ライフタイムの重複を解決する関数
    pub fn resolve(&mut self, other: &mut Self) {
        match (self.end, other.end) {
            (Some(self_end), Some(other_end)) => {
                if other.start < self_end && self.start < other_end {
                    if self.start <= other.start {
                        self.end = Some(other.start);
                    }
                    if other.start <= self.start {
                        other.end = Some(self.start);
                    }
                }
            }
            (Some(self_end), None) => {
                if other.start < self_end {
                    other.end = Some(self.start);
                }
            }
            (None, Some(other_end)) => {
                if self.start < other_end {
                    self.end = Some(other.start);
                }
            }
            (None, None) => {
                if self.start <= other.start {
                    self.end = Some(other.start);
                }
                if other.start <= self.start {
                    other.end = Some(self.start);
                }
            }
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

impl Object {
    /// 存在が重複しているか判定する関数
    pub fn doubling(&self, other: &Self) -> bool {
        self.data.doubling(&other.data) && self.lifetime.doubling(&other.lifetime)
    }

    /// 存在の重複を解決する関数
    pub fn resolve(&mut self, other: &mut Self) {
        if self.doubling(other) {
            self.lifetime.resolve(&mut other.lifetime);
        }
    }
}

impl Type {
    /// プリミティブなデータ型のリストを返す
    pub fn primitive_types() -> Vec<Rc<RefCell<Self>>> {
        let u32 = Type {
            name: "u32".to_string(),
            avaiable_registers: vec![Register::Eax, Register::Ecx, Register::Edx, Register::Ebx],
            copy: true,
        };
        let i32 = Type {
            name: "i32".to_string(),
            avaiable_registers: vec![Register::Eax, Register::Ecx, Register::Edx, Register::Ebx],
            copy: true,
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

/// 関数を表す構造体
#[derive(Clone, Debug, PartialEq)]
pub struct Function {
    name: String,
    arguments: Vec<Rc<RefCell<Object>>>,
    data: Data,
}

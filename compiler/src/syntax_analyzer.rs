use asm::assembler::register::Register;
use std::{cell::RefCell, cmp::min, rc::Rc};
use util::{ErrorMessage, Offset, SResult};

/// コンパイル中に情報を記録するための型
#[derive(Clone, Debug)]
pub struct State {
    variable_list: Vec<Rc<RefCell<Variable>>>,
    function_list: Vec<Rc<RefCell<Function>>>,
    type_list: Vec<Rc<RefCell<Type>>>,
    assembly: String,
}

impl State {
    pub fn new() -> Self {
        State {
            variable_list: Vec::new(),
            function_list: Vec::new(),
            type_list: Type::primitive_types(),
            assembly: String::new(),
        }
    }
}

/// 構文ツリー
pub struct SyntaxTree {
    tree: Vec<Box<dyn SyntaxNode>>,
    state: State,
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

/// 構文ツリーの要素となるためのトレイト
pub trait SyntaxNode {
    /// ソース中での場所を返す関数
    fn offset(&self) -> Offset;

    /// 式として返すデータを返す関数
    fn as_data(&self, state: &State) -> Data;

    /// コンパイル時の先読みを行う関数
    fn look_ahead(&self, state: &mut State) -> SResult<()>;

    /// コンパイルし、アセンブリをState.assemblyに追記する
    fn compile(&self, state: &mut State) -> SResult<()>;
}

/// データを表す列挙体
#[derive(Clone, Debug, PartialEq)]
pub enum Data {
    Some {
        r#type: String,
        storage: Vec<Register>,
    },
    None,
}

impl Data {
    /// データの保存場所が重なっているかどうかを返す関数
    pub fn doubling(&self, other: &Self) -> bool {
        match self {
            Self::Some { storage: s1, .. } => match other {
                Self::Some { storage: s2, .. } => {
                    for i in s1 {
                        for k in s2 {
                            if i.doubling(*k) {
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

/// データ型を表す構造体
#[derive(Clone, Debug, PartialEq)]
pub struct Type {
    name: String,
    storage: Vec<Register>,
}

impl Type {
    /// プリミティブなデータ型のリストを返す
    pub fn primitive_types() -> Vec<Rc<RefCell<Self>>> {
        let u32 = Type {
            name: "u32".to_string(),
            storage: vec![Register::Eax, Register::Ecx, Register::Edx, Register::Ebx],
        };
        vec![Rc::new(RefCell::new(u32))]
    }
}

/// データや変数のライフタイムを表す構造体
#[derive(Clone, Debug, PartialEq)]
pub struct Lifetime {
    start: Offset,
    end: Option<Offset>,
}

impl Lifetime {
    // Lifetime型のコンストラクタ
    pub fn new(start: Offset, end: Option<Offset>) -> Self {
        Lifetime {
            start: start,
            end: end,
        }
    }

    /// ライフタイムの開始を取得する関数
    pub fn start(&self) -> Offset {
        self.start
    }

    /// ライフタイムの寿命を取得する関数
    pub fn end(&self) -> Option<Offset> {
        self.end
    }

    /// ライフタイムの寿命を設定する関数
    /// すでにセットされている値と、渡された値のうち、早い方がセットされる
    pub fn set_end(&mut self, offset: Offset) {
        let min_offset = if let Some(self_end) = self.end {
            min(self_end, offset)
        } else {
            offset
        };
        self.end = Some(min_offset);
    }

    /// ライフタイムの寿命が存在するか判定する関数
    pub fn exist_end(&self) -> bool {
        self.end.is_some()
    }

    /// ソース中offsetでlifetimeが生存しているか判定する関数
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

    /// lifetimeがかぶっているか判定する関数
    pub fn doubling(&self, other: &Self) -> bool {
        let self_start = self.start;
        let other_start = other.start;
        match (self.end, other.end) {
            (Some(self_end), Some(other_end)) => {
                !(self_end < other_start || other_end < self_start)
            }
            (Some(self_end), None) => !(self_end < other_start),
            (None, Some(other_end)) => !(other_end < self_start),
            (None, None) => true,
        }
    }
}

/// 変数を表す構造体
#[derive(Clone, Debug, PartialEq)]
pub struct Variable {
    name: String,
    data: Data,
    mutable: bool,
    lifetime: Lifetime,
}

impl Variable {
    /// Variable型のコンストラクタ
    pub fn new(name: String, data: Data, mutable: bool, lifetime: Lifetime) -> Self {
        Variable {
            name: name,
            data: data,
            mutable: mutable,
            lifetime: lifetime,
        }
    }

    pub fn doubling(&self, other: &Self) -> bool {
        let doubling_name = self.name == other.name;
        let doubling_data = self.data.doubling(&other.data);
        let doubling_lifetime = self.lifetime.doubling(&other.lifetime);
        (doubling_name || doubling_data) && doubling_lifetime
    }
}

/// 関数を表す構造体
#[derive(Clone, Debug, PartialEq)]
pub struct Function {
    name: String,
    arguments: Vec<Rc<RefCell<Variable>>>,
    data: Data,
}

/// 数値リテラルを表すSyntaxNode
pub struct NumberLiteral {
    value: String,
    offset: Offset,
}

impl NumberLiteral {
    pub fn new(value: String, offset: Offset) -> Self {
        NumberLiteral {
            value: value,
            offset: offset,
        }
    }
}

impl SyntaxNode for NumberLiteral {
    fn offset(&self) -> Offset {
        self.offset
    }

    fn as_data(&self, state: &State) -> Data {
        Data::Some {
            r#type: "i32".to_string(),
            storage: vec![Register::Eax],
        }
    }

    fn look_ahead(&self, state: &mut State) -> SResult<()> {
        for i in &mut state.variable_list {
            let mut i_variable = i.borrow_mut();
            if i_variable.lifetime.alive(self.offset) {
                let self_data = Data::Some {
                    r#type: "i32".to_string(),
                    storage: vec![Register::Eax],
                };
                if i_variable.data.doubling(&self_data) {
                    i_variable.lifetime.end = Some(self.offset);
                }
            }
        }
        Ok(())
    }

    fn compile(&self, state: &mut State) -> SResult<()> {
        state.assembly += "mov rax ";
        state.assembly += &self.value;
        state.assembly += "\n";
        Ok(())
    }
}

/// 変数定義宣言を行うSyntaxNode
pub struct VariableDeclaration {
    variable: Rc<RefCell<Variable>>,
    expr: Box<dyn SyntaxNode>,
    offset: Offset,
}

impl VariableDeclaration {
    pub fn new(variable: Variable, expr: Box<dyn SyntaxNode>, offset: Offset) -> Self {
        VariableDeclaration {
            variable: Rc::new(RefCell::new(variable)),
            expr: expr,
            offset: offset,
        }
    }
}

impl SyntaxNode for VariableDeclaration {
    fn offset(&self) -> Offset {
        self.offset
    }

    fn as_data(&self, state: &State) -> Data {
        Data::None
    }

    fn look_ahead(&self, state: &mut State) -> SResult<()> {
        self.expr.look_ahead(state)?;

        for i in &mut state.variable_list {
            let mut i_variable = i.borrow_mut();
            let mut self_variable = self.variable.borrow_mut();
            if i_variable.name == self_variable.name {
                if i_variable.lifetime.doubling(&self_variable.lifetime) {
                    i_variable.lifetime.end = Some(self_variable.lifetime.start);
                }
            }
        }

        state.variable_list.push(self.variable.clone());
        Ok(())
    }

    fn compile(&self, state: &mut State) -> SResult<()> {
        let expr_data = self.expr.as_data(state);
        let self_variable = self.variable.borrow();
        if expr_data == self_variable.data {
            self.expr.compile(state)?;
            Ok(())
        } else {
            Err(format!("Mismatching data"))
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

    fn as_data(&self, state: &State) -> Data {
        Data::None
    }

    fn look_ahead(&self, state: &mut State) -> SResult<()> {
        self.expr.look_ahead(state)
    }

    fn compile(&self, state: &mut State) -> SResult<()> {
        let mut error_flag = true;

        for i in &state.variable_list {
            let i_variable = i.borrow();
            if i_variable.name == self.name && i_variable.lifetime.alive(self.offset) {
                if i_variable.data == self.expr.as_data(state) {
                    if i_variable.mutable {
                        error_flag = false;
                        break;
                    } else {
                        return Err(format!("Variable \"{}\" is not mutable.", i_variable.name));
                    }
                } else {
                    return Err(format!("Missing data type."));
                }
            }
        }
        if error_flag {
            Err(format!("Variable \"{}\" is not defined.", self.name))
        } else {
            self.expr.compile(state)
        }
    }
}

pub struct FunctionDeclaration {
    function: Rc<RefCell<Function>>,
    expr: Vec<Box<dyn SyntaxNode>>,
    offset: Offset,
}

impl SyntaxNode for FunctionDeclaration {
    fn offset(&self) -> Offset {
        self.offset
    }

    fn as_data(&self, state: &State) -> Data {
        self.function.borrow().data.clone()
    }

    fn look_ahead(&self, state: &mut State) -> SResult<()> {
        for i in &state.function_list {
            let self_name = &self.function.borrow().name;
            let i_name = &i.borrow().name;
            if self_name == i_name {
                return Err(format!(
                    "function \"{}\" is defined multiple times.",
                    self_name
                ));
            }
        }

        state.function_list.push(self.function.clone());
        Ok(())
    }

    fn compile(&self, state: &mut State) -> SResult<()> {
        let mut child_state = State {
            variable_list: self.function.borrow().arguments.clone(),
            function_list: state.function_list.clone(),
            type_list: state.type_list.clone(),
            assembly: String::new(),
        };

        for i in &self.expr {
            i.look_ahead(&mut child_state)?;
        }
        for i in &self.expr {
            i.compile(&mut child_state)?;
        }

        state.assembly += "\n";
        state.assembly += &self.function.borrow().name;
        state.assembly += ":\n";
        state.assembly += &child_state.assembly;

        Ok(())
    }
}

pub struct CallingFunctionExpr {
    name: String,
    arguments: Vec<Box<dyn SyntaxNode>>,
}

use asm::assembler::register::Register;
use std::{cell::Ref, cell::RefCell, cmp::min, rc::Rc};
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

    pub fn add_variable(&mut self, variable: Rc<RefCell<Variable>>) {
        for i in &mut self.variable_list {
            let mut i_borrow_mut = i.borrow_mut();
            let mut variable_borrow_mut = variable.borrow_mut();

            if i_borrow_mut.doubling(&*variable_borrow_mut) {
                let i_lifetime = &mut i_borrow_mut.lifetime;
                let variable_lifetime = &mut variable_borrow_mut.lifetime;

                if i_lifetime.start() <= variable_lifetime.start() {
                    i_lifetime.set_end(variable_lifetime.start());
                } else {
                    variable_lifetime.set_end(i_lifetime.start());
                }
            }
        }

        self.variable_list.push(variable);
    }

    pub fn get_variable(&self, name: &str, offset: Offset) -> Option<Ref<Variable>> {
        for i in &self.variable_list {
            let i_borrow = i.borrow();

            if i_borrow.name == name && i_borrow.lifetime.alive(offset) {
                return Some(i_borrow);
            }
        }
        None
    }

    pub fn add_data(&mut self, data: &Data, offset: Offset) {
        if let Data::Some {
            r#type: r#type,
            storage: storage,
        } = data
        {
            for i in &mut self.variable_list {
                let mut i_borrow_mut = i.borrow_mut();
                if i_borrow_mut.lifetime.alive(offset) {
                    i_borrow_mut.lifetime.set_end(offset);
                }
            }
        }
    }

    pub fn add_function(&self, function: Rc<RefCell<Function>>) {
        todo!()
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
    fn as_data(&self, state: &State) -> SResult<Data>;

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

    fn as_data(&self, state: &State) -> SResult<Data> {
        Ok(Data::Some {
            r#type: "i32".to_string(),
            storage: vec![Register::Eax],
        })
    }

    fn look_ahead(&self, state: &mut State) -> SResult<()> {
        let data = self.as_data(state)?;
        state.add_data(&data, self.offset());
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

    fn as_data(&self, state: &State) -> SResult<Data> {
        Ok(Data::None)
    }

    fn look_ahead(&self, state: &mut State) -> SResult<()> {
        self.expr.look_ahead(state)?;
        state.add_variable(self.variable.clone());
        Ok(())
    }

    fn compile(&self, state: &mut State) -> SResult<()> {
        let expr_data = self.expr.as_data(state)?;
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

impl VariableAssignment {
    pub fn new(name: String, expr: Box<dyn SyntaxNode>, offset: Offset) -> Self {
        VariableAssignment {
            name: name,
            expr: expr,
            offset: offset,
        }
    }
}

impl SyntaxNode for VariableAssignment {
    fn offset(&self) -> Offset {
        self.offset
    }

    fn as_data(&self, state: &State) -> SResult<Data> {
        Ok(Data::None)
    }

    fn look_ahead(&self, state: &mut State) -> SResult<()> {
        self.expr.look_ahead(state)
    }

    fn compile(&self, state: &mut State) -> SResult<()> {
        if let Some(variable) = state.get_variable(&self.name, self.offset()) {
            if !variable.mutable {
                return Err(format!("variable \"{}\" is not mutable.", variable.name));
            } else if variable.data != self.expr.as_data(state)? {
                return Err(format!("mismatching data type."));
            }
        } else {
            return Err(format!("variable \"{}\" is not defined.", self.name));
        }
        self.expr.compile(state)?;
        Ok(())
    }
}

pub struct VariableReference {
    name: String,
    offset: Offset,
}

impl VariableReference {
    pub fn new(name: String, offset: Offset) -> Self {
        VariableReference {
            name: name,
            offset: offset,
        }
    }
}

impl SyntaxNode for VariableReference {
    fn offset(&self) -> Offset {
        self.offset
    }

    fn as_data(&self, state: &State) -> SResult<Data> {
        if let Some(variable) = state.get_variable(&self.name, self.offset) {
            Ok(variable.data.clone())
        } else {
            Err(format!("variable \"{}\" is not defined.", self.name))
        }
    }

    fn look_ahead(&self, state: &mut State) -> SResult<()> {
        Ok(())
    }

    fn compile(&self, state: &mut State) -> SResult<()> {
        if let Some(variable) = state.get_variable(&self.name, self.offset) {
            Ok(())
        } else {
            Err(format!("variable \"{}\" is not defined.", self.name))
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

    fn as_data(&self, state: &State) -> SResult<Data> {
        Ok(Data::None)
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

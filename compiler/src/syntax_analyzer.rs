use asm::assembler::register::Register;
use std::{
    cell::Ref,
    cell::RefCell,
    cmp::min,
    fmt::Debug,
    rc::{Rc, Weak},
    str::FromStr,
};
use util::{ErrorMessage, Offset, SResult, parser, parser::Parser};

pub mod syntax_node;

/// 構文ツリーの要素のためのトレイト
pub trait SyntaxNode: Debug {
    fn look_ahead(&self, state: Rc<dyn CompilerState>);
    fn data(&self, state: Rc<dyn CompilerState>) -> Option<Data>;
    fn compile(&self, state: Rc<dyn CompilerState>);
}

/// コンパイル中に情報を記録するためのデータ型のトレイト
pub trait CompilerState {
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
}

/// 構文ツリーを表現する構造体
pub struct SyntaxTree {
    tree: Vec<Box<dyn SyntaxNode>>,
    state: Rc<dyn CompilerState>,
}

/// グローバルなCompilerState
#[derive(Debug)]
pub struct GlobalState {
    function_list: RefCell<Vec<Function>>,
    type_list: RefCell<Vec<Type>>,
    assembly: RefCell<String>,
    error: RefCell<Vec<(Offset, String)>>,
}

/// プロシージャ用のCompilerState
pub struct ProcState {
    parent: Weak<dyn CompilerState>,
    object_list: RefCell<Vec<Object>>,
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
}

/// 関数を表す構造体
#[derive(Clone, Debug, PartialEq)]
pub struct Function {
    name: String,
    arguments: Vec<Object>,
    data: Option<Data>,
}

impl GlobalState {
    pub fn new() -> Self {
        GlobalState {
            function_list: RefCell::new(Vec::new()),
            type_list: RefCell::new(Type::primitive_types()),
            assembly: RefCell::new(String::new()),
            error: RefCell::new(Vec::new()),
        }
    }
}

impl CompilerState for GlobalState {
    fn child_for_proc(self: Rc<Self>) -> SResult<Rc<dyn CompilerState>> {
        let proc_state = ProcState {
            parent: Rc::downgrade(&self) as Weak<dyn CompilerState>,
            object_list: RefCell::new(Vec::new()),
        };
        Ok(Rc::new(proc_state))
    }

    fn add_function(self: Rc<Self>, function: Function) -> SResult<()> {
        if self.clone().get_function(&function.name).is_none() {
            self.function_list.borrow_mut().push(function);
            Ok(())
        } else {
            Err(format!(
                "function \"{}\" is defined multiple times",
                &function.name
            ))
        }
    }

    fn get_function(self: Rc<Self>, name: &str) -> Option<Function> {
        for i in &*self.function_list.borrow() {
            if &i.name == name {
                return Some(i.clone());
            }
        }
        None
    }

    fn add_type(self: Rc<Self>, r#type: Type) -> SResult<()> {
        if self.clone().get_type(&r#type.name).is_none() {
            self.type_list.borrow_mut().push(r#type);
            Ok(())
        } else {
            Err(format!(
                "type \"{}\" is defined multiple times",
                &r#type.name
            ))
        }
    }

    fn get_type(self: Rc<Self>, name: &str) -> Option<Type> {
        for i in &*self.type_list.borrow() {
            if &i.name == name {
                return Some(i.clone());
            }
        }
        None
    }

    fn clean_object(self: Rc<Self>) {
        // do nothing
    }

    fn add_object(self: Rc<Self>, object: Object) -> SResult<()> {
        Err("any object can't exist here".to_string())
    }

    fn get_object_by_name(self: Rc<Self>, name: &str) -> Option<Object> {
        None
    }

    fn get_object_by_register(self: Rc<Self>, register: Register) -> Option<Object> {
        None
    }

    fn map_object_by_name(
        self: Rc<Self>,
        name: &str,
        p: &dyn Fn(Option<&mut Object>) -> SResult<()>,
    ) -> SResult<()> {
        p(None)
    }

    fn map_object_by_register(
        self: Rc<Self>,
        register: Register,
        p: &dyn Fn(Option<&mut Object>) -> SResult<()>,
    ) -> SResult<()> {
        p(None)
    }

    fn drop_object_by_name(self: Rc<Self>, name: &str) {
        // do nothing
    }

    fn drop_object_by_register(self: Rc<Self>, register: Register) {
        // do nothing
    }

    fn add_asm(self: Rc<Self>, code: &str) {
        *self.assembly.borrow_mut() += code;
    }

    fn add_error(self: Rc<Self>, offset: Offset, msg: String) {
        self.error.borrow_mut().push((offset, msg))
    }
}

impl CompilerState for ProcState {
    fn child_for_proc(self: Rc<Self>) -> SResult<Rc<dyn CompilerState>> {
        let proc_state = ProcState {
            parent: Rc::downgrade(&self) as Weak<dyn CompilerState>,
            object_list: RefCell::new(Vec::new()),
        };
        Ok(Rc::new(proc_state))
    }

    fn add_function(self: Rc<Self>, function: Function) -> SResult<()> {
        Err("functions cannot defined here".to_string())
    }
    fn get_function(self: Rc<Self>, name: &str) -> Option<Function> {
        self.parent
            .upgrade()
            .expect("internal error")
            .get_function(name)
    }

    fn add_type(self: Rc<Self>, r#type: Type) -> SResult<()> {
        Err("types cannot defined here".to_string())
    }
    fn get_type(self: Rc<Self>, name: &str) -> Option<Type> {
        self.parent
            .upgrade()
            .expect("internal error")
            .get_type(name)
    }

    fn clean_object(self: Rc<Self>) {
        *self.object_list.borrow_mut() = Vec::new();
        self.parent
            .upgrade()
            .expect("internal error")
            .clean_object();
    }
    fn add_object(self: Rc<Self>, object: Object) -> SResult<()> {
        if let Some(ref name) = object.name {
            self.clone().drop_object_by_name(name);
        }
        self.clone().drop_object_by_register(object.data.register);

        let Some(r#type) = self.clone().get_type(&object.data.r#type) else {
            return Err(format!("type \"{}\" is undefined", object.data.r#type));
        };
        if r#type.avaiable_registers.contains(&object.data.register) {
            return Err(format!(
                "register \"{}\" is unavaiable for type \"{}\"",
                object.data.register, &object.data.r#type
            ));
        }
        self.object_list.borrow_mut().push(object);
        Ok(())
    }
    fn get_object_by_name(self: Rc<Self>, name: &str) -> Option<Object> {
        for i in &*self.object_list.borrow() {
            if let Some(ref i_name) = i.name {
                if i_name == name {
                    return Some(i.clone());
                }
            }
        }
        self.parent
            .upgrade()
            .expect("internal error")
            .get_object_by_name(name)
    }
    fn get_object_by_register(self: Rc<Self>, register: Register) -> Option<Object> {
        for i in &*self.object_list.borrow() {
            if i.data.register == register {
                return Some(i.clone());
            }
        }
        self.parent
            .upgrade()
            .expect("internal error")
            .get_object_by_register(register)
    }

    fn map_object_by_name(
        self: Rc<Self>,
        name: &str,
        p: &dyn Fn(Option<&mut Object>) -> SResult<()>,
    ) -> SResult<()> {
        let mut object_mut: Option<&mut Object> = None;
        for i in &mut *self.object_list.borrow_mut() {
            if let Some(ref i_name) = i.name {
                if i_name == name {
                    return p(Some(i));
                }
            }
        }
        self.parent
            .upgrade()
            .expect("internal erorr")
            .map_object_by_name(name, p)
    }
    fn map_object_by_register(
        self: Rc<Self>,
        register: Register,
        p: &dyn Fn(Option<&mut Object>) -> SResult<()>,
    ) -> SResult<()> {
        for i in &mut *self.object_list.borrow_mut() {
            if i.data.register == register {
                return p(Some(i));
            }
        }
        self.parent
            .upgrade()
            .expect("internal error")
            .map_object_by_register(register, p)
    }
    fn drop_object_by_name(self: Rc<Self>, name: &str) {
        let mut object_list = self.object_list.borrow_mut();
        for i in 0..object_list.len() {
            if let Some(ref object_name) = object_list[i].name {
                if object_name == name {
                    object_list.remove(i);
                    break;
                }
            }
        }
        self.parent
            .upgrade()
            .expect("internal error")
            .drop_object_by_name(name);
    }
    fn drop_object_by_register(self: Rc<Self>, register: Register) {
        let mut object_list = self.object_list.borrow_mut();
        for i in 0..object_list.len() {
            if object_list[i].data.register == register {
                object_list.remove(i);
            }
        }
        self.parent
            .upgrade()
            .expect("internal error")
            .drop_object_by_register(register);
    }

    fn add_asm(self: Rc<Self>, code: &str) {
        self.parent.upgrade().expect("internal error").add_asm(code);
    }

    fn add_error(self: Rc<Self>, offset: Offset, msg: String) {
        self.parent
            .upgrade()
            .expect("internal error")
            .add_error(offset, msg);
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

    /// パース
    pub fn parse(p: &mut Parser<'_>) -> Option<Self> {
        let mut p_copy = *p;
        let a = Self::parse_(&mut p_copy)?;
        *p = p_copy;
        Some(a)
    }

    fn parse_(p: &mut Parser<'_>) -> Option<Self> {
        // $type @ $register
        let r#type = p.parse_identifier()?.1;
        p.parse_symbol(":")?;
        let register_string = p.parse_identifier()?.1;
        let Ok(register) = register_string.parse() else {
            return None;
        };
        Some(Data {
            r#type: r#type.to_string(),
            register: register,
        })
    }
}

impl Object {
    /// 存在が重複しているか判定する関数
    pub fn doubling(&self, other: &Self) -> bool {
        self.data.doubling(&other.data)
    }

    /// パース
    pub fn parse(p: &mut Parser<'_>) -> Option<Self> {
        let mut p_copy = *p;
        let a = Self::parse_(&mut p_copy)?;
        *p = p_copy;
        Some(a)
    }

    fn parse_(p: &mut Parser<'_>) -> Option<Self> {
        let mutable = p.parse_keyword("mut").is_some();
        let name = p.parse_identifier()?.1;
        p.parse_keyword(":")?;
        let data = Data::parse(p)?;
        Some(Object {
            name: Some(name.to_string()),
            mutable: mutable,
            data: data,
        })
    }
}

impl Type {
    /// プリミティブなデータ型のリストを返す
    pub fn primitive_types() -> Vec<Self> {
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
        vec![u32, i32]
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

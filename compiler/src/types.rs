use asm::assembler::register::Register;
use util::parser::Parser;

/// データ型を表す構造体
#[derive(Clone, Debug, PartialEq)]
pub struct Type {
    pub name: String,
    pub avaiable_registers: Vec<Register>,
    pub copy: bool,
}

/// データを表す構造体
#[derive(Clone, Debug, PartialEq)]
pub struct Data {
    pub r#type: String,
    pub register: Register,
}

/// オブジェクトを表す構造体
#[derive(Clone, Debug, PartialEq)]
pub struct Object {
    pub name: Option<String>,
    pub mutable: bool,
    pub data: Data,
}

/// 関数を表す構造体
#[derive(Clone, Debug, PartialEq)]
pub struct Function {
    pub name: String,
    pub arguments: Vec<Object>,
    pub data: Option<Data>,
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
        let r#type = p.parse_identifier()?;
        p.parse_symbol("@")?;
        let register_string = p.parse_identifier()?;
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
        let name = p.parse_identifier()?;
        p.parse_symbol(":")?;
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

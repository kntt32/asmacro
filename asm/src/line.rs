use instruction::{Instruction, OperandType};
use pseudo::Pseudo;
use crate::object::Object;

/// Assembly line information
#[derive(Clone, Copy, Debug)]
pub enum Line<'a> {
    /// 空の行
    None,
    /// ラベル
    Label(&'a str),
    /// 疑似命令
    Pseudo(&'a str),
    /// CPU命令
    Instruction(&'a str),
    /// 不明な表現
    Unknown(&'a str),
}

impl<'a> Line<'a> {
    pub fn encode(self, object: &mut Object, pseudo_commands: &[Pseudo]) -> Result<(), String> {
        match self {
            Line::None => Ok(()),
            Line::Label(_) => self.label_encode(object),
            Line::Pseudo(_) => self.pseudo_encode(object, &pseudo_commands),
            Line::Instruction(_) => self.instruction_encode(object),
            Line::Unknown(s) => Err(format!("unknown expression \"{}\"", s)),
        }
    }

    /// Split instruction and return mnemonic and operands
    /// (mnemonic, operand1, operand2)
    pub fn split_instruction(self) -> Option<(&'a str, Vec<&'a str>)> {
        if let Line::Instruction(s) = self {
            let mut s_split = s.trim().split(' ');

            let mnemonic = s_split.next().expect("unknown error");
            let operands = s_split.collect();
            Some((mnemonic, operands))
        } else {
            None
        }
    }

    // Is valid instruction
    pub fn is_valid_instruction(self, instructions: &[Instruction]) -> bool {
        match self {
            Line::Instruction(_) => self.get_instruction(instructions).is_some(),
            _ => false,
        }
    }

    /// Get mneonic
    pub fn mnemonic(self) -> Option<&'a str> {
        if let Line::Instruction(s) = self {
            let mut s_split = s.trim().split(' ');
            let mnemonic = s_split.next().expect("unknown error");
            Some(mnemonic)
        } else {
            None
        }
    }

    /// Get operands
    pub fn operands(self) -> Option<Vec<&'a str>> {
        Some(self.split_instruction()?.1)
    }

    /// Get instruction information
    pub fn get_instruction(self, instructions: &[Instruction]) -> Option<&Instruction> {
        for i in instructions {
            if i.match_with(&self) {
                return Some(&i);
            }
        }
        None
    }

    fn get_operand_by_type(
        self,
        instruction: &Instruction,
        operand_type: OperandType,
    ) -> Option<&'a str> {
        let operand_index = instruction
            .expression()
            .get_operand_index_by_type(operand_type)?;
        Some(self.operands()?[operand_index])
    }
}

/// 疑似命令関連のモジュール
/// # サポートする疑似命令
/// - .db8  : 8bit数値列書き込み
/// - .db16 : 16bit数値列書き込み
/// - .db32 : 32bit数値列書き込み
/// - .db64 : 64bit数値列書き込み
/// - .utf8 : utf8文字列書き込み
/// - .align16 : 16バイトでアライメント
mod pseudo {
    use super::{Line, Object};

    #[derive(Clone, Copy, Debug)]
    pub struct Pseudo<'a> {
        name: &'a str,
        encode: fn(&str, &mut Object) -> Result<(), String>,
    }

    impl<'a> Line<'a> {
        pub fn pseudo_encode(self, object: &mut Object, pseudo_commands: &[Pseudo<'_>]) -> Result<(), String> {
            if let Line::Pseudo(s) = self {
                let name = pseudo_name(s);
                let arg = pseudo_arg(s);
                if let Some(p) = get_pseudo(name, pseudo_commands) {
                    (p.encode)(arg, object)
                } else {
                    panic!("internal error: undefined pseudo");
                }
            } else {
                panic!("internal error: input must be Line::Pseudo");
            }
        }
    }

    fn get_pseudo<'a>(name: &str, pseudo_commands: &'a [Pseudo<'a>]) -> Option<&'a Pseudo<'a>> {
        pseudo_commands.iter().find(|&x| name == x.name)
    }

    fn pseudo_name(mut line: &str) -> &str {
        const error_message: &str = "internal error: input must be pseudo instruction";
        line = line.trim();
        if !line.starts_with('.') {
            panic!("{}", error_message);
        }
        line = &line['.'.len_utf8()..line.len()];
        line.split(' ').next().expect(error_message)
    }

    fn pseudo_arg(line: &str) -> &str {
        if let Some((_, right)) = line.trim().split_once(' ') {
            right
        } else {
            &""
        }
    }
}

/// Label関連のモジュール
pub mod label {
    use super::{Line, Object};
    
    impl Line<'_> {
        pub fn label_len(self) -> Result<usize, String> {
            if let Line::Label(_) = self {
                Ok(0)
            }else {
                panic!("internal error");
            }
        }

        pub fn label_encode(self, object: &mut Object) -> Result<(), String> {
            if let Line::Label(s) = self {
                let label = Label { name: s.to_string(), value: object.code_len(), public: false };
                object.add_label(label);
                Ok(())
            }else {
                panic!("internal error");
            }
        }
    }

    #[derive(Clone, Debug)]
    pub struct Label {
        name: String,
        value: usize,
        public: bool,
    }

    #[derive(Clone, Debug)]
    pub enum Location {
        Disp32{label: String, offset: usize},
        Rel8{label: String, offset: usize},
        Rel16{label: String, offset: usize},
        Rel32{label: String, offset: usize},
    }
}

/// Instruction関連のモジュール
pub mod instruction {
    use super::{Line, Object};
    use crate::{
        functions::{is_keyword, parse_rm, Disp},
        register::Register,
    };
    pub use instruction_database::INSTRUCTION_LIST;
    use std::{
        cmp::{Eq, Ord, Ordering, PartialEq, PartialOrd},
        str::FromStr,
    };
    use util::functions::{stoi, stoi_hex_no_prefix};
    mod instruction_database;
    pub use encoding_rule::{EncodingRule, ImmRule, ModRmRule, OpecodeRegisterRule};
    pub use expression::{Expression, OperandType};
    pub use operand_size::OperandSize;

    impl<'a> Line<'a> {
        pub fn instruction_encode(self, object: &mut Object) -> Result<(), String> {
            if let Line::Instruction(s) = self {
                todo!()
            }else {
                panic!("internal error")
            }
        }
    }

    ///命令の詳細を記述する構造体
    #[derive(Clone, Debug)]
    pub struct Instruction {
        encoding: EncodingRule,
        expression: Expression,
    }

    impl Instruction {
        /// 命令がLine<'_>にマッチするか判定
        pub fn match_with(&self, line: &Line) -> bool {
            self.expression.match_with(line)
        }

        /// エンコードルール取得
        pub const fn encoding(&self) -> &EncodingRule {
            &self.encoding
        }

        /// アセンブラでの表現ルール取得
        pub const fn expression(&self) -> &Expression {
            &self.expression
        }
    }

    impl FromStr for Instruction {
        type Err = String;

        // ADC reg/mem64 reg64 , 11 /r
        fn from_str(s: &str) -> Result<Self, Self::Err> {
            if let Some((expression_str, encoding_str)) = s.split_once(',') {
                let expression = expression_str.trim().parse()?;
                let encoding = encoding_str.trim().parse()?;

                Ok(Instruction {
                    encoding: encoding,
                    expression: expression,
                })
            } else {
                Err("unimplemented".to_string())
            }
        }
    }

    /// エンコードルールに関するモジュール
    pub mod encoding_rule {
        use super::*;
        /// エンコードルール
        #[derive(Clone, Debug)]
        pub struct EncodingRule {
            opecode: Vec<u8>,
            modrm: Option<ModRmRule>,
            imm: Option<ImmRule>,
            opecode_register: Option<OpecodeRegisterRule>,
            default_operand_size: OperandSize,
        }

        impl EncodingRule {
            /// オペコード取得
            pub fn opecode(&self) -> &[u8] {
                &self.opecode
            }

            /// レジスタのオペコードへの埋め込みルール取得
            pub fn opecode_register_rule(&self) -> Option<OpecodeRegisterRule> {
                self.opecode_register
            }

            /// ModRmのエンコードルール取得
            pub fn modrm_rule(&self) -> Option<ModRmRule> {
                self.modrm
            }

            /// 即値のエンコードルール取得
            pub fn imm_rule(&self) -> Option<ImmRule> {
                self.imm
            }

            /// デフォルトオペランドサイズ取得
            pub fn default_operand_size(&self) -> OperandSize {
                self.default_operand_size
            }

            fn parse_opecode_rule(target: &str) -> Result<u8, String> {
                static ERROR_MESSAGE: &str = "invalid opecode";
                let Some(v) = stoi_hex_no_prefix(target) else {
                    return Err(ERROR_MESSAGE.to_string());
                };
                if 0 <= v && v <= u8::MAX as i128 {
                    Ok(v as u8)
                } else {
                    Err(ERROR_MESSAGE.to_string())
                }
            }
        }

        impl FromStr for EncodingRule {
            type Err = String;

            // ADC reg/mem64 reg64 , 11 /r
            fn from_str(s: &str) -> Result<Self, Self::Err> {
                static ERROR_MESSAGE: &str = "invalid encoding rule";

                let splited_s: Vec<&str> = s.split(' ').collect();
                let mut encoding = EncodingRule {
                    opecode: Vec::new(),
                    modrm: None,
                    imm: None,
                    opecode_register: None,
                    default_operand_size: OperandSize::Od,
                };

                for i in 0..splited_s.len() {
                    let target = splited_s[i].trim();
                    if let Ok(v) = Self::parse_opecode_rule(target) {
                        encoding.opecode.push(v);
                    } else if let Ok(v) = target.parse::<ModRmRule>() {
                        encoding.modrm = Some(v);
                    } else if let Ok(v) = target.parse::<ImmRule>() {
                        encoding.imm = Some(v);
                    } else if let Ok(v) = target.parse::<OpecodeRegisterRule>() {
                        encoding.opecode_register = Some(v);
                    } else if let Ok(v) = target.parse::<OperandSize>() {
                        encoding.default_operand_size = v;
                    } else {
                        return Err(ERROR_MESSAGE.to_string());
                    }
                }

                Ok(encoding)
            }
        }

        /// ModRmエンコードルール
        #[derive(Clone, Copy, Debug, PartialEq)]
        pub enum ModRmRule {
            R,
            Dight(u8),
        }

        impl FromStr for ModRmRule {
            type Err = String;

            fn from_str(s: &str) -> Result<ModRmRule, Self::Err> {
                static ERROR_MESSAGE: &str = "invalid modrm rule";
                let mut target = s.trim();
                if !target.starts_with('/') {
                    return Err(ERROR_MESSAGE.to_string());
                }
                target = &target['/'.len_utf8()..];
                if target == "r" {
                    Ok(ModRmRule::R)
                } else {
                    let Some(v) = stoi(target) else {
                        return Err(ERROR_MESSAGE.to_string());
                    };
                    if 0 <= v && v <= 7 {
                        Ok(ModRmRule::Dight(v as u8))
                    } else {
                        Err(ERROR_MESSAGE.to_string())
                    }
                }
            }
        }

        /// 即値エンコードルール
        #[derive(Clone, Copy, Debug, PartialEq)]
        pub enum ImmRule {
            Ib,
            Iw,
            Id,
            Iq,
        }

        impl FromStr for ImmRule {
            type Err = String;

            fn from_str(s: &str) -> Result<ImmRule, Self::Err> {
                static ERROR_MESSAGE: &str = "invalid immrule";
                match s {
                    "ib" => Ok(Self::Ib),
                    "iw" => Ok(Self::Iw),
                    "id" => Ok(Self::Id),
                    "iq" => Ok(Self::Iq),
                    _ => Err(ERROR_MESSAGE.to_string()),
                }
            }
        }

        impl ImmRule {
            /// 対応するOperandTypeの値を取得
            pub fn operand_type(self) -> OperandType {
                match self {
                    ImmRule::Ib => OperandType::Imm8,
                    ImmRule::Iw => OperandType::Imm16,
                    ImmRule::Id => OperandType::Imm32,
                    ImmRule::Iq => OperandType::Imm64,
                }
            }
        }

        /// オペコード埋め込みレジスタのエンコードルール
        #[derive(Clone, Copy, Debug, PartialEq)]
        pub enum OpecodeRegisterRule {
            Rb,
            Rw,
            Rd,
            Rq,
        }

        impl FromStr for OpecodeRegisterRule {
            type Err = String;

            fn from_str(s: &str) -> Result<OpecodeRegisterRule, Self::Err> {
                static ERROR_MESSAGE: &str = "invalid opecode embbed register rule";
                match s {
                    "+rb" => Ok(Self::Rb),
                    "+rw" => Ok(Self::Rw),
                    "+rd" => Ok(Self::Rd),
                    "+rq" => Ok(Self::Rq),
                    _ => Err(ERROR_MESSAGE.to_string()),
                }
            }
        }
    }

    /// オペランドサイズ関連モジュール
    pub mod operand_size {
        use super::*;
        /// オペランドサイズ
        #[derive(Clone, Copy, Debug, PartialEq)]
        pub enum OperandSize {
            Ob,
            Ow,
            Od,
            Oq,
        }

        impl OperandSize {
            /// オペランドサイズの値取得
            pub fn value(self) -> usize {
                match self {
                    OperandSize::Ob => 1,
                    OperandSize::Ow => 2,
                    OperandSize::Od => 4,
                    OperandSize::Oq => 8,
                }
            }
        }

        impl FromStr for OperandSize {
            type Err = String;

            fn from_str(s: &str) -> Result<OperandSize, Self::Err> {
                static ERROR_MESSAGE: &str = "invalid operand size";
                match s {
                    "ob" => Ok(Self::Ob),
                    "ow" => Ok(Self::Ow),
                    "od" => Ok(Self::Od),
                    "oq" => Ok(Self::Oq),
                    _ => Err(ERROR_MESSAGE.to_string()),
                }
            }
        }

        impl Eq for OperandSize {}

        impl PartialOrd for OperandSize {
            fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
                let lhs = *self as usize;
                let rhs = *other as usize;
                lhs.partial_cmp(&rhs)
            }
        }

        impl Ord for OperandSize {
            fn cmp(&self, other: &Self) -> Ordering {
                self.partial_cmp(other).expect("unknown error")
            }
        }
    }

    pub mod expression {
        use super::*;

        /// アセンブリコードでの表現ルール取得
        #[derive(Clone, Debug)]
        pub struct Expression {
            /// ニーモニック
            mnemonic: String,
            /// オペランド
            operands: Vec<OperandType>,
        }

        impl FromStr for Expression {
            type Err = String;

            // ADC reg/mem64 reg64
            fn from_str(s: &str) -> Result<Expression, Self::Err> {
                static ERROR_MESSAGE: &str = "invalid expression rule";
                let mut tokens = s.split(' ');
                let mnemonic;
                if let Some(s) = tokens.next() {
                    mnemonic = s.to_ascii_lowercase();
                } else {
                    return Err(ERROR_MESSAGE.to_string());
                }

                let mut operands = Vec::new();
                for t in tokens {
                    operands.push(t.parse()?);
                }

                Ok(Expression {
                    mnemonic: mnemonic,
                    operands: operands,
                })
            }
        }

        impl Expression {
            /// ニーモニック取得
            pub fn mnemonic(&self) -> &str {
                &self.mnemonic
            }

            /// オペランド取得
            pub fn operands(&self) -> &[OperandType] {
                &self.operands
            }

            /// アセンブリコードにマッチするか判定
            pub fn match_with(&self, line: &Line) -> bool {
                line.mnemonic() == Some(self.mnemonic()) && self.operands_match_with(line)
            }

            fn operands_match_with(&self, line: &Line) -> bool {
                let Some(operands) = line.operands() else {
                    return false;
                };
                let operand_types = self.operands();

                if operands.len() == operand_types.len() {
                    for i in 0..operand_types.len() {
                        if !operand_types[i].match_with(operands[i]) {
                            return false;
                        }
                    }
                    true
                } else {
                    false
                }
            }

            /// operand_typeのオペランドが何番目の引数か取得
            pub fn get_operand_index_by_type(&self, operand_type: OperandType) -> Option<usize> {
                for i in 0..self.operands.len() {
                    if self.operands[i] == operand_type {
                        return Some(i);
                    }
                }
                None
            }
        }

        /// オペランドタイプ
        #[derive(Clone, Copy, Debug, PartialEq)]
        pub enum OperandType {
            Al,
            Ax,
            Eax,
            Rax,
            Rel8,
            Rel16,
            Rel32,
            R8,
            R16,
            R32,
            R64,
            Imm8,
            Imm16,
            Imm32,
            Imm64,
            Rm8,
            Rm16,
            Rm32,
            Rm64,
        }

        impl FromStr for OperandType {
            type Err = String;

            fn from_str(s: &str) -> Result<OperandType, Self::Err> {
                static ERROR_MESSAGE: &str = "invalid operand type";
                Ok(match s {
                    "al" => Self::Al,
                    "ax" => Self::Ax,
                    "eax" => Self::Eax,
                    "rax" => Self::Rax,
                    "rel8" => Self::Rel8,
                    "rel16" => Self::Rel16,
                    "rel32" => Self::Rel32,
                    "reg8" => Self::R8,
                    "reg16" => Self::R16,
                    "reg32" => Self::R32,
                    "reg64" => Self::R64,
                    "imm8" => Self::Imm8,
                    "imm16" => Self::Imm16,
                    "imm32" => Self::Imm32,
                    "imm64" => Self::Imm64,
                    "reg/mem8" => Self::Rm8,
                    "reg/mem16" => Self::Rm16,
                    "reg/mem32" => Self::Rm32,
                    "reg/mem64" => Self::Rm64,
                    _ => return Err(ERROR_MESSAGE.to_string()),
                })
            }
        }

        impl OperandType {
            /// オペランドサイズ取得
            pub const fn size(self) -> OperandSize {
                match self {
                    OperandType::Al => OperandSize::Ob,
                    OperandType::Ax => OperandSize::Ow,
                    OperandType::Eax => OperandSize::Od,
                    OperandType::Rax => OperandSize::Oq,
                    OperandType::Rel8 => OperandSize::Ob,
                    OperandType::Rel16 => OperandSize::Ow,
                    OperandType::Rel32 => OperandSize::Od,
                    OperandType::R8 => OperandSize::Ob,
                    OperandType::R16 => OperandSize::Ow,
                    OperandType::R32 => OperandSize::Od,
                    OperandType::R64 => OperandSize::Oq,
                    OperandType::Imm8 => OperandSize::Ob,
                    OperandType::Imm16 => OperandSize::Ow,
                    OperandType::Imm32 => OperandSize::Od,
                    OperandType::Imm64 => OperandSize::Oq,
                    OperandType::Rm8 => OperandSize::Ob,
                    OperandType::Rm16 => OperandSize::Ow,
                    OperandType::Rm32 => OperandSize::Od,
                    OperandType::Rm64 => OperandSize::Oq,
                }
            }

            /// 表現がオペランドタイプにマッチするか判定
            pub fn match_with(self, expr: &str) -> bool {
                match self {
                    OperandType::Al => expr == "al",
                    OperandType::Ax => expr == "ax",
                    OperandType::Eax => expr == "eax",
                    OperandType::Rax => expr == "rax",
                    OperandType::Rel8 => {
                        number_match_with(expr, i8::MIN as i128, i8::MAX as i128)
                            || is_keyword(expr)
                    }
                    OperandType::Rel16 => {
                        number_match_with(expr, i16::MIN as i128, i16::MAX as i128)
                            || is_keyword(expr)
                    }
                    OperandType::Rel32 => {
                        number_match_with(expr, i32::MIN as i128, i32::MAX as i128)
                            || is_keyword(expr)
                    }
                    OperandType::R8 => register_match_with(expr, Register::operand_r8),
                    OperandType::R16 => register_match_with(expr, Register::operand_r16),
                    OperandType::R32 => register_match_with(expr, Register::operand_r32),
                    OperandType::R64 => register_match_with(expr, Register::operand_r64),
                    OperandType::Imm8 => number_match_with(expr, i8::MIN as i128, u8::MAX as i128),
                    OperandType::Imm16 => {
                        number_match_with(expr, i16::MIN as i128, u16::MAX as i128)
                    }
                    OperandType::Imm32 => {
                        number_match_with(expr, i32::MIN as i128, u32::MAX as i128)
                    }
                    OperandType::Imm64 => {
                        number_match_with(expr, i64::MIN as i128, u64::MAX as i128)
                    }
                    OperandType::Rm8 => rm_match_with(
                        expr,
                        Register::operand_r8,
                        i8::MIN as i128,
                        i8::MAX as i128,
                        'b',
                    ),
                    OperandType::Rm16 => rm_match_with(
                        expr,
                        Register::operand_r16,
                        i16::MIN as i128,
                        i16::MAX as i128,
                        'w',
                    ),
                    OperandType::Rm32 => rm_match_with(
                        expr,
                        Register::operand_r32,
                        i32::MIN as i128,
                        i32::MAX as i128,
                        'd',
                    ),
                    OperandType::Rm64 => rm_match_with(
                        expr,
                        Register::operand_r64,
                        i64::MIN as i128,
                        i64::MAX as i128,
                        'q',
                    ),
                }
            }
        }

        fn number_match_with(expr: &str, min: i128, max: i128) -> bool {
            let value = stoi(expr);
            value.is_some()
                && min <= value.expect("unknown error")
                && value.expect("unknown error") <= max
        }

        fn register_match_with(expr: &str, matching: impl Fn(Register) -> bool) -> bool {
            let value = expr.parse::<Register>();
            if let Ok(r) = value {
                matching(r)
            } else {
                false
            }
        }

        fn rm_match_with(
            expr: &str,
            register_matching: impl Fn(Register) -> bool,
            disp_min: i128,
            disp_max: i128,
            address_size_matching: char,
        ) -> bool {
            const fn is_valid_scale(scale: u8) -> bool {
                scale == 1 || scale == 2 || scale == 4 || scale == 8
            }

            if register_match_with(expr, register_matching) {
                true
            } else {
                match parse_rm(expr.trim(), address_size_matching) {
                    Some((disp, base, optional_index)) => {
                        let base_match = base.operand_rm_ref_base() || base == Register::Rip;
                        let index_match = match optional_index {
                            Some((index, scale)) => {
                                index.operand_rm_ref_index() && is_valid_scale(scale)
                            }
                            None => true,
                        };
                        let disp_match = if let Disp::Value(d) = disp {
                            disp_min <= d as i128 && d as i128 <= disp_max
                        } else {
                            true
                        };
                        base_match && index_match && disp_match
                    }
                    None => false,
                }
            }
        }
    }
}

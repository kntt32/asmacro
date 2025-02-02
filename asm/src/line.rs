use crate::object::Object;
use instruction::{Instruction, OperandType};
use label::{Label, Location};
use pseudo::Pseudo;

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
    pub fn encode(
        self,
        object: &mut Object,
        pseudo_commands: &[Pseudo],
        instructions: &[Instruction],
    ) -> Result<(), String> {
        match self {
            Line::None => Ok(()),
            Line::Label(_) => self.label_encode(object),
            Line::Pseudo(_) => self.pseudo_encode(object, pseudo_commands),
            Line::Instruction(_) => self.instruction_encode(object, instructions),
            Line::Unknown(s) => Err(format!("unknown expression \"{}\"", s)),
        }
    }

    /// Split instruction and return mnemonic and operands
    /// (mnemonic, operand1, operand2)
    pub fn split_instruction(self) -> (&'a str, Vec<&'a str>) {
        if let Line::Instruction(s) = self {
            let mut s_split = s.trim().split(' ');

            let mnemonic = s_split.next().expect("internal error");
            let operands = s_split.collect();
            (mnemonic, operands)
        } else {
            panic!("invalid operation");
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
    pub fn mnemonic(self) -> &'a str {
        self.split_instruction().0
    }

    /// Get operands
    pub fn operands(self) -> Vec<&'a str> {
        self.split_instruction().1
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
        Some(self.operands()[operand_index])
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
        pub fn pseudo_encode(
            self,
            object: &mut Object,
            pseudo_commands: &[Pseudo<'_>],
        ) -> Result<(), String> {
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
            } else {
                panic!("internal error");
            }
        }

        pub fn label_encode(self, object: &mut Object) -> Result<(), String> {
            if let Line::Label(s) = self {
                let label = Label {
                    name: s.to_string(),
                    value: object.code_len(),
                    public: false,
                };
                object.add_label(label);
                Ok(())
            } else {
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
    pub struct Location {
        pub label: String,
        pub offset: usize,
        pub size: usize,
        pub rel_base: usize,
    }
}

/// Instruction関連のモジュール
pub mod instruction {
    use super::{Line, Location, Object};
    use crate::{
        functions::{is_keyword, parse_rm, parse_rm_anysize, Disp, Imm},
        register::{Register, RegisterCode},
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

    type SResult<T> = Result<T, String>;

    impl<'a> Line<'a> {
        /// 命令のエンコード
        pub fn instruction_encode(
            self,
            object: &mut Object,
            instructions: &[Instruction],
        ) -> SResult<()> {
            let Some(instruction) = self.get_instruction(instructions) else {
                return Err("unknown instruction".to_string());
            };
            let instruction_base = object.code.len();

            self.push_legacy_prefix(object, instruction)?;
            self.push_rex_prefix(object, instruction)?;
            self.push_opecode(object, instruction)?;
            self.push_modrm(object, instruction)?;
            self.push_sib(object, instruction)?;
            self.push_disp(object, instruction, instruction_base)?;
            self.push_imm(object, instruction)?;
            Ok(())
        }

        fn register_operand(self, instruction: &Instruction) -> Option<&'a str> {
            let register_operand_index = instruction.expression().get_register_operand_index()?;
            Some(self.operands()[register_operand_index])
        }

        fn imm_operand(self, instruction: &Instruction) -> Option<&'a str> {
            let register_operand_index = instruction.expression().get_imm_operand_index()?;
            Some(self.operands()[register_operand_index])
        }

        fn prefix_x66_exist(self, instruction: &Instruction) -> bool {
            let expression = instruction.expression();
            if let Some(operand_size) = expression.get_operand_size() {
                operand_size == OperandSize::Ob
            } else {
                false
            }
        }

        fn push_legacy_prefix(self, object: &mut Object, instruction: &Instruction) -> SResult<()> {
            if self.prefix_x66_exist(instruction) {
                object.code.push(0x66);
            }
            Ok(())
        }

        fn rex_prefix_is_required(self, instruction: &Instruction) -> bool {
            let encoding = instruction.encoding();
            let expression = instruction.expression();
            let default_operand_size = encoding.default_operand_size();
            assert!(OperandSize::Od <= default_operand_size);

            if let Some(operand_size) = expression.get_operand_size() {
                default_operand_size < operand_size
            } else {
                false
            }
        }

        fn rex_w(self, instruction: &Instruction) -> u8 {
            if self.rex_prefix_is_required(instruction) {
                0x1
            } else {
                0x0
            }
        }

        fn rex_r(self, instruction: &Instruction) -> u8 {
            if let Some(Ok((Some(true), _))) = self.opecode_register_code(instruction) {
                0b1
            } else if let Ok((Some(true), _)) = self.modrm_register_code(instruction) {
                0b1
            } else {
                0b0
            }
        }

        fn rex_b(self, instruction: &Instruction) -> u8 {
            if let Some(Ok((Some(true), _))) = self.modrm_base_register_code(instruction) {
                0b1
            } else {
                0b0
            }
        }

        fn rex_x(self, instruction: &Instruction) -> u8 {
            if let Some(Ok((Some(true), _))) = self.modrm_index_register_code(instruction) {
                0b1
            } else {
                0b0
            }
        }

        fn push_rex_prefix(self, object: &mut Object, instruction: &Instruction) -> SResult<()> {
            let rex_w = self.rex_w(instruction);
            let rex_r = self.rex_r(instruction);
            let rex_b = self.rex_b(instruction);
            let rex_x = self.rex_x(instruction);

            if rex_w | rex_r | rex_x | rex_b != 0 {
                let rex_prefix = 0x40 | (rex_w << 3) | (rex_r << 2) | (rex_x << 1) | rex_b;
                object.code.push(rex_prefix);
            }

            Ok(())
        }

        fn opecode_register_code(self, instruction: &Instruction) -> Option<SResult<RegisterCode>> {
            match instruction.encoding().opecode_register_rule() {
                Some(_) => match self.register_operand(instruction)?.parse::<Register>() {
                    Ok(r) => Some(r.register_code_for_opecode_register()),
                    Err(e) => Some(Err(e)),
                },
                None => None,
            }
        }

        fn push_opecode(self, object: &mut Object, instruction: &Instruction) -> SResult<()> {
            let encoding_rule = instruction.encoding();
            let opecode = instruction.encoding().opecode();
            let register_code = if let Some(v) = self.opecode_register_code(instruction) {
                v?.1
            } else {
                0
            };

            for i in 0..opecode.len() {
                object.code.push(opecode[i]);
            }
            let object_code_len = object.code.len();
            object.code[object_code_len - 1] += register_code;
            Ok(())
        }

        fn modrm_register_code(self, instruction: &Instruction) -> SResult<RegisterCode> {
            let encoding_rule = instruction.encoding();

            match encoding_rule.modrm_rule() {
                None => panic!("internal error"),
                Some(ModRmRule::R) => {
                    let register = self
                        .register_operand(instruction)
                        .expect("internal error")
                        .parse::<Register>()
                        .expect("internal error");
                    register.register_code_for_opecode_register()
                }
                Some(ModRmRule::Dight(i)) => Ok((None, i)),
            }
        }

        fn rm_operand(self, instruction: &Instruction) -> &'a str {
            let rm_operand_index = instruction
                .expression()
                .get_rm_operand_index()
                .expect("internal error");
            self.operands()[rm_operand_index]
        }

        fn modrm_rm_register(self, instruction: &Instruction) -> SResult<Register> {
            self.rm_operand(instruction).parse()
        }

        fn modrm_rm_base(self, instruction: &Instruction) -> Option<Register> {
            if let Some((_, base, _)) = parse_rm_anysize(self.rm_operand(instruction)) {
                Some(base)
            } else {
                None
            }
        }

        fn modrm_base_register_code(
            self,
            instruction: &Instruction,
        ) -> Option<SResult<RegisterCode>> {
            Some(if let Ok(r) = self.modrm_rm_register(instruction) {
                r.register_code_for_opecode_register()
            } else {
                self.modrm_rm_base(instruction)?
                    .register_code_for_rm_ref_base()
            })
        }

        fn modrm_rm_index(self, instruction: &Instruction) -> Option<Register> {
            if let Some((_, _, Some((index, _)))) = parse_rm_anysize(self.rm_operand(instruction)) {
                Some(index)
            } else {
                None
            }
        }

        fn modrm_index_register_code(
            self,
            instruction: &Instruction,
        ) -> Option<SResult<RegisterCode>> {
            Some(
                self.modrm_rm_index(instruction)?
                    .register_code_for_rm_ref_index(),
            )
        }

        fn modrm_scale(self, instruction: &Instruction) -> Option<u8> {
            if let Some((_, _, Some((_, scale)))) = parse_rm_anysize(self.rm_operand(instruction)) {
                Some(scale)
            } else {
                None
            }
        }

        fn modrm_scale_code(self, instruction: &Instruction) -> Option<u8> {
            match self.modrm_scale(instruction) {
                Some(1) => Some(0),
                Some(2) => Some(1),
                Some(4) => Some(2),
                Some(8) => Some(3),
                None => None,
                _ => panic!("internal error"),
            }
        }

        fn modrm_disp(self, instruction: &Instruction) -> Option<Disp> {
            if let Some((disp, _, _)) = parse_rm_anysize(self.rm_operand(instruction)) {
                Some(disp)
            } else {
                None
            }
        }

        fn disp_len(self, instruction: &Instruction) -> usize {
            if self.modrm_exist(instruction) {
                match self.modrm_mode(instruction).expect("internal error") {
                    0b00 | 0b11 => 0,
                    0b01 => 1,
                    0b10 => 4,
                    _ => panic!("internal error"),
                }
            } else {
                0
            }
        }

        fn modrm_mode(self, instruction: &Instruction) -> Option<u8> {
            if self.modrm_exist(instruction) {
                let modrm_rm_base = self.modrm_rm_base(instruction);
                Some(match modrm_rm_base {
                    Some(Register::Rip) => 0b00,
                    Some(r) => {
                        let modrm_disp = self.modrm_disp(instruction);
                        let disp_is_8bit;
                        let disp_is_exist;

                        if let Some(d) = modrm_disp {
                            if let Disp::Value(v) = d {
                                disp_is_8bit = i8::MIN as i32 <= v && v <= i8::MAX as i32;
                                disp_is_exist = v != 0
                                    || r == Register::Rbp
                                    || r == Register::R13
                                    || (self.sib_exist(instruction)
                                        && (r == Register::Rbp || r == Register::R13));
                            } else {
                                disp_is_8bit = false;
                                disp_is_exist = true;
                            }
                        } else {
                            disp_is_8bit = false;
                            disp_is_exist = false;
                        }

                        if disp_is_exist {
                            if disp_is_8bit {
                                0b01
                            } else {
                                0b10
                            }
                        } else {
                            0b00
                        }
                    }
                    None => 0b11,
                })
            } else {
                None
            }
        }

        fn modrm_exist(self, instruction: &Instruction) -> bool {
            instruction.encoding().modrm_rule().is_some()
        }

        fn push_modrm(self, object: &mut Object, instruction: &Instruction) -> SResult<()> {
            if self.modrm_exist(instruction) {
                let Some(mode) = self.modrm_mode(instruction) else {
                    return Err("internal error".to_string());
                };
                let (_, reg) = self.modrm_register_code(instruction)?;
                let rm_base = if self.sib_exist(instruction) {
                    0b100
                } else {
                    self.modrm_base_register_code(instruction)
                        .expect("internal error")?
                        .1
                };
                let modrm = (mode << 6) | (reg << 3) | rm_base;
                object.code.push(modrm);
            }
            Ok(())
        }

        fn sib_exist(self, instruction: &Instruction) -> bool {
            let modrm_rm_base = self.modrm_rm_base(instruction);
            let modrm_rm_index = self.modrm_rm_index(instruction);
            modrm_rm_index.is_some()
                || modrm_rm_base == Some(Register::Rsp)
                || modrm_rm_base == Some(Register::R12)
        }

        fn push_sib(self, object: &mut Object, instruction: &Instruction) -> SResult<()> {
            if self.sib_exist(instruction) {
                let (_, base) = self
                    .modrm_base_register_code(instruction)
                    .expect("internal error")?;
                let index;
                if let Some(c) = self.modrm_index_register_code(instruction) {
                    index = c?.1;
                } else {
                    index = 0b100;
                }
                let scale = self.modrm_scale_code(instruction).expect("internal error");
                let sib = (scale << 6) | (index << 3) | base;
                object.code.push(sib);
            }
            Ok(())
        }

        fn push_imm(self, object: &mut Object, instruction: &Instruction) -> SResult<()> {
            let imm: Imm = if let Some(s) = self.imm_operand(instruction) {
                s.parse()?
            } else {
                return Ok(());
            };
            let imm_encoding_rule = instruction.encoding().imm_rule();

            let size = imm_encoding_rule.expect("internal error").size();
            let value = match imm {
                Imm::Value(v) => v,
                Imm::Label(l) => {
                    object.location.push(Location {
                        label: l,
                        offset: object.code.len(),
                        size: size,
                        rel_base: object.code.len() + size,
                    });
                    0
                }
            };
            for i in 0..8 {
                let target = ((value >> (i * 8)) & 0xff) as u8;
                if i < size {
                    object.code.push(target);
                } else {
                    if target != 0x00 && target != 0xff {
                        return Err("too large imm operand".to_string());
                    }
                }
            }
            Ok(())
        }

        fn push_disp(
            self,
            object: &mut Object,
            instruction: &Instruction,
            instruction_base: usize,
        ) -> SResult<()> {
            match self.modrm_disp(instruction) {
                Some(disp) => {
                    let value = match disp {
                        Disp::Value(v) => v,
                        Disp::Label(l) => {
                            object.location.push(Location {
                                label: l,
                                offset: object.code.len(),
                                size: 4,
                                rel_base: instruction_base,
                            });
                            0
                        }
                    };

                    let disp_len = self.disp_len(instruction);
                    for i in 0..disp_len {
                        let target = ((value >> (i * 8)) & 0xff) as u8;
                        object.code.push(target);
                    }
                    Ok(())
                }
                None => Ok(()),
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
            /// サイズ取得
            pub fn size(self) -> usize {
                match self {
                    ImmRule::Ib => 1,
                    ImmRule::Iw => 2,
                    ImmRule::Id => 4,
                    ImmRule::Iq => 8,
                }
            }

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
                line.mnemonic() == self.mnemonic() && self.operands_match_with(line)
            }

            fn operands_match_with(&self, line: &Line) -> bool {
                let operands = line.operands();
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

            /// operand_typesのオペランドが何番目の引数か取得
            pub fn get_operand_index(&self, operand_types: &[OperandType]) -> Option<usize> {
                for i in 0..self.operands.len() {
                    if operand_types.contains(&self.operands[i]) {
                        return Some(i);
                    }
                }
                None
            }

            /// 即値オペランドが何番目の引数か取得
            pub fn get_imm_operand_index(&self) -> Option<usize> {
                self.get_operand_index(&[
                    OperandType::Imm8,
                    OperandType::Imm16,
                    OperandType::Imm32,
                    OperandType::Imm64,
                    OperandType::Rel8,
                    OperandType::Rel16,
                    OperandType::Rel32,
                ])
            }

            /// レジスタオペランドが何番目の引数か取得
            pub fn get_register_operand_index(&self) -> Option<usize> {
                self.get_operand_index(&[
                    OperandType::R8,
                    OperandType::R16,
                    OperandType::R32,
                    OperandType::R64,
                ])
            }

            /// Rmオペランドが何番目の引数か取得
            pub fn get_rm_operand_index(&self) -> Option<usize> {
                self.get_operand_index(&[
                    OperandType::Rm8,
                    OperandType::Rm16,
                    OperandType::Rm32,
                    OperandType::Rm64,
                ])
            }

            /// オペランドサイズ取得
            pub fn get_operand_size(&self) -> Option<OperandSize> {
                let mut max: Option<OperandSize> = None;
                for i in self.operands() {
                    if let Some(m) = max {
                        if let Some(o) = i.operand_size() {
                            max = Some(m.max(o));
                        }
                    } else {
                        max = i.operand_size();
                    }
                }
                max
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
            /// サイズ取得
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

            pub const fn operand_size(self) -> Option<OperandSize> {
                Some(match self {
                    OperandType::Al => OperandSize::Ob,
                    OperandType::Ax => OperandSize::Ow,
                    OperandType::Eax => OperandSize::Od,
                    OperandType::Rax => OperandSize::Oq,
                    OperandType::R8 => OperandSize::Ob,
                    OperandType::R16 => OperandSize::Ow,
                    OperandType::R32 => OperandSize::Od,
                    OperandType::R64 => OperandSize::Oq,
                    OperandType::Rm8 => OperandSize::Ob,
                    OperandType::Rm16 => OperandSize::Ow,
                    OperandType::Rm32 => OperandSize::Od,
                    OperandType::Rm64 => OperandSize::Oq,
                    _ => return None,
                })
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

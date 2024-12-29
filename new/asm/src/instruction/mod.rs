use crate::parser::Line;
use crate::register::Register;
pub use instruction_database::INSTRUCTION_LIST;
use util::functions::stoi;
use util::svec::SVec;

mod instruction_database;

pub struct Instruction {
    encoding: EncodingRule,
    expression: Expression,
}

impl Instruction {
    pub fn is_match(&self, line: &Line) -> bool {
        self.expression.is_match(line)
    }
}

pub struct EncodingRule {
    opecode: SVec<3, u8>,
    rex: Option<RexRule>,
    modrm: Option<ModRmRule>,
    imm: Option<ImmRule>,
    addreg: Option<AddRegRule>,
}

pub enum RexRule {
    Rex,
    RexW,
}

pub enum ModRmRule {
    R,
    Dight(u8),
}

pub enum ImmRule {
    Ib,
    Iw,
    Id,
    Io,
}

pub enum AddRegRule {
    Rb,
    Rw,
    Rd,
    Ro,
}

pub struct Expression {
    mnemonic: &'static str,
    operands: [Option<OperandType>; 2],
}

impl Expression {
    pub fn is_match(&self, line: &Line) -> bool {
        let mnemonic_is_match = line.mnemonic() == Some(self.mnemonic);
        let operands_is_match = [false; 2];
        todo!()
    }
}

#[derive(Clone, Copy)]
pub enum OperandType {
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

impl OperandType {
    pub fn is_match(self, expr: &str) -> bool {
        match self {
            OperandType::Rel32 => number_is_match(expr, i32::MIN as i128, i32::MAX as i128),
            OperandType::R8 => register_is_match(expr, Register::is_8bit),
            OperandType::R16 => register_is_match(expr, Register::is_16bit),
            OperandType::R32 => register_is_match(expr, Register::is_32bit),
            OperandType::R64 => register_is_match(expr, Register::is_64bit),
            OperandType::Imm8 => number_is_match(expr, i8::MIN as i128, u8::MAX as i128),
            OperandType::Imm16 => number_is_match(expr, i16::MIN as i128, u16::MAX as i128),
            OperandType::Imm32 => number_is_match(expr, i32::MIN as i128, u32::MAX as i128),
            OperandType::Imm64 => number_is_match(expr, i64::MIN as i128, u64::MAX as i128),
            OperandType::Rm8 => {
                rm_is_match(expr, Register::is_8bit, i8::MIN as i128, i8::MAX as i128)
            }
            OperandType::Rm16 => {
                rm_is_match(expr, Register::is_16bit, i16::MIN as i128, i16::MAX as i128)
            }
            OperandType::Rm32 => {
                rm_is_match(expr, Register::is_32bit, i32::MIN as i128, i32::MAX as i128)
            }
            OperandType::Rm64 => {
                rm_is_match(expr, Register::is_64bit, i64::MIN as i128, i64::MAX as i128)
            }
        }
    }
}

fn number_is_match(expr: &str, min: i128, max: i128) -> bool {
    let value = stoi(expr);
    value.is_some() && min <= value.expect("unknown error") && value.expect("unknown error") <= max
}

fn register_is_match(expr: &str, p: fn(Register) -> bool) -> bool {
    let value = expr.parse::<Register>();
    if let Ok(r) = value {
        p(r)
    } else {
        false
    }
}

fn rm_is_match(expr: &str, p: fn(Register) -> bool, max: i128, min: i128) -> bool {
    // disp[base, index, scale]
    let Some(parse_rm) = parse_rm(expr) else {
        return false;
    };

    let base_is_match = p(parse_rm.1);
    let index_is_match = if let Some((i, _)) = parse_rm.2 {
        p(i)
    } else {
        true
    };
    let disp_is_match = min as i64 <= parse_rm.0 && parse_rm.0 <= max as i64;

    base_is_match && index_is_match && disp_is_match
}

fn parse_rm(expr: &str) -> Option<(i64, Register, Option<(Register, u8)>)> {
    let disp: i64 = if !expr.starts_with('[') {
        stoi(expr.split_once('[')?.0)? as i64
    } else {
        0
    };
    todo!()
}

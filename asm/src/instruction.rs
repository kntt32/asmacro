use crate::{functions::parse_rm, line::Line, register::Register};
use std::cmp::{Eq, Ord, Ordering, PartialEq, PartialOrd};
use util::{functions::stoi, svec::SVec};

pub use instruction_database::INSTRUCTION_LIST;
mod instruction_database;

/// Instruction properties
#[derive(Clone, Copy, Debug)]
pub struct Instruction {
    encoding: EncodingRule,
    expression: Expression,
}

impl Instruction {
    /// Get opecode
    pub const fn opecode(&self) -> SVec<3, u8> {
        self.encoding.opecode
    }

    /// Get mnemonic
    pub const fn mnemonic(&self) -> &'static str {
        self.expression.mnemonic()
    }

    /// If line is match
    pub fn match_with(&self, line: &Line) -> bool {
        self.expression.match_with(line)
    }

    /// Get reference to encoding rule
    pub const fn encoding(&self) -> &EncodingRule {
        &self.encoding
    }

    /// Get reference to expression
    pub const fn expression(&self) -> &Expression {
        &self.expression
    }
}

/// Encoding rule information
#[derive(Clone, Copy, Debug)]
pub struct EncodingRule {
    opecode: SVec<3, u8>,
    modrm: Option<ModRmRule>,
    imm: Option<ImmRule>,
    opecode_register: Option<OpecodeRegisterRule>,
    default_operand_size: OperandSize,
}

impl EncodingRule {
    /// Get opecode
    pub fn opecode(&self) -> SVec<3, u8> {
        self.opecode
    }

    /// Get opecode register rule
    pub fn opecode_register_rule(&self) -> Option<OpecodeRegisterRule> {
        self.opecode_register
    }

    /// Get modrm rule
    pub fn modrm_rule(&self) -> Option<ModRmRule> {
        self.modrm
    }

    /// Get imm rule
    pub fn imm_rule(&self) -> Option<ImmRule> {
        self.imm
    }

    /// Get default operand size
    pub fn default_operand_size(&self) -> OperandSize {
        self.default_operand_size
    }
}

/// Operand size
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum OperandSize {
    Ob,
    Ow,
    Od,
    Oq,
}

impl OperandSize {
    pub fn value(self) -> usize {
        match self {
            OperandSize::Ob => 1,
            OperandSize::Ow => 2,
            OperandSize::Od => 4,
            OperandSize::Oq => 8,
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
    // Required method
    fn cmp(&self, other: &Self) -> Ordering {
        self.partial_cmp(other).expect("unknown error")
    }
}

/// ModRm encoding rule
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ModRmRule {
    R,
    Dight(u8),
}

/// Immediately encoding rule
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ImmRule {
    Ib,
    Iw,
    Id,
    Iq,
}

impl ImmRule {
    pub fn operand_type(self) -> OperandType {
        match self {
            ImmRule::Ib => OperandType::Imm8,
            ImmRule::Iw => OperandType::Imm16,
            ImmRule::Id => OperandType::Imm32,
            ImmRule::Iq => OperandType::Imm64,
        }
    }
}

/// Encoding rule of register embed in opecode
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum OpecodeRegisterRule {
    Rb,
    Rw,
    Rd,
    Rq,
}

/// Information about how to expressed in assembly code
#[derive(Clone, Copy, Debug)]
pub struct Expression {
    mnemonic: &'static str,
    operands: [Option<OperandType>; 2],
}

impl Expression {
    /// Get mnemonic
    pub const fn mnemonic(&self) -> &'static str {
        self.mnemonic
    }

    /// Get operand types
    pub const fn operands(&self) -> [Option<OperandType>; 2] {
        self.operands
    }

    /// If self is match with line
    pub fn match_with(&self, line: &Line) -> bool {
        self.mnemonic_match_with(line) && self.operands_match_with(line)
    }

    fn mnemonic_match_with(&self, line: &Line) -> bool {
        line.mnemonic() == Some(self.mnemonic)
    }

    fn operands_match_with(&self, line: &Line) -> bool {
        let Some(operands) = line.operands() else {
            return false;
        };

        for i in 0..2 {
            if let Some(operand_type) = self.operands[i] {
                let Some(operand) = operands[i] else {
                    return false;
                };
                if !operand_type.match_with(operand) {
                    return false;
                }
            }
        }

        true
    }

    /// Get operand index by operand type
    pub fn get_operand_index_by_type(&self, operand_type: OperandType) -> Option<usize> {
        for i in 0..2 {
            if self.operands[i] == Some(operand_type) {
                return Some(i);
            }
        }
        None
    }
}

/// Operand types
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

impl OperandType {
    /// Get operand size
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

    /// If self is match with expr
    pub fn match_with(self, expr: &str) -> bool {
        match self {
            OperandType::Al => expr == "al",
            OperandType::Ax => expr == "ax",
            OperandType::Eax => expr == "eax",
            OperandType::Rax => expr == "rax",
            OperandType::Rel8 => number_match_with(expr, i8::MIN as i128, i8::MAX as i128),
            OperandType::Rel16 => number_match_with(expr, i16::MIN as i128, i16::MAX as i128),
            OperandType::Rel32 => number_match_with(expr, i32::MIN as i128, i32::MAX as i128),
            OperandType::R8 => register_match_with(expr, Register::operand_r8),
            OperandType::R16 => register_match_with(expr, Register::operand_r16),
            OperandType::R32 => register_match_with(expr, Register::operand_r32),
            OperandType::R64 => register_match_with(expr, Register::operand_r64),
            OperandType::Imm8 => number_match_with(expr, i8::MIN as i128, u8::MAX as i128),
            OperandType::Imm16 => number_match_with(expr, i16::MIN as i128, u16::MAX as i128),
            OperandType::Imm32 => number_match_with(expr, i32::MIN as i128, u32::MAX as i128),
            OperandType::Imm64 => number_match_with(expr, i64::MIN as i128, u64::MAX as i128),
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
    value.is_some() && min <= value.expect("unknown error") && value.expect("unknown error") <= max
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
                    Some((index, scale)) => index.operand_rm_ref_index() && is_valid_scale(scale),
                    None => true,
                };
                let disp_match = disp_min <= disp as i128 && disp as i128 <= disp_max;
                base_match && index_match && disp_match
            }
            None => false,
        }
    }
}

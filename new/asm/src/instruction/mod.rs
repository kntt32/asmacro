use crate::line::Line;
use crate::register::Register;
pub use instruction_database::INSTRUCTION_LIST;
use util::functions::{get_inner_bracket, result_to_option, stoi};
use util::svec::SVec;

mod instruction_database;

#[derive(Clone, Copy, Debug)]
pub struct Instruction {
    encoding: EncodingRule,
    expression: Expression,
}

impl Instruction {
    pub const fn mnemonic(&self) -> &'static str {
        self.expression.mnemonic()
    }

    pub fn is_match(&self, line: &Line) -> bool {
        self.expression.is_match(line)
    }
}

#[derive(Clone, Copy, Debug)]
pub struct EncodingRule {
    opecode: SVec<3, u8>,
    rex: Option<RexRule>,
    modrm: Option<ModRmRule>,
    imm: Option<ImmRule>,
    addreg: Option<AddRegRule>,
}

#[derive(Clone, Copy, Debug)]
pub enum RexRule {
    Rex,
    RexW,
}

#[derive(Clone, Copy, Debug)]
pub enum ModRmRule {
    R,
    Dight(u8),
}

#[derive(Clone, Copy, Debug)]
pub enum ImmRule {
    Imm8,
    Imm16,
    Imm32,
    Imm64,
}

#[derive(Clone, Copy, Debug)]
pub enum AddRegRule {
    R8,
    R16,
    R32,
    R64,
}

#[derive(Clone, Copy, Debug)]
pub struct Expression {
    mnemonic: &'static str,
    operands: [Option<OperandType>; 2],
}

impl Expression {
    pub const fn mnemonic(&self) -> &'static str {
        self.mnemonic
    }

    pub fn is_match(&self, line: &Line) -> bool {
        self.mnemonic_is_match(line) && self.operands_is_match(line)
    }

    fn mnemonic_is_match(&self, line: &Line) -> bool {
        line.mnemonic() == Some(self.mnemonic)
    }

    fn operands_is_match(&self, line: &Line) -> bool {
        let Some(operands) = line.operands() else {
            return false;
        };

        for i in 0..2 {
            if let Some(operand_type) = self.operands[i] {
                let Some(operand) = operands[i] else {
                    return false;
                };
                if !operand_type.is_match(operand) {
                    return false;
                }
            }
        }

        true
    }
}

#[derive(Clone, Copy, Debug)]
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

fn rm_is_match(expr: &str, p: fn(Register) -> bool, min: i128, max: i128) -> bool {
    if register_is_match(expr, p) {
        true
    } else {
        // disp[base, index, scale]
        let Some(parse_rm) = parse_rm(expr.trim()) else {
            return false;
        };

        let base_is_match = p(parse_rm.1);
        let index_is_match = if let Some((i, _)) = parse_rm.2 {
            p(i)
        } else {
            true
        };
        let disp_is_match = min <= parse_rm.0 as i128 && parse_rm.0 as i128 <= max;
        
        base_is_match && index_is_match && disp_is_match
    }
}

fn parse_rm(mut expr: &str) -> Option<(i64, Register, Option<(Register, u8)>)> {
    let disp: i64 = if !expr.starts_with('[') {
        let value = stoi(expr.split_once('[')?.0)?;
        if i64::MIN as i128 <= value && value <= i64::MAX as i128 {
            value as i64
        } else {
            return None;
        }
    } else {
        0
    };

    expr = expr.split_once('[')?.1.trim();
    if !expr.ends_with(']') {
        return None;
    };
    expr = &expr[..expr.len() - ']'.len_utf8()];
    let mut arguments_iter = expr.split(',');

    let base = result_to_option(arguments_iter.next()?.parse::<Register>())?;

    let index = if let Some(s) = arguments_iter.next() {
        result_to_option(s.parse::<Register>())?
    } else {
        return Some((disp, base, None));
    };

    let scale = if let Some(s) = arguments_iter.next() {
        let value = stoi(s)?;
        if value == 1 || value == 2 || value == 4 || value == 8 {
            value as u8
        } else {
            return None;
        }
    } else {
        return Some((disp, base, Some((index, 1))));
    };

    Some((disp, base, Some((index, scale))))
}

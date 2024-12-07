use super::ml_gen::*;
use super::*;
use line_parser::{get_reg64_str, get_rm64_ref_str};
use util::functions::stoi;
use util::functions::{get_inner_expr, match_str, MatchStr};
use util::svec::SVec;

mod line_parser;

#[derive(Clone, Copy, Debug)]
pub struct Line<'a> {
    label: Option<&'a str>,
    mnemonic: Option<&'a str>,
    operands: SVec<2, &'a str>,
}

impl<'a> Line<'a> {
    pub fn new(
        label: Option<&'a str>,
        mnemonic: Option<&'a str>,
        operands: SVec<2, &'a str>,
    ) -> Self {
        Line {
            label: label,
            mnemonic: mnemonic,
            operands: operands,
        }
    }

    pub fn get_opindex(self) -> Option<usize> {
        for i in 0..ops_list.len() {
            if self.mnemonic.is_some()
                && self.mnemonic? == ops_list[i].mnemonic
                && ops_list[i].operands.len() == self.operands.len()
            {
                let mut flag = true;
                for k in 0..ops_list[i].operands.len() {
                    if !ops_list[i].operands[k].is_match(self.operands[k]) {
                        flag = false;
                        break;
                    }
                }
                if flag {
                    return Some(i);
                }
            }
        }
        return None;
    }

    pub fn encode(self) -> Result<EncodedLine<'a>, ()> {
        if self.mnemonic == None {
            Ok(EncodedLine {
                label: self.label,
                code: None,
                len: 0,
            })
        } else {
            for i in ops_list {
                if let Ok(encoded_line) = self.encode_with_operator(*i) {
                    return Ok(encoded_line);
                }
            }
            Err(())
        }
    }

    fn encode_with_operator(&self, op: Operator) -> Result<EncodedLine<'a>, ()> {
        todo!();
        Err(())
    }
}

struct EncodedLine<'a> {
    label: Option<&'a str>,
    code: Option<MlBin>,
    len: usize,
}

#[derive(Clone, Copy)]
struct Operator {
    mnemonic: &'static str,
    opecode: SVec<3, u8>,
    operands: SVec<2, OperandType>,
    encode_rule: Rule,
}

static ops_list: &[Operator] = &[
    Operator {
        mnemonic: "mov",
        opecode: SVec::value([0xb8, 0, 0], 1),
        operands: SVec::value([OperandType::Reg64, OperandType::Imm64], 2),
        encode_rule: Rule {
            rex: RexRule::RexW,
            modrm: ModRmRule::None,
            imm: ImmRule::Id,
            add_reg: AddRegRule::Rd,
        },
    },
    Operator {
        mnemonic: "mov",
        opecode: SVec::value([0x8b, 0, 0], 1),
        operands: SVec::value([OperandType::Reg64, OperandType::Rm64], 2),
        encode_rule: Rule {
            rex: RexRule::RexW,
            modrm: ModRmRule::R,
            imm: ImmRule::None,
            add_reg: AddRegRule::None,
        },
    }, //50+rd PUSH r64
    Operator {
        mnemonic: "push",
        opecode: SVec::value([0x50, 0, 0], 1),
        operands: SVec::value([OperandType::Reg64, OperandType::None], 1),
        encode_rule: Rule {
            rex: RexRule::None,
            modrm: ModRmRule::None,
            imm: ImmRule::None,
            add_reg: AddRegRule::Rd,
        },
    }, //REX.W + 58+ rd POP r64
    Operator {
        mnemonic: "pop",
        opecode: SVec::value([0x58, 0, 0], 1),
        operands: SVec::value([OperandType::Reg64, OperandType::None], 1),
        encode_rule: Rule {
            rex: RexRule::RexW,
            modrm: ModRmRule::None,
            imm: ImmRule::None,
            add_reg: AddRegRule::Rd,
        },
    }, //C3 RET
    Operator {
        mnemonic: "ret",
        opecode: SVec::value([0xc3, 0, 0], 1),
        operands: SVec::value([OperandType::None, OperandType::None], 0),
        encode_rule: Rule {
            rex: RexRule::None,
            modrm: ModRmRule::None,
            imm: ImmRule::None,
            add_reg: AddRegRule::None,
        },
    },
];

#[derive(Clone, Copy, Default)]
enum OperandType {
    #[default]
    None,
    Imm64,
    Reg64,
    Rm64,
}

impl OperandType {
    fn is_match(self, expr: &str) -> bool {
        match self {
            OperandType::None => {
                if expr.is_empty() {
                    true
                } else {
                    false
                }
            }
            OperandType::Imm64 => stoi(expr).is_some(),
            OperandType::Reg64 => get_reg64_str(expr).is_some(),
            OperandType::Rm64 => get_reg64_str(expr).is_some() || get_rm64_ref_str(expr).is_some(),
        }
    }
}

#[derive(Clone, Copy)]
struct Rule {
    pub rex: RexRule,
    pub modrm: ModRmRule,
    pub imm: ImmRule,
    pub add_reg: AddRegRule,
}

#[derive(Clone, Copy, Default)]
enum RexRule {
    #[default]
    None,
    Rex,
    RexW,
}

#[derive(Clone, Copy)]
pub enum ModRmRule {
    None,
    R,
    Dight(u8),
}

#[derive(Clone, Copy)]
pub enum ImmRule {
    None,
    Ib,
    Iw,
    Id,
    Io,
}

#[derive(Clone, Copy)]
pub enum AddRegRule {
    None,
    Rb,
    Rw,
    Rd,
    Ro,
}

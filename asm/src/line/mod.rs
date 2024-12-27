use super::ml_gen::*;
use super::*;
use crate::ml_gen::raw_encoder::{ModRmMode, Rm, RexMode, ImmMode, AddRegMode};
use crate::registers::Register;
use line_parser::{get_reg64, get_rm64_ref};
use util::functions::stoi;
use util::functions::{get_inner_expr, match_str, MatchStr};
use util::svec::SVec;
use std::mem::transmute;

mod line_parser;

#[derive(Clone, Copy, Debug)]
pub struct Line<'a> {
    label: Option<&'a str>,
    operation: Option<(Operator, SVec<2, &'a str>)>, // (operator, operands)
}

impl<'a> Line<'a> {
    pub fn opecode(&self) -> SVec<3, u8> {
        self.operation.expect("invalid operation").0.encoding_rule.opecode
    }

    pub fn rex(&self) -> RexMode {
        self.operation.expect("invalid input").0.encoding_rule.rex
    }

    pub fn modrm(&self) -> ModRmMode {
        match self.operation.expect("invalid input").0.encoding_rule.modrm {
            ModRmRule::None => ModRmMode::None,
            ModRmRule::R => ModRmMode::R(self.modrm_reg(), self.modrm_to_rm()),
            ModRmRule::Dight(d) => ModRmMode::Dight(d, self.modrm_to_rm()),
        }
    }

    pub fn imm(&self) -> ImmMode {
        let value = stoi(self.get_operand(OperandType::Imm).expect("invalid input")).expect("invalid operation");
        let transmuted_value = unsafe { transmute::<isize, usize>(value) };

        match self.operation.expect("invalid input").0.encoding_rule.imm {
            ImmRule::None => ImmMode::None,
            ImmRule::Ib => ImmMode::Ib(transmuted_value as u8),
            ImmRule::Iw => ImmMode::Iw(transmuted_value as u16),
            ImmRule::Id => ImmMode::Id(transmuted_value as u32),
            ImmRule::Io => ImmMode::Io(transmuted_value as u64),
        }
    }

    pub fn add_reg(&self) -> AddRegMode {
        let reg: Register = self.get_operand(OperandType::Reg).expect("invalid operation").parse().expect("invalid operation");

        match self.operation.expect("invalid input").0.encoding_rule.add_reg{
            AddRegRule::None => AddRegMode::None,
            AddRegRule::Rb => if reg.is_8bit() { AddRegMode::Rb(reg) } else { panic!("invalid input") },
            AddRegRule::Rw => if reg.is_16bit() { AddRegMode::Rw(reg) } else { panic!("invalid input") },
            AddRegRule::Rd => if reg.is_32bit() { AddRegMode::Rd(reg) } else { panic!("invalid input") },
            AddRegRule::Ro => if reg.is_64bit() { AddRegMode::Ro(reg) } else { panic!("invalid input") },
        }
    }

    fn modrm_to_rm(&self) -> Rm {
        if let Some(r) = self.modrm_rm_reg() {
            Rm::Reg(r)
        }else {
            Rm::Ref {
                scale: self.modrm_rm_ref_scale().expect("invalid operation"),
                index: self.modrm_rm_ref_index().expect("invalid operation"),
                base: self.modrm_rm_ref_base().expect("invalid operation"),
                disp: self.modrm_rm_ref_disp().expect("invalid operation"),
            }
        }
    }

    fn get_operand(&self, operand_type: OperandType) -> Option<&str> {
        if 1 <= self.operation?.0.operand_types.len()
            && self.operation?.0.operand_types[0] == operand_type
        {
            Some(self.operation?.1[0])
        } else if 2 <= self.operation?.0.operand_types.len()
            && self.operation?.0.operand_types[1] == operand_type
        {
            Some(self.operation?.1[1])
        } else {
            None
        }
    }
/*
    pub fn add_reg(&self) -> Option<Register> {
        if self.operation?.0.encoding_rule.add_reg == AddRegRule::None {
            None
        } else {
            let Ok(register) = self
                .get_operand(OperandType::Reg)
                .expect("internal error")
                .parse()
            else {
                return None;
            };
            Some(register)
        }
    }
*/
    pub fn modrm_reg(&self) -> Register {
        if self.operation.expect("invalid operation").0.encoding_rule.modrm != ModRmRule::R {
            panic!("invalid operation")
        } else {
            let Ok(reg): Result<Register, ()> = self.get_operand(OperandType::Reg).expect("invalid operation").parse() else {
                panic!("invalid operation")
            };
            reg
        }
    }

    pub fn modrm_rm_reg(&self) -> Option<Register> {
        let Ok(reg): Result<Register, ()> = self.get_operand(OperandType::Rm)?.parse() else {
            return None;
        };
        Some(reg)
    }

    pub fn modrm_rm_ref_disp(&self) -> Option<i32> {
        let operand = self.get_operand(OperandType::Rm)?;
        Some(get_rm64_ref(operand)?.0 as i32)
    }

    pub fn modrm_rm_ref_base(&self) -> Option<Register> {
        let operand = self.get_operand(OperandType::Rm)?;
        Some(get_rm64_ref(operand)?.1)
    }

    pub fn modrm_rm_ref_index(&self) -> Option<Register> {
        let operand = self.get_operand(OperandType::Rm)?;
        Some(get_rm64_ref(operand)?.2?.0)
    }

    pub fn modrm_rm_ref_scale(&self) -> Option<u8> {
        let operand = self.get_operand(OperandType::Rm)?;
        Some(get_rm64_ref(operand)?.2?.1)
    }
/*
    pub fn imm(&self) -> Option<(isize, ImmRule)> {
        let imm_number: isize = stoi(self.get_operand(OperandType::Imm).expect("internal error"))?;

        Some((imm_number, self.operation?.0.encoding_rule.imm))
    }

    pub fn rel(&self) -> Option<(isize)> {
        todo!()
    }*/
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct RowLine<'a> {
    label: Option<&'a str>,
    mnemonic: Option<&'a str>,
    operands: SVec<2, &'a str>,
}

impl<'a> RowLine<'a> {
    pub fn new(
        label: Option<&'a str>,
        mnemonic: Option<&'a str>,
        operands: SVec<2, &'a str>,
    ) -> Self {
        RowLine {
            label: label,
            mnemonic: mnemonic,
            operands: operands,
        }
    }

    pub fn to_line(&self, operators_list: &[Operator]) -> Option<Line<'a>> {
        if self.mnemonic.is_some() {
            Some(Line {
                label: self.label,
                operation: Some((
                    operators_list[self.get_operation_index(operators)?],
                    self.operands,
                )),
            })
        } else {
            Some(Line {
                label: self.label,
                operation: None,
            })
        }
    }

    pub fn get_operation_index(self, operators_list: &[Operator]) -> Option<usize> {
        for i in 0..operators_list.len() {
            if self.mnemonic.is_some()
                && self.mnemonic? == operators_list[i].mnemonic
                && operators_list[i].operand_types.len() == self.operands.len()
            {
                let mut flag = true;
                for k in 0..operators_list[i].operand_types.len() {
                    if !operators_list[i].operand_types[k].is_match(self.operands[k]) {
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
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Operator {
    mnemonic: &'static str,
    operand_types: SVec<2, OperandType>,
    encoding_rule: Rule,
}

pub static operators: &[Operator] = &[
    Operator {
        mnemonic: "mov",
        operand_types: SVec::value([OperandType::Reg, OperandType::Imm], 2),
        encoding_rule: Rule {
            opecode: SVec::value([0xb8, 0, 0], 1),
            rex: RexRule::RexW,
            modrm: ModRmRule::None,
            imm: ImmRule::Id,
            add_reg: AddRegRule::Rd,
        },
    },
    Operator {
        mnemonic: "mov",
        operand_types: SVec::value([OperandType::Reg, OperandType::Rm], 2),
        encoding_rule: Rule {
            opecode: SVec::value([0x8b, 0, 0], 1),
            rex: RexRule::RexW,
            modrm: ModRmRule::R,
            imm: ImmRule::None,
            add_reg: AddRegRule::None,
        },
    }, //50+rd PUSH r64
    Operator {
        mnemonic: "push",
        operand_types: SVec::value([OperandType::Reg, OperandType::None], 1),
        encoding_rule: Rule {
            opecode: SVec::value([0x50, 0, 0], 1),
            rex: RexRule::None,
            modrm: ModRmRule::None,
            imm: ImmRule::None,
            add_reg: AddRegRule::Rd,
        },
    }, //REX.W + 58+ rd POP r64
    Operator {
        mnemonic: "pop",
        operand_types: SVec::value([OperandType::Reg, OperandType::None], 1),
        encoding_rule: Rule {
            opecode: SVec::value([0x58, 0, 0], 1),
            rex: RexRule::RexW,
            modrm: ModRmRule::None,
            imm: ImmRule::None,
            add_reg: AddRegRule::Rd,
        },
    }, //C3 RET
    Operator {
        mnemonic: "ret",
        operand_types: SVec::value([OperandType::None, OperandType::None], 0),
        encoding_rule: Rule {
            opecode: SVec::value([0xc3, 0, 0], 1),
            rex: RexRule::None,
            modrm: ModRmRule::None,
            imm: ImmRule::None,
            add_reg: AddRegRule::None,
        },
    },
];

#[derive(Clone, Copy, Default, Debug, PartialEq)]
enum OperandType {
    #[default]
    None,
    Imm,
    Reg,
    Rm,
    Rel,
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
            OperandType::Imm => stoi(expr).is_some(),
            OperandType::Reg => get_reg64(expr).is_some(),
            OperandType::Rm => get_reg64(expr).is_some() || get_rm64_ref(expr).is_some(),
            OperandType::Rel => stoi(expr).is_some(),
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
struct Rule {
    pub opecode: SVec<3, u8>,
    pub rex: RexRule,
    pub modrm: ModRmRule,
    pub imm: ImmRule,
    pub add_reg: AddRegRule,
}

pub type RexRule = RexMode;

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ModRmRule {
    None,
    R,
    Dight(u8),
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ImmRule {
    None,
    Ib,
    Iw,
    Id,
    Io,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum AddRegRule {
    None,
    Rb,
    Rw,
    Rd,
    Ro,
}

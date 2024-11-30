use super::ml_gen::*;
use super::parser::Line;
use super::*;
use util::svec::SVec;

impl<'a> Line<'a> {
    pub fn encode(self) -> Result<MlBin, ()> {
        for i in ops_list {
            if let Ok(mlgen) = self.encode_with_operator(*i) {
                return Ok(mlgen);
            }
        }
        Err(())
    }

    fn encode_with_operator(&self, op: Operator) -> Result<MlBin, ()> {
        todo!("todo");
        Err(())
    }
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

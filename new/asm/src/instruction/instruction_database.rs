use super::*;

pub static INSTRUCTION_LIST: &[Instruction] = &[PUSH_R64, PUSH_RM64];

// 50+rdPUSH r64
const PUSH_R64: Instruction = Instruction {
    encoding: EncodingRule {
        opecode: SVec::value([0x50, 0x00, 0x00], 1),
        rex: None,
        modrm: None,
        imm: None,
        addreg: Some(AddRegRule::Ro),
    },
    expression: Expression {
        mnemonic: "push",
        operands: [Some(OperandType::R64), None],
    },
};

// FF /6 PUSH r/m64
const PUSH_RM64: Instruction = Instruction {
    encoding: EncodingRule {
        opecode: SVec::value([0xff, 0x00, 0x00], 1),
        rex: None,
        modrm: Some(ModRmRule::Dight(6)),
        imm: None,
        addreg: None,
    },
    expression: Expression {
        mnemonic: "push",
        operands: [Some(OperandType::Rm64), None],
    },
};

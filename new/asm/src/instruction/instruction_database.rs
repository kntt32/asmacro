use super::*;

pub static INSTRUCTION_LIST: &[Instruction] = &[
    PUSH_R64,
    PUSH_RM64,
    PUSH_IMM64,
    MOV_RM64_R64,
    MOV_R64_IMM64,
    MOV_RM64_IMM32,
    POP_R64,
    NEAR_RET,
];

// 50+rdPUSH r64
const PUSH_R64: Instruction = Instruction {
    encoding: EncodingRule {
        opecode: SVec::value([0x50, 0x00, 0x00], 1),
        rex: None,
        modrm: None,
        imm: None,
        addreg: Some(AddRegRule::R64),
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

// 68 PUSH imm64
const PUSH_IMM64: Instruction = Instruction {
    encoding: EncodingRule {
        opecode: SVec::value([0x68, 0x00, 0x00], 1),
        rex: None,
        modrm: None,
        imm: Some(ImmRule::Imm64),
        addreg: None,
    },
    expression: Expression {
        mnemonic: "push",
        operands: [Some(OperandType::Imm64), None],
    },
};

// REX.W + 89 /rMOV r/m64,r64
const MOV_RM64_R64: Instruction = Instruction {
    encoding: EncodingRule {
        opecode: SVec::value([0x89, 0x00, 0x00], 1),
        rex: Some(RexRule::RexW),
        modrm: Some(ModRmRule::R),
        imm: None,
        addreg: None,
    },
    expression: Expression {
        mnemonic: "mov",
        operands: [Some(OperandType::Rm64), Some(OperandType::R64)],
    },
};

// REX.W + B8+ rd MOV r64,imm64
const MOV_R64_IMM64: Instruction = Instruction {
    encoding: EncodingRule {
        opecode: SVec::value([0xb8, 0x00, 0x00], 1),
        rex: Some(RexRule::RexW),
        modrm: None,
        imm: Some(ImmRule::Imm64),
        addreg: Some(AddRegRule::R64),
    },
    expression: Expression {
        mnemonic: "mov",
        operands: [Some(OperandType::R64), Some(OperandType::Imm64)],
    },
};

// REX.W + C7 /0 MOV r/m64,imm32
const MOV_RM64_IMM32: Instruction = Instruction {
    encoding: EncodingRule {
        opecode: SVec::value([0xc7, 0x00, 0x00], 1),
        rex: Some(RexRule::RexW),
        modrm: Some(ModRmRule::Dight(0)),
        imm: Some(ImmRule::Imm32),
        addreg: None,
    },
    expression: Expression {
        mnemonic: "mov",
        operands: [Some(OperandType::Rm64), Some(OperandType::Imm64)],
    },
};

// REX.W + 58+ rd POP r64
const POP_R64: Instruction = Instruction {
    encoding: EncodingRule {
        opecode: SVec::value([0x58, 0x00, 0x00], 1),
        rex: Some(RexRule::RexW),
        modrm: None,
        imm: None,
        addreg: Some(AddRegRule::R64),
    },
    expression: Expression {
        mnemonic: "pop",
        operands: [Some(OperandType::R64), None],
    },
};

// C3 RET
const NEAR_RET: Instruction = Instruction {
    encoding: EncodingRule {
        opecode: SVec::value([0xc3, 0x00, 0x00], 1),
        rex: None,
        modrm: None,
        imm: None,
        addreg: None,
    },
    expression: Expression {
        mnemonic: "ret",
        operands: [None, None],
    },
};

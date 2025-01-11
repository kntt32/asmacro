use super::*;

/// Instruction list
pub static INSTRUCTION_LIST: &[Instruction] = &[
    PUSH_R64,
    PUSH_RM64,
    PUSH_IMM64,
    MOV_RM64_R64,
    MOV_R64_IMM64,
    MOV_RM64_IMM32,
    MOV_R32_IMM32,
    POP_R64,
    NEAR_RET,
    NEAR_CALL,
];

// PUSH reg64   50 +rq
const PUSH_R64: Instruction = Instruction {
    encoding: EncodingRule {
        opecode: SVec::from_raw([0x50, 0x00, 0x00], 1),
        modrm: None,
        imm: None,
        opecode_register: Some(OpecodeRegisterRule::Rq),
        default_operand_size: OperandSize::Oq,
    },
    expression: Expression {
        mnemonic: "push",
        operands: [Some(OperandType::R64), None],
    },
};

// PUSH reg/mem64   FF /6
const PUSH_RM64: Instruction = Instruction {
    encoding: EncodingRule {
        opecode: SVec::from_raw([0xff, 0x00, 0x00], 1),
        modrm: Some(ModRmRule::Dight(6)),
        imm: None,
        opecode_register: None,
        default_operand_size: OperandSize::Oq,
    },
    expression: Expression {
        mnemonic: "push",
        operands: [Some(OperandType::Rm64), None],
    },
};

// PUSH imm64   68 id
const PUSH_IMM64: Instruction = Instruction {
    encoding: EncodingRule {
        opecode: SVec::from_raw([0x68, 0x00, 0x00], 1),
        modrm: None,
        imm: Some(ImmRule::Id),
        opecode_register: None,
        default_operand_size: OperandSize::Oq,
    },
    expression: Expression {
        mnemonic: "push",
        operands: [Some(OperandType::Imm32), None],
    },
};

// MOV reg/mem64, reg64     89 /r
const MOV_RM64_R64: Instruction = Instruction {
    encoding: EncodingRule {
        opecode: SVec::from_raw([0x89, 0x00, 0x00], 1),
        modrm: Some(ModRmRule::R),
        imm: None,
        opecode_register: None,
        default_operand_size: OperandSize::Od,
    },
    expression: Expression {
        mnemonic: "mov",
        operands: [Some(OperandType::Rm64), Some(OperandType::R64)],
    },
};

// MOV reg64, imm64     B8 +rq iq
const MOV_R64_IMM64: Instruction = Instruction {
    encoding: EncodingRule {
        opecode: SVec::from_raw([0xb8, 0x00, 0x00], 1),
        modrm: None,
        imm: Some(ImmRule::Iq),
        opecode_register: Some(OpecodeRegisterRule::Rq),
        default_operand_size: OperandSize::Od,
    },
    expression: Expression {
        mnemonic: "mov",
        operands: [Some(OperandType::R64), Some(OperandType::Imm64)],
    },
};

// MOV reg/mem64, imm32     C7 /0 id
const MOV_RM64_IMM32: Instruction = Instruction {
    encoding: EncodingRule {
        opecode: SVec::from_raw([0xc7, 0x00, 0x00], 1),
        modrm: Some(ModRmRule::Dight(0)),
        imm: Some(ImmRule::Id),
        opecode_register: None,
        default_operand_size: OperandSize::Od,
    },
    expression: Expression {
        mnemonic: "mov",
        operands: [Some(OperandType::Rm64), Some(OperandType::Imm32)],
    },
};

// MOV reg32, imm32     B8 +rd id
const MOV_R32_IMM32: Instruction = Instruction {
    encoding: EncodingRule {
        opecode: SVec::from_raw([0xb8, 0x00, 0x00], 1),
        modrm: None,
        imm: Some(ImmRule::Id),
        opecode_register: Some(OpecodeRegisterRule::Rd),
        default_operand_size: OperandSize::Od,
    },
    expression: Expression {
        mnemonic: "mov",
        operands: [Some(OperandType::R32), Some(OperandType::Imm32)],
    },
};

// POP reg64    58 +rq
const POP_R64: Instruction = Instruction {
    encoding: EncodingRule {
        opecode: SVec::from_raw([0x58, 0x00, 0x00], 1),
        modrm: None,
        imm: None,
        opecode_register: Some(OpecodeRegisterRule::Rq),
        default_operand_size: OperandSize::Oq,
    },
    expression: Expression {
        mnemonic: "pop",
        operands: [Some(OperandType::R64), None],
    },
};

// C3 RET
const NEAR_RET: Instruction = Instruction {
    encoding: EncodingRule {
        opecode: SVec::from_raw([0xc3, 0x00, 0x00], 1),
        modrm: None,
        imm: None,
        opecode_register: None,
        default_operand_size: OperandSize::Oq,
    },
    expression: Expression {
        mnemonic: "ret",
        operands: [None, None],
    },
};

// CALL rel32off    E8 id
const NEAR_CALL: Instruction = Instruction {
    encoding: EncodingRule {
        opecode: SVec::from_raw([0xe8, 0x00, 0x00], 1),
        modrm: None,
        imm: Some(ImmRule::Id),
        opecode_register: None,
        default_operand_size: OperandSize::Oq,
    },
    expression: Expression {
        mnemonic: "call",
        operands: [Some(OperandType::Rel32), None],
    },
};

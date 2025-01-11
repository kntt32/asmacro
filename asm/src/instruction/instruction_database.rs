use super::*;

/// Instruction list
pub static INSTRUCTION_LIST: &[Instruction] = &[
    ADC_AL_IMM8,
    ADC_AX_IMM16,
    ADC_EAX_IMM32,
    ADC_RAX_IMM32,
    ADC_RM8_IMM8,
    ADC_RM16_IMM16,
    ADC_RM32_IMM32,
    ADC_RM64_IMM32,
    ADC_RM16_IMM8,
    ADC_RM32_IMM8,
    ADC_RM64_IMM8,
    ADC_RM8_REG8,
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

/// ADC AL, imm8    14 ib
const ADC_AL_IMM8: Instruction = Instruction {
    encoding: EncodingRule {
        opecode: SVec::from_raw([0x14, 0x00, 0x00], 1),
        modrm: None,
        imm: Some(ImmRule::Ib),
        opecode_register: None,
        default_operand_size: OperandSize::Od,
    },
    expression: Expression {
        mnemonic: "adc",
        operands: [Some(OperandType::Al), Some(OperandType::Imm8)],
    },
};

/// ADC AX, imm16   15 iw
const ADC_AX_IMM16: Instruction = Instruction {
    encoding: EncodingRule {
        opecode: SVec::from_raw([0x15, 0x00, 0x00], 1),
        modrm: None,
        imm: Some(ImmRule::Iw),
        opecode_register: None,
        default_operand_size: OperandSize::Od,
    },
    expression: Expression {
        mnemonic: "adc",
        operands: [Some(OperandType::Ax), Some(OperandType::Imm16)],
    },
};

// ADC EAX, imm32   15 id
const ADC_EAX_IMM32: Instruction = Instruction {
    encoding: EncodingRule {
        opecode: SVec::from_raw([0x15, 0x00, 0x00], 1),
        modrm: None,
        imm: Some(ImmRule::Id),
        opecode_register: None,
        default_operand_size: OperandSize::Od,
    },
    expression: Expression {
        mnemonic: "adc",
        operands: [Some(OperandType::Eax), Some(OperandType::Imm32)],
    },
};

// ADC RAX, imm32   15 id
const ADC_RAX_IMM32: Instruction = Instruction {
    encoding: EncodingRule {
        opecode: SVec::from_raw([0x15, 0x00, 0x00], 1),
        modrm: None,
        imm: Some(ImmRule::Id),
        opecode_register: None,
        default_operand_size: OperandSize::Od,
    },
    expression: Expression {
        mnemonic: "adc",
        operands: [Some(OperandType::Rax), Some(OperandType::Imm32)],
    },
};

// ADC reg/mem8, imm8   80 /2 ib
const ADC_RM8_IMM8: Instruction = Instruction {
    encoding: EncodingRule {
        opecode: SVec::from_raw([0x80, 0x00, 0x00], 1),
        modrm: Some(ModRmRule::Dight(2)),
        imm: Some(ImmRule::Ib),
        opecode_register: None,
        default_operand_size: OperandSize::Od,
    },
    expression: Expression {
        mnemonic: "adc",
        operands: [Some(OperandType::Rm8), Some(OperandType::Imm8)],
    },
};

// ADC reg/mem16, imm16     81 /2 iw
const ADC_RM16_IMM16: Instruction = Instruction {
    encoding: EncodingRule {
        opecode: SVec::from_raw([0x81, 0x00, 0x00], 1),
        modrm: Some(ModRmRule::Dight(2)),
        imm: Some(ImmRule::Iw),
        opecode_register: None,
        default_operand_size: OperandSize::Od,
    },
    expression: Expression {
        mnemonic: "adc",
        operands: [Some(OperandType::Rm16), Some(OperandType::Imm16)],
    },
};

// ADC reg/mem32, imm32     81 /2 id
const ADC_RM32_IMM32: Instruction = Instruction {
    encoding: EncodingRule {
        opecode: SVec::from_raw([0x81, 0x00, 0x00], 1),
        modrm: Some(ModRmRule::Dight(2)),
        imm: Some(ImmRule::Id),
        opecode_register: None,
        default_operand_size: OperandSize::Od,
    },
    expression: Expression {
        mnemonic: "adc",
        operands: [Some(OperandType::Rm32), Some(OperandType::Imm32)],
    },
};

// ADC reg/mem64, imm32     81 /2 id
const ADC_RM64_IMM32: Instruction = Instruction {
    encoding: EncodingRule {
        opecode: SVec::from_raw([0x81, 0x00, 0x00], 1),
        modrm: Some(ModRmRule::Dight(2)),
        imm: Some(ImmRule::Id),
        opecode_register: None,
        default_operand_size: OperandSize::Od,
    },
    expression: Expression {
        mnemonic: "adc",
        operands: [Some(OperandType::Rm64), Some(OperandType::Imm32)],
    },
};

// ADC reg/mem16, imm8      83 /2 ib
const ADC_RM16_IMM8: Instruction = Instruction {
    encoding: EncodingRule {
        opecode: SVec::from_raw([0x83, 0x00, 0x00], 1),
        modrm: Some(ModRmRule::Dight(2)),
        imm: Some(ImmRule::Ib),
        opecode_register: None,
        default_operand_size: OperandSize::Od,
    },
    expression: Expression {
        mnemonic: "adc",
        operands: [Some(OperandType::Rm16), Some(OperandType::Imm8)],
    },
};

// ADC reg/mem32, imm8      83 /2 ib
const ADC_RM32_IMM8: Instruction = Instruction {
    encoding: EncodingRule {
        opecode: SVec::from_raw([0x83, 0x00, 0x00], 1),
        modrm: Some(ModRmRule::Dight(2)),
        imm: Some(ImmRule::Ib),
        opecode_register: None,
        default_operand_size: OperandSize::Od,
    },
    expression: Expression {
        mnemonic: "adc",
        operands: [Some(OperandType::Rm32), Some(OperandType::Imm8)],
    },
};

// ADC reg/mem64, imm8      83 /2 ib
const ADC_RM64_IMM8: Instruction = Instruction {
    encoding: EncodingRule {
        opecode: SVec::from_raw([0x83, 0x00, 0x00], 1),
        modrm: Some(ModRmRule::Dight(2)),
        imm: Some(ImmRule::Ib),
        opecode_register: None,
        default_operand_size: OperandSize::Od,
    },
    expression: Expression {
        mnemonic: "adc",
        operands: [Some(OperandType::Rm64), Some(OperandType::Imm8)],
    },
};

// ADC reg/mem8, reg8   10 /r
const ADC_RM8_REG8: Instruction = Instruction {
    encoding: EncodingRule {
        opecode: SVec::from_raw([0x10, 0x00, 0x00], 1),
        modrm: Some(ModRmRule::R),
        imm: None,
        opecode_register: None,
        default_operand_size: OperandSize::Od,
    },
    expression: Expression {
        mnemonic: "adc",
        operands: [Some(OperandType::Rm8), Some(OperandType::R8)],
    },
};

// ADC reg/mem16, reg1611 /rAdd reg16 to reg/mem16 + CF.
// ADC reg/mem32, reg3211 /rAdd reg32 to reg/mem32 + CF.
// ADC reg/mem64, reg6411 /rAdd reg64 to reg/mem64 + CF.
// ADC reg8, reg/mem812 /rAdd reg/mem8 to reg8 + CF.
// ADC reg16, reg/mem1613 /rAdd reg/mem16 to reg16 + CF.
// ADC reg32, reg/mem3213 /rAdd reg/mem32 to reg32 + CF.
// ADC reg64, reg/mem6413 /rAdd reg/mem64 to reg64 + CF.

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

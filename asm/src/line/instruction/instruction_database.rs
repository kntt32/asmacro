use super::*;
pub static INSTRUCTION_LIST: &[Instruction] = &[];
/*
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
    ADC_RM16_REG16,
    ADC_RM32_REG32,
    ADC_RM64_REG64,
    ADC_REG8_RM8,
    ADC_REG16_RM16,
    ADC_REG32_RM32,
    ADC_REG64_RM64,
    ADD_AL_IMM8,
    ADD_AX_IMM16,
    ADD_EAX_IMM32,
    ADD_RAX_IMM32,
    ADD_RM8_IMM8,
    ADD_RM16_IMM16,
    ADD_RM32_IMM32,
    ADD_RM64_IMM32,
    ADD_RM16_IMM8,
    ADD_RM32_IMM8,
    ADD_RM64_IMM8,
    ADD_RM8_REG8,
    ADD_RM16_REG16,
    ADD_RM32_REG32,
    ADD_RM64_REG64,
    ADD_REG8_RM8,
    ADD_REG16_RM16,
    ADD_REG32_RM32,
    ADD_REG64_RM64,
    AND_AL_IMM8,
    AND_AX_IMM16,
    AND_EAX_IMM32,
    AND_RAX_IMM32,
    AND_RM8_IMM8,
    AND_RM16_IMM16,
    AND_RM32_IMM32,
    AND_RM64_IMM32,
    AND_RM16_IMM8,
    AND_RM32_IMM8,
    AND_RM64_IMM8,
    AND_RM8_REG8,
    AND_RM16_REG16,
    AND_RM32_REG32,
    AND_RM64_REG64,
    AND_REG8_RM8,
    AND_REG16_RM16,
    AND_REG32_RM32,
    AND_REG64_RM64,
    BSF_REG16_RM16,
    BSF_REG32_RM32,
    BSF_REG64_RM64,
    BSR_REG16_RM16,
    BSR_REG32_RM32,
    BSR_REG64_RM64,
    NEAR_CALL_REL32,
    NEAR_CALL_RM64,
    CLD,
    CMP_AL_IMM8,
    CMP_AX_IMM16,
    CMP_EAX_IMM32,
    CMP_RAX_IMM32,
    CMP_RM8_IMM8,
    CMP_RM16_IMM16,
    CMP_RM32_IMM32,
    CMP_RM64_IMM32,
    CMP_RM16_IMM8,
    CMP_RM32_IMM8,
    CMP_RM64_IMM8,
    CMP_RM8_R8,
    CMP_RM16_R16,
    CMP_RM32_R32,
    CMP_RM64_R64,
    CMP_R8_RM8,
    CMP_R16_RM16,
    CMP_R32_RM32,
    CMP_R64_RM64,
    CPUID,
    CQO,
    DEC_RM8,
    DEC_RM16,
    DEC_RM32,
    DEC_RM64,
    DEC_R16,
    DEC_R32,
    DIV_RM8,
    DIV_RM16,
    DIV_RM32,
    DIV_RM64,
    IDIV_RM8,
    IDIV_RM16,
    IDIV_RM32,
    IDIV_RM64,
    PUSH_R64,
    PUSH_RM64,
    PUSH_IMM64,
    MOV_RM64_R64,
    MOV_R64_IMM64,
    MOV_RM64_IMM32,
    MOV_R32_IMM32,
    POP_R64,
    NEAR_RET,
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

// ADC reg/mem16, reg16    11 /r    Add reg16 to reg/mem16 + CF.
const ADC_RM16_REG16: Instruction = Instruction {
    encoding: EncodingRule {
        opecode: SVec::from_raw([0x11, 0x00, 0x00], 1),
        modrm: Some(ModRmRule::R),
        imm: None,
        opecode_register: None,
        default_operand_size: OperandSize::Od,
    },
    expression: Expression {
        mnemonic: "adc",
        operands: [Some(OperandType::Rm16), Some(OperandType::R16)],
    },
};

// ADC reg/mem32, reg3211 /rAdd reg32 to reg/mem32 + CF.
const ADC_RM32_REG32: Instruction = Instruction {
    encoding: EncodingRule {
        opecode: SVec::from_raw([0x11, 0x00, 0x00], 1),
        modrm: Some(ModRmRule::R),
        imm: None,
        opecode_register: None,
        default_operand_size: OperandSize::Od,
    },
    expression: Expression {
        mnemonic: "adc",
        operands: [Some(OperandType::Rm32), Some(OperandType::R32)],
    },
};

// ADC reg/mem64, reg6411 /rAdd reg64 to reg/mem64 + CF.
const ADC_RM64_REG64: Instruction = Instruction {
    encoding: EncodingRule {
        opecode: SVec::from_raw([0x11, 0x00, 0x00], 1),
        modrm: Some(ModRmRule::R),
        imm: None,
        opecode_register: None,
        default_operand_size: OperandSize::Od,
    },
    expression: Expression {
        mnemonic: "adc",
        operands: [Some(OperandType::Rm64), Some(OperandType::R64)],
    },
};

// ADC reg8, reg/mem812 /rAdd reg/mem8 to reg8 + CF.
const ADC_REG8_RM8: Instruction = Instruction {
    encoding: EncodingRule {
        opecode: SVec::from_raw([0x12, 0x00, 0x00], 1),
        modrm: Some(ModRmRule::R),
        imm: None,
        opecode_register: None,
        default_operand_size: OperandSize::Od,
    },
    expression: Expression {
        mnemonic: "adc",
        operands: [Some(OperandType::R8), Some(OperandType::Rm8)],
    },
};

// ADC reg16, reg/mem1613 /rAdd reg/mem16 to reg16 + CF.
const ADC_REG16_RM16: Instruction = Instruction {
    encoding: EncodingRule {
        opecode: SVec::from_raw([0x13, 0x00, 0x00], 1),
        modrm: Some(ModRmRule::R),
        imm: None,
        opecode_register: None,
        default_operand_size: OperandSize::Od,
    },
    expression: Expression {
        mnemonic: "adc",
        operands: [Some(OperandType::R16), Some(OperandType::Rm16)],
    },
};

// ADC reg32, reg/mem3213 /rAdd reg/mem32 to reg32 + CF.
const ADC_REG32_RM32: Instruction = Instruction {
    encoding: EncodingRule {
        opecode: SVec::from_raw([0x13, 0x00, 0x00], 1),
        modrm: Some(ModRmRule::R),
        imm: None,
        opecode_register: None,
        default_operand_size: OperandSize::Od,
    },
    expression: Expression {
        mnemonic: "adc",
        operands: [Some(OperandType::R32), Some(OperandType::Rm32)],
    },
};

// ADC reg64, reg/mem6413 /rAdd reg/mem64 to reg64 + CF.
const ADC_REG64_RM64: Instruction = Instruction {
    encoding: EncodingRule {
        opecode: SVec::from_raw([0x13, 0x00, 0x00], 1),
        modrm: Some(ModRmRule::R),
        imm: None,
        opecode_register: None,
        default_operand_size: OperandSize::Od,
    },
    expression: Expression {
        mnemonic: "adc",
        operands: [Some(OperandType::R64), Some(OperandType::Rm64)],
    },
};

// ADD AL, imm8 04 ib Add imm8 to AL.
const ADD_AL_IMM8: Instruction = Instruction {
    encoding: EncodingRule {
        opecode: SVec::from_raw([0x04, 0x00, 0x00], 1),
        modrm: None,
        imm: Some(ImmRule::Ib),
        opecode_register: None,
        default_operand_size: OperandSize::Od,
    },
    expression: Expression {
        mnemonic: "add",
        operands: [Some(OperandType::Al), Some(OperandType::Imm8)],
    },
};

// ADD AX, imm16 05 iw Add imm16 to AX.
const ADD_AX_IMM16: Instruction = Instruction {
    encoding: EncodingRule {
        opecode: SVec::from_raw([0x05, 0x00, 0x00], 1),
        modrm: None,
        imm: Some(ImmRule::Iw),
        opecode_register: None,
        default_operand_size: OperandSize::Od,
    },
    expression: Expression {
        mnemonic: "add",
        operands: [Some(OperandType::Ax), Some(OperandType::Imm16)],
    },
};

// ADD EAX, imm32 05 id Add imm32 to EAX.
const ADD_EAX_IMM32: Instruction = Instruction {
    encoding: EncodingRule {
        opecode: SVec::from_raw([0x05, 0x00, 0x00], 1),
        modrm: None,
        imm: Some(ImmRule::Id),
        opecode_register: None,
        default_operand_size: OperandSize::Od,
    },
    expression: Expression {
        mnemonic: "add",
        operands: [Some(OperandType::Eax), Some(OperandType::Imm32)],
    },
};

// ADD RAX, imm32 05 id Add sign-extended imm32 to RAX.
const ADD_RAX_IMM32: Instruction = Instruction {
    encoding: EncodingRule {
        opecode: SVec::from_raw([0x05, 0x00, 0x00], 1),
        modrm: None,
        imm: Some(ImmRule::Id),
        opecode_register: None,
        default_operand_size: OperandSize::Od,
    },
    expression: Expression {
        mnemonic: "add",
        operands: [Some(OperandType::Rax), Some(OperandType::Imm32)],
    },
};

// ADD reg/mem8, imm8 80 /0 ib Add imm8 to reg/mem8.
const ADD_RM8_IMM8: Instruction = Instruction {
    encoding: EncodingRule {
        opecode: SVec::from_raw([0x80, 0x00, 0x00], 1),
        modrm: Some(ModRmRule::Dight(0)),
        imm: Some(ImmRule::Ib),
        opecode_register: None,
        default_operand_size: OperandSize::Od,
    },
    expression: Expression {
        mnemonic: "add",
        operands: [Some(OperandType::Rm8), Some(OperandType::Imm8)],
    },
};

// ADD reg/mem16, imm16 81 /0 iw Add imm16 to reg/mem16
const ADD_RM16_IMM16: Instruction = Instruction {
    encoding: EncodingRule {
        opecode: SVec::from_raw([0x81, 0x00, 0x00], 1),
        modrm: Some(ModRmRule::Dight(0)),
        imm: Some(ImmRule::Iw),
        opecode_register: None,
        default_operand_size: OperandSize::Od,
    },
    expression: Expression {
        mnemonic: "add",
        operands: [Some(OperandType::Rm16), Some(OperandType::Imm16)],
    },
};

// ADD reg/mem32, imm32 81 /0 id Add imm32 to reg/mem32.
const ADD_RM32_IMM32: Instruction = Instruction {
    encoding: EncodingRule {
        opecode: SVec::from_raw([0x81, 0x00, 0x00], 1),
        modrm: Some(ModRmRule::Dight(0)),
        imm: Some(ImmRule::Id),
        opecode_register: None,
        default_operand_size: OperandSize::Od,
    },
    expression: Expression {
        mnemonic: "add",
        operands: [Some(OperandType::Rm32), Some(OperandType::Imm32)],
    },
};

// ADD reg/mem64, imm32 81 /0 id Add sign-extended imm32 to reg/mem64.
const ADD_RM64_IMM32: Instruction = Instruction {
    encoding: EncodingRule {
        opecode: SVec::from_raw([0x81, 0x00, 0x00], 1),
        modrm: Some(ModRmRule::Dight(0)),
        imm: Some(ImmRule::Id),
        opecode_register: None,
        default_operand_size: OperandSize::Od,
    },
    expression: Expression {
        mnemonic: "add",
        operands: [Some(OperandType::Rm64), Some(OperandType::Imm32)],
    },
};

// ADD reg/mem16, imm8 83 /0 ib Add sign-extended imm8 to reg/mem16.
const ADD_RM16_IMM8: Instruction = Instruction {
    encoding: EncodingRule {
        opecode: SVec::from_raw([0x83, 0x00, 0x00], 1),
        modrm: Some(ModRmRule::Dight(0)),
        imm: Some(ImmRule::Ib),
        opecode_register: None,
        default_operand_size: OperandSize::Od,
    },
    expression: Expression {
        mnemonic: "add",
        operands: [Some(OperandType::Rm16), Some(OperandType::Imm8)],
    },
};

// ADD reg/mem32, imm8 83 /0 ib Add sign-extended imm8 to reg/mem32.
const ADD_RM32_IMM8: Instruction = Instruction {
    encoding: EncodingRule {
        opecode: SVec::from_raw([0x83, 0x00, 0x00], 1),
        modrm: Some(ModRmRule::Dight(0)),
        imm: Some(ImmRule::Ib),
        opecode_register: None,
        default_operand_size: OperandSize::Od,
    },
    expression: Expression {
        mnemonic: "add",
        operands: [Some(OperandType::Rm32), Some(OperandType::Imm8)],
    },
};

// ADD reg/mem64, imm8 83 /0 ib Add sign-extended imm8 to reg/mem64.
const ADD_RM64_IMM8: Instruction = Instruction {
    encoding: EncodingRule {
        opecode: SVec::from_raw([0x83, 0x00, 0x00], 1),
        modrm: Some(ModRmRule::Dight(0)),
        imm: Some(ImmRule::Ib),
        opecode_register: None,
        default_operand_size: OperandSize::Od,
    },
    expression: Expression {
        mnemonic: "add",
        operands: [Some(OperandType::Rm64), Some(OperandType::Imm8)],
    },
};

// ADD reg/mem8, reg8 00 /r Add reg8 to reg/mem8.
const ADD_RM8_REG8: Instruction = Instruction {
    encoding: EncodingRule {
        opecode: SVec::from_raw([0x00, 0x00, 0x00], 1),
        modrm: Some(ModRmRule::R),
        imm: None,
        opecode_register: None,
        default_operand_size: OperandSize::Od,
    },
    expression: Expression {
        mnemonic: "add",
        operands: [Some(OperandType::Rm8), Some(OperandType::R8)],
    },
};

// ADD reg/mem16, reg16 01 /r Add reg16 to reg/mem16.
const ADD_RM16_REG16: Instruction = Instruction {
    encoding: EncodingRule {
        opecode: SVec::from_raw([0x01, 0x00, 0x00], 1),
        modrm: Some(ModRmRule::R),
        imm: None,
        opecode_register: None,
        default_operand_size: OperandSize::Od,
    },
    expression: Expression {
        mnemonic: "add",
        operands: [Some(OperandType::Rm16), Some(OperandType::R16)],
    },
};

// ADD reg/mem32, reg32 01 /r Add reg32 to reg/mem32.
const ADD_RM32_REG32: Instruction = Instruction {
    encoding: EncodingRule {
        opecode: SVec::from_raw([0x01, 0x00, 0x00], 1),
        modrm: Some(ModRmRule::R),
        imm: None,
        opecode_register: None,
        default_operand_size: OperandSize::Od,
    },
    expression: Expression {
        mnemonic: "add",
        operands: [Some(OperandType::Rm32), Some(OperandType::R32)],
    },
};

// ADD reg/mem64, reg64 01 /r Add reg64 to reg/mem64.
const ADD_RM64_REG64: Instruction = Instruction {
    encoding: EncodingRule {
        opecode: SVec::from_raw([0x01, 0x00, 0x00], 1),
        modrm: Some(ModRmRule::R),
        imm: None,
        opecode_register: None,
        default_operand_size: OperandSize::Od,
    },
    expression: Expression {
        mnemonic: "add",
        operands: [Some(OperandType::Rm64), Some(OperandType::R64)],
    },
};

// ADD reg8, reg/mem8 02 /r Add reg/mem8 to reg8.
const ADD_REG8_RM8: Instruction = Instruction {
    encoding: EncodingRule {
        opecode: SVec::from_raw([0x02, 0x00, 0x00], 1),
        modrm: Some(ModRmRule::R),
        imm: None,
        opecode_register: None,
        default_operand_size: OperandSize::Od,
    },
    expression: Expression {
        mnemonic: "add",
        operands: [Some(OperandType::R8), Some(OperandType::Rm8)],
    },
};

// ADD reg16, reg/mem16 03 /r Add reg/mem16 to reg16.
const ADD_REG16_RM16: Instruction = Instruction {
    encoding: EncodingRule {
        opecode: SVec::from_raw([0x03, 0x00, 0x00], 1),
        modrm: Some(ModRmRule::R),
        imm: None,
        opecode_register: None,
        default_operand_size: OperandSize::Od,
    },
    expression: Expression {
        mnemonic: "add",
        operands: [Some(OperandType::R16), Some(OperandType::Rm16)],
    },
};

// ADD reg32, reg/mem32 03 /r Add reg/mem32 to reg32.
const ADD_REG32_RM32: Instruction = Instruction {
    encoding: EncodingRule {
        opecode: SVec::from_raw([0x03, 0x00, 0x00], 1),
        modrm: Some(ModRmRule::R),
        imm: None,
        opecode_register: None,
        default_operand_size: OperandSize::Od,
    },
    expression: Expression {
        mnemonic: "add",
        operands: [Some(OperandType::R32), Some(OperandType::Rm32)],
    },
};

// ADD reg64, reg/mem64 03 /r Add reg/mem64 to reg64.
const ADD_REG64_RM64: Instruction = Instruction {
    encoding: EncodingRule {
        opecode: SVec::from_raw([0x03, 0x00, 0x00], 1),
        modrm: Some(ModRmRule::R),
        imm: None,
        opecode_register: None,
        default_operand_size: OperandSize::Od,
    },
    expression: Expression {
        mnemonic: "add",
        operands: [Some(OperandType::R64), Some(OperandType::Rm64)],
    },
};

// AND AL, imm8     24 ib
const AND_AL_IMM8: Instruction = Instruction {
    encoding: EncodingRule {
        opecode: SVec::from_raw([0x24, 0x00, 0x00], 1),
        modrm: None,
        imm: Some(ImmRule::Ib),
        opecode_register: None,
        default_operand_size: OperandSize::Od,
    },
    expression: Expression {
        mnemonic: "and",
        operands: [Some(OperandType::Al), Some(OperandType::Imm8)],
    },
};

// AND AX, imm16    25 iw
const AND_AX_IMM16: Instruction = Instruction {
    encoding: EncodingRule {
        opecode: SVec::from_raw([0x25, 0x00, 0x00], 1),
        modrm: None,
        imm: Some(ImmRule::Iw),
        opecode_register: None,
        default_operand_size: OperandSize::Od,
    },
    expression: Expression {
        mnemonic: "and",
        operands: [Some(OperandType::Ax), Some(OperandType::Imm16)],
    },
};

// AND EAX, imm32   25 id
const AND_EAX_IMM32: Instruction = Instruction {
    encoding: EncodingRule {
        opecode: SVec::from_raw([0x25, 0x00, 0x00], 1),
        modrm: None,
        imm: Some(ImmRule::Id),
        opecode_register: None,
        default_operand_size: OperandSize::Od,
    },
    expression: Expression {
        mnemonic: "and",
        operands: [Some(OperandType::Eax), Some(OperandType::Imm32)],
    },
};

// AND RAX, imm32   25 id
const AND_RAX_IMM32: Instruction = Instruction {
    encoding: EncodingRule {
        opecode: SVec::from_raw([0x25, 0x00, 0x00], 1),
        modrm: None,
        imm: Some(ImmRule::Id),
        opecode_register: None,
        default_operand_size: OperandSize::Od,
    },
    expression: Expression {
        mnemonic: "and",
        operands: [Some(OperandType::Rax), Some(OperandType::Imm32)],
    },
};

// AND reg/mem8, imm8   80 /4 ib
const AND_RM8_IMM8: Instruction = Instruction {
    encoding: EncodingRule {
        opecode: SVec::from_raw([0x80, 0x00, 0x00], 1),
        modrm: Some(ModRmRule::Dight(4)),
        imm: Some(ImmRule::Ib),
        opecode_register: None,
        default_operand_size: OperandSize::Od,
    },
    expression: Expression {
        mnemonic: "and",
        operands: [Some(OperandType::Rm8), Some(OperandType::Imm8)],
    },
};

// AND reg/mem16, imm16     81 /4 iw
const AND_RM16_IMM16: Instruction = Instruction {
    encoding: EncodingRule {
        opecode: SVec::from_raw([0x81, 0x00, 0x00], 1),
        modrm: Some(ModRmRule::Dight(4)),
        imm: Some(ImmRule::Iw),
        opecode_register: None,
        default_operand_size: OperandSize::Od,
    },
    expression: Expression {
        mnemonic: "and",
        operands: [Some(OperandType::Rm16), Some(OperandType::Imm16)],
    },
};

// AND reg/mem32, imm32     81 /4 id
const AND_RM32_IMM32: Instruction = Instruction {
    encoding: EncodingRule {
        opecode: SVec::from_raw([0x81, 0x00, 0x00], 1),
        modrm: Some(ModRmRule::Dight(4)),
        imm: Some(ImmRule::Id),
        opecode_register: None,
        default_operand_size: OperandSize::Od,
    },
    expression: Expression {
        mnemonic: "and",
        operands: [Some(OperandType::Rm32), Some(OperandType::Imm32)],
    },
};

// AND reg/mem64, imm32     81 /4 id
const AND_RM64_IMM32: Instruction = Instruction {
    encoding: EncodingRule {
        opecode: SVec::from_raw([0x81, 0x00, 0x00], 1),
        modrm: Some(ModRmRule::Dight(4)),
        imm: Some(ImmRule::Id),
        opecode_register: None,
        default_operand_size: OperandSize::Od,
    },
    expression: Expression {
        mnemonic: "and",
        operands: [Some(OperandType::Rm64), Some(OperandType::Imm32)],
    },
};

// AND reg/mem16, imm8      83 /4 ib
const AND_RM16_IMM8: Instruction = Instruction {
    encoding: EncodingRule {
        opecode: SVec::from_raw([0x83, 0x00, 0x00], 1),
        modrm: Some(ModRmRule::Dight(4)),
        imm: Some(ImmRule::Ib),
        opecode_register: None,
        default_operand_size: OperandSize::Od,
    },
    expression: Expression {
        mnemonic: "and",
        operands: [Some(OperandType::Rm16), Some(OperandType::Imm8)],
    },
};

// AND reg/mem32, imm8      83 /4 ib
const AND_RM32_IMM8: Instruction = Instruction {
    encoding: EncodingRule {
        opecode: SVec::from_raw([0x83, 0x00, 0x00], 1),
        modrm: Some(ModRmRule::Dight(4)),
        imm: Some(ImmRule::Ib),
        opecode_register: None,
        default_operand_size: OperandSize::Od,
    },
    expression: Expression {
        mnemonic: "and",
        operands: [Some(OperandType::Rm32), Some(OperandType::Imm8)],
    },
};

// AND reg/mem64, imm8      83 /4 ib
const AND_RM64_IMM8: Instruction = Instruction {
    encoding: EncodingRule {
        opecode: SVec::from_raw([0x83, 0x00, 0x00], 1),
        modrm: Some(ModRmRule::Dight(4)),
        imm: Some(ImmRule::Ib),
        opecode_register: None,
        default_operand_size: OperandSize::Od,
    },
    expression: Expression {
        mnemonic: "and",
        operands: [Some(OperandType::Rm64), Some(OperandType::Imm8)],
    },
};

// AND reg/mem8, reg8       20 /r
const AND_RM8_REG8: Instruction = Instruction {
    encoding: EncodingRule {
        opecode: SVec::from_raw([0x20, 0x00, 0x00], 1),
        modrm: Some(ModRmRule::R),
        imm: None,
        opecode_register: None,
        default_operand_size: OperandSize::Od,
    },
    expression: Expression {
        mnemonic: "and",
        operands: [Some(OperandType::Rm8), Some(OperandType::R8)],
    },
};

// AND reg/mem16, reg16     21 /r
const AND_RM16_REG16: Instruction = Instruction {
    encoding: EncodingRule {
        opecode: SVec::from_raw([0x21, 0x00, 0x00], 1),
        modrm: Some(ModRmRule::R),
        imm: None,
        opecode_register: None,
        default_operand_size: OperandSize::Od,
    },
    expression: Expression {
        mnemonic: "and",
        operands: [Some(OperandType::Rm16), Some(OperandType::R16)],
    },
};

// AND reg/mem32, reg32     21 /r
const AND_RM32_REG32: Instruction = Instruction {
    encoding: EncodingRule {
        opecode: SVec::from_raw([0x21, 0x00, 0x00], 1),
        modrm: Some(ModRmRule::R),
        imm: None,
        opecode_register: None,
        default_operand_size: OperandSize::Od,
    },
    expression: Expression {
        mnemonic: "and",
        operands: [Some(OperandType::Rm32), Some(OperandType::R32)],
    },
};

// AND reg/mem64, reg64     21 /r
const AND_RM64_REG64: Instruction = Instruction {
    encoding: EncodingRule {
        opecode: SVec::from_raw([0x21, 0x00, 0x00], 1),
        modrm: Some(ModRmRule::R),
        imm: None,
        opecode_register: None,
        default_operand_size: OperandSize::Od,
    },
    expression: Expression {
        mnemonic: "and",
        operands: [Some(OperandType::Rm64), Some(OperandType::R64)],
    },
};

// AND reg8, reg/mem8       22 /r
const AND_REG8_RM8: Instruction = Instruction {
    encoding: EncodingRule {
        opecode: SVec::from_raw([0x22, 0x00, 0x00], 1),
        modrm: Some(ModRmRule::R),
        imm: None,
        opecode_register: None,
        default_operand_size: OperandSize::Od,
    },
    expression: Expression {
        mnemonic: "and",
        operands: [Some(OperandType::R8), Some(OperandType::Rm8)],
    },
};

// AND reg16, reg/mem16     23 /r
const AND_REG16_RM16: Instruction = Instruction {
    encoding: EncodingRule {
        opecode: SVec::from_raw([0x23, 0x00, 0x00], 1),
        modrm: Some(ModRmRule::R),
        imm: None,
        opecode_register: None,
        default_operand_size: OperandSize::Od,
    },
    expression: Expression {
        mnemonic: "and",
        operands: [Some(OperandType::R16), Some(OperandType::Rm16)],
    },
};

// AND reg32, reg/mem32     23 /r
const AND_REG32_RM32: Instruction = Instruction {
    encoding: EncodingRule {
        opecode: SVec::from_raw([0x23, 0x00, 0x00], 1),
        modrm: Some(ModRmRule::R),
        imm: None,
        opecode_register: None,
        default_operand_size: OperandSize::Od,
    },
    expression: Expression {
        mnemonic: "and",
        operands: [Some(OperandType::R32), Some(OperandType::Rm32)],
    },
};

// AND reg64, reg/mem64     23 /r
const AND_REG64_RM64: Instruction = Instruction {
    encoding: EncodingRule {
        opecode: SVec::from_raw([0x23, 0x00, 0x00], 1),
        modrm: Some(ModRmRule::R),
        imm: None,
        opecode_register: None,
        default_operand_size: OperandSize::Od,
    },
    expression: Expression {
        mnemonic: "and",
        operands: [Some(OperandType::R64), Some(OperandType::Rm64)],
    },
};

// BSF reg16, reg/mem16     0F BC /r
const BSF_REG16_RM16: Instruction = Instruction {
    encoding: EncodingRule {
        opecode: SVec::from_raw([0x0f, 0xbc, 0x00], 2),
        modrm: Some(ModRmRule::R),
        imm: None,
        opecode_register: None,
        default_operand_size: OperandSize::Od,
    },
    expression: Expression {
        mnemonic: "bsf",
        operands: [Some(OperandType::R16), Some(OperandType::Rm16)],
    },
};

// BSF reg32, reg/mem32     0F BC /r
const BSF_REG32_RM32: Instruction = Instruction {
    encoding: EncodingRule {
        opecode: SVec::from_raw([0x0f, 0xbc, 0x00], 2),
        modrm: Some(ModRmRule::R),
        imm: None,
        opecode_register: None,
        default_operand_size: OperandSize::Od,
    },
    expression: Expression {
        mnemonic: "bsf",
        operands: [Some(OperandType::R32), Some(OperandType::Rm32)],
    },
};

// BSF reg64, reg/mem64     0F BC /r
const BSF_REG64_RM64: Instruction = Instruction {
    encoding: EncodingRule {
        opecode: SVec::from_raw([0x0f, 0xbc, 0x00], 2),
        modrm: Some(ModRmRule::R),
        imm: None,
        opecode_register: None,
        default_operand_size: OperandSize::Od,
    },
    expression: Expression {
        mnemonic: "bsf",
        operands: [Some(OperandType::R64), Some(OperandType::Rm64)],
    },
};

// BSR reg16, reg/mem160F BD /rBit scan reverse on the contents of reg/mem16.
const BSR_REG16_RM16: Instruction = Instruction {
    encoding: EncodingRule {
        opecode: SVec::from_raw([0x0f, 0xbd, 0x00], 2),
        modrm: Some(ModRmRule::R),
        imm: None,
        opecode_register: None,
        default_operand_size: OperandSize::Od,
    },
    expression: Expression {
        mnemonic: "bsr",
        operands: [Some(OperandType::R16), Some(OperandType::Rm16)],
    },
};

// BSR reg32, reg/mem320F BD /rBit scan reverse on the contents of reg/mem32.
const BSR_REG32_RM32: Instruction = Instruction {
    encoding: EncodingRule {
        opecode: SVec::from_raw([0x0f, 0xbd, 0x00], 2),
        modrm: Some(ModRmRule::R),
        imm: None,
        opecode_register: None,
        default_operand_size: OperandSize::Od,
    },
    expression: Expression {
        mnemonic: "bsr",
        operands: [Some(OperandType::R32), Some(OperandType::Rm32)],
    },
};

// BSR reg64, reg/mem640F BD /rBit scan reverse on the contents of reg/mem64.
const BSR_REG64_RM64: Instruction = Instruction {
    encoding: EncodingRule {
        opecode: SVec::from_raw([0x0f, 0xbd, 0x00], 2),
        modrm: Some(ModRmRule::R),
        imm: None,
        opecode_register: None,
        default_operand_size: OperandSize::Od,
    },
    expression: Expression {
        mnemonic: "bsr",
        operands: [Some(OperandType::R64), Some(OperandType::Rm64)],
    },
};

// NEAR CALL rel32off    E8 id
const NEAR_CALL_REL32: Instruction = Instruction {
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

// NEAR CALL reg/mem64    FF \2
const NEAR_CALL_RM64: Instruction = Instruction {
    encoding: EncodingRule {
        opecode: SVec::from_raw([0xff, 0x00, 0x00], 1),
        modrm: Some(ModRmRule::Dight(2)),
        imm: None,
        opecode_register: None,
        default_operand_size: OperandSize::Oq,
    },
    expression: Expression {
        mnemonic: "call",
        operands: [Some(OperandType::Rm64), None],
    },
};

// CLD  FC  Clear the direction flag (DF) to zero.
const CLD: Instruction = Instruction {
    encoding: EncodingRule {
        opecode: SVec::from_raw([0xfc, 0x00, 0x00], 1),
        modrm: None,
        imm: None,
        opecode_register: None,
        default_operand_size: OperandSize::Od,
    },
    expression: Expression {
        mnemonic: "cld",
        operands: [None, None],
    },
};

// CMP AL, imm83C ib
const CMP_AL_IMM8: Instruction = Instruction {
    encoding: EncodingRule {
        opecode: SVec::from_raw([0x3c, 0x00, 0x00], 1),
        modrm: None,
        imm: Some(ImmRule::Ib),
        opecode_register: None,
        default_operand_size: OperandSize::Od,
    },
    expression: Expression {
        mnemonic: "cmp",
        operands: [Some(OperandType::Al), Some(OperandType::Imm8)],
    },
};

// CMP AX, imm163D iw
const CMP_AX_IMM16: Instruction = Instruction {
    encoding: EncodingRule {
        opecode: SVec::from_raw([0x3d, 0x00, 0x00], 1),
        modrm: None,
        imm: Some(ImmRule::Iw),
        opecode_register: None,
        default_operand_size: OperandSize::Od,
    },
    expression: Expression {
        mnemonic: "cmp",
        operands: [Some(OperandType::Ax), Some(OperandType::Imm16)],
    },
};

// CMP EAX, imm323D id
const CMP_EAX_IMM32: Instruction = Instruction {
    encoding: EncodingRule {
        opecode: SVec::from_raw([0x3d, 0x00, 0x00], 1),
        modrm: None,
        imm: Some(ImmRule::Id),
        opecode_register: None,
        default_operand_size: OperandSize::Od,
    },
    expression: Expression {
        mnemonic: "cmp",
        operands: [Some(OperandType::Eax), Some(OperandType::Imm32)],
    },
};

// CMP RAX, imm323D id
const CMP_RAX_IMM32: Instruction = Instruction {
    encoding: EncodingRule {
        opecode: SVec::from_raw([0x3d, 0x00, 0x00], 1),
        modrm: None,
        imm: Some(ImmRule::Id),
        opecode_register: None,
        default_operand_size: OperandSize::Od,
    },
    expression: Expression {
        mnemonic: "cmp",
        operands: [Some(OperandType::Rax), Some(OperandType::Imm32)],
    },
};

// CMP reg/mem8, imm880 /7 ib
const CMP_RM8_IMM8: Instruction = Instruction {
    encoding: EncodingRule {
        opecode: SVec::from_raw([0x80, 0x00, 0x00], 1),
        modrm: Some(ModRmRule::Dight(7)),
        imm: Some(ImmRule::Ib),
        opecode_register: None,
        default_operand_size: OperandSize::Od,
    },
    expression: Expression {
        mnemonic: "cmp",
        operands: [Some(OperandType::Rm8), Some(OperandType::Imm8)],
    },
};

// CMP reg/mem16, imm1681 /7 iw
const CMP_RM16_IMM16: Instruction = Instruction {
    encoding: EncodingRule {
        opecode: SVec::from_raw([0x81, 0x00, 0x00], 1),
        modrm: Some(ModRmRule::Dight(7)),
        imm: Some(ImmRule::Iw),
        opecode_register: None,
        default_operand_size: OperandSize::Od,
    },
    expression: Expression {
        mnemonic: "cmp",
        operands: [Some(OperandType::Rm16), Some(OperandType::Imm16)],
    },
};

// CMP reg/mem32, imm3281 /7 id
const CMP_RM32_IMM32: Instruction = Instruction {
    encoding: EncodingRule {
        opecode: SVec::from_raw([0x81, 0x00, 0x00], 1),
        modrm: Some(ModRmRule::Dight(7)),
        imm: Some(ImmRule::Id),
        opecode_register: None,
        default_operand_size: OperandSize::Od,
    },
    expression: Expression {
        mnemonic: "cmp",
        operands: [Some(OperandType::Rm32), Some(OperandType::Imm32)],
    },
};

// CMP reg/mem64, imm3281 /7 id
const CMP_RM64_IMM32: Instruction = Instruction {
    encoding: EncodingRule {
        opecode: SVec::from_raw([0x81, 0x00, 0x00], 1),
        modrm: Some(ModRmRule::Dight(7)),
        imm: Some(ImmRule::Id),
        opecode_register: None,
        default_operand_size: OperandSize::Od,
    },
    expression: Expression {
        mnemonic: "cmp",
        operands: [Some(OperandType::Rm64), Some(OperandType::Imm32)],
    },
};

// CMP reg/mem16, imm883 /7 ib
const CMP_RM16_IMM8: Instruction = Instruction {
    encoding: EncodingRule {
        opecode: SVec::from_raw([0x83, 0x00, 0x00], 1),
        modrm: Some(ModRmRule::Dight(7)),
        imm: Some(ImmRule::Ib),
        opecode_register: None,
        default_operand_size: OperandSize::Od,
    },
    expression: Expression {
        mnemonic: "cmp",
        operands: [Some(OperandType::Rm16), Some(OperandType::Imm8)],
    },
};

// CMP reg/mem32, imm883 /7 ib
const CMP_RM32_IMM8: Instruction = Instruction {
    encoding: EncodingRule {
        opecode: SVec::from_raw([0x83, 0x00, 0x00], 1),
        modrm: Some(ModRmRule::Dight(7)),
        imm: Some(ImmRule::Ib),
        opecode_register: None,
        default_operand_size: OperandSize::Od,
    },
    expression: Expression {
        mnemonic: "cmp",
        operands: [Some(OperandType::Rm32), Some(OperandType::Imm8)],
    },
};

// CMP reg/mem64, imm883 /7 ib
const CMP_RM64_IMM8: Instruction = Instruction {
    encoding: EncodingRule {
        opecode: SVec::from_raw([0x83, 0x00, 0x00], 1),
        modrm: Some(ModRmRule::Dight(7)),
        imm: Some(ImmRule::Ib),
        opecode_register: None,
        default_operand_size: OperandSize::Od,
    },
    expression: Expression {
        mnemonic: "cmp",
        operands: [Some(OperandType::Rm64), Some(OperandType::Imm8)],
    },
};

// CMP reg/mem8, reg838 /r
const CMP_RM8_R8: Instruction = Instruction {
    encoding: EncodingRule {
        opecode: SVec::from_raw([0x38, 0x00, 0x00], 1),
        modrm: Some(ModRmRule::R),
        imm: None,
        opecode_register: None,
        default_operand_size: OperandSize::Od,
    },
    expression: Expression {
        mnemonic: "cmp",
        operands: [Some(OperandType::Rm8), Some(OperandType::R8)],
    },
};

// CMP reg/mem16, reg1639 /r
const CMP_RM16_R16: Instruction = Instruction {
    encoding: EncodingRule {
        opecode: SVec::from_raw([0x39, 0x00, 0x00], 1),
        modrm: Some(ModRmRule::R),
        imm: None,
        opecode_register: None,
        default_operand_size: OperandSize::Od,
    },
    expression: Expression {
        mnemonic: "cmp",
        operands: [Some(OperandType::Rm16), Some(OperandType::R16)],
    },
};

// CMP reg/mem32, reg3239 /r
const CMP_RM32_R32: Instruction = Instruction {
    encoding: EncodingRule {
        opecode: SVec::from_raw([0x39, 0x00, 0x00], 1),
        modrm: Some(ModRmRule::R),
        imm: None,
        opecode_register: None,
        default_operand_size: OperandSize::Od,
    },
    expression: Expression {
        mnemonic: "cmp",
        operands: [Some(OperandType::Rm32), Some(OperandType::R32)],
    },
};

// CMP reg/mem64, reg6439 /r
const CMP_RM64_R64: Instruction = Instruction {
    encoding: EncodingRule {
        opecode: SVec::from_raw([0x39, 0x00, 0x00], 1),
        modrm: Some(ModRmRule::R),
        imm: None,
        opecode_register: None,
        default_operand_size: OperandSize::Od,
    },
    expression: Expression {
        mnemonic: "cmp",
        operands: [Some(OperandType::Rm64), Some(OperandType::R64)],
    },
};

// CMP reg8, reg/mem83A /r
const CMP_R8_RM8: Instruction = Instruction {
    encoding: EncodingRule {
        opecode: SVec::from_raw([0x3a, 0x00, 0x00], 1),
        modrm: Some(ModRmRule::R),
        imm: None,
        opecode_register: None,
        default_operand_size: OperandSize::Od,
    },
    expression: Expression {
        mnemonic: "cmp",
        operands: [Some(OperandType::R8), Some(OperandType::Rm8)],
    },
};

// CMP reg16, reg/mem163B /r
const CMP_R16_RM16: Instruction = Instruction {
    encoding: EncodingRule {
        opecode: SVec::from_raw([0x3b, 0x00, 0x00], 1),
        modrm: Some(ModRmRule::R),
        imm: None,
        opecode_register: None,
        default_operand_size: OperandSize::Od,
    },
    expression: Expression {
        mnemonic: "cmp",
        operands: [Some(OperandType::R16), Some(OperandType::Rm16)],
    },
};

// CMP reg32, reg/mem323B /r
const CMP_R32_RM32: Instruction = Instruction {
    encoding: EncodingRule {
        opecode: SVec::from_raw([0x3b, 0x00, 0x00], 1),
        modrm: Some(ModRmRule::R),
        imm: None,
        opecode_register: None,
        default_operand_size: OperandSize::Od,
    },
    expression: Expression {
        mnemonic: "cmp",
        operands: [Some(OperandType::R32), Some(OperandType::Rm32)],
    },
};

// CMP reg64, reg/mem643B /r
const CMP_R64_RM64: Instruction = Instruction {
    encoding: EncodingRule {
        opecode: SVec::from_raw([0x3b, 0x00, 0x00], 1),
        modrm: Some(ModRmRule::R),
        imm: None,
        opecode_register: None,
        default_operand_size: OperandSize::Od,
    },
    expression: Expression {
        mnemonic: "cmp",
        operands: [Some(OperandType::R64), Some(OperandType::Rm64)],
    },
};

// CPUID    0F A2
const CPUID: Instruction = Instruction {
    encoding: EncodingRule {
        opecode: SVec::from_raw([0x0f, 0xa2, 0x00], 2),
        modrm: None,
        imm: None,
        opecode_register: None,
        default_operand_size: OperandSize::Od,
    },
    expression: Expression {
        mnemonic: "cpuid",
        operands: [None, None],
    },
};

// CQO 99
const CQO: Instruction = Instruction {
    encoding: EncodingRule {
        opecode: SVec::from_raw([0x99, 0x00, 0x00], 1),
        modrm: None,
        imm: None,
        opecode_register: None,
        default_operand_size: OperandSize::Od,
    },
    expression: Expression {
        mnemonic: "cqo",
        operands: [None, None],
    },
};


// DEC reg/mem8FE /1
const DEC_RM8: Instruction = Instruction {
    encoding: EncodingRule {
        opecode: SVec::from_raw([0xfe, 0x00, 0x00], 1),
        modrm: Some(ModRmRule::Dight(1)),
        imm: None,
        opecode_register: None,
        default_operand_size: OperandSize::Od,
    },
    expression: Expression {
        mnemonic: "dec",
        operands: [Some(OperandType::Rm8), None],
    },
};

// DEC reg/mem16FF /1
const DEC_RM16: Instruction = Instruction {
    encoding: EncodingRule {
        opecode: SVec::from_raw([0xff, 0x00, 0x00], 1),
        modrm: Some(ModRmRule::Dight(1)),
        imm: None,
        opecode_register: None,
        default_operand_size: OperandSize::Od,
    },
    expression: Expression {
        mnemonic: "dec",
        operands: [Some(OperandType::Rm16), None],
    },
};

// DEC reg/mem32FF /1
const DEC_RM32: Instruction = Instruction {
    encoding: EncodingRule {
        opecode: SVec::from_raw([0xff, 0x00, 0x00], 1),
        modrm: Some(ModRmRule::Dight(1)),
        imm: None,
        opecode_register: None,
        default_operand_size: OperandSize::Od,
    },
    expression: Expression {
        mnemonic: "dec",
        operands: [Some(OperandType::Rm32), None],
    },
};

// DEC reg/mem64FF /1
const DEC_RM64: Instruction = Instruction {
    encoding: EncodingRule {
        opecode: SVec::from_raw([0xff, 0x00, 0x00], 1),
        modrm: Some(ModRmRule::Dight(1)),
        imm: None,
        opecode_register: None,
        default_operand_size: OperandSize::Od,
    },
    expression: Expression {
        mnemonic: "dec",
        operands: [Some(OperandType::Rm64), None],
    },
};

// DEC reg1648 +rw
const DEC_R16: Instruction = Instruction {
    encoding: EncodingRule {
        opecode: SVec::from_raw([0x48, 0x00, 0x00], 1),
        modrm: None,
        imm: None,
        opecode_register: Some(OpecodeRegisterRule::Rw),
        default_operand_size: OperandSize::Od,
    },
    expression: Expression {
        mnemonic: "dec",
        operands: [Some(OperandType::R16), None],
    },
};

// DEC reg3248 +rd
const DEC_R32: Instruction = Instruction {
    encoding: EncodingRule {
        opecode: SVec::from_raw([0x48, 0x00, 0x00], 1),
        modrm: None,
        imm: None,
        opecode_register: Some(OpecodeRegisterRule::Rd),
        default_operand_size: OperandSize::Od,
    },
    expression: Expression {
        mnemonic: "dec",
        operands: [Some(OperandType::R32), None],
    },
};

// DIV reg/mem8F6 /6
const DIV_RM8: Instruction = Instruction {
    encoding: EncodingRule {
        opecode: SVec::from_raw([0xf6, 0x00, 0x00], 1),
        modrm: Some(ModRmRule::Dight(6)),
        imm: None,
        opecode_register: None,
        default_operand_size: OperandSize::Od,
    },
    expression: Expression {
        mnemonic: "div",
        operands: [Some(OperandType::Rm8), None],
    },
};

// DIV reg/mem16F7 /6
const DIV_RM16: Instruction = Instruction {
    encoding: EncodingRule {
        opecode: SVec::from_raw([0xf7, 0x00, 0x00], 1),
        modrm: Some(ModRmRule::Dight(6)),
        imm: None,
        opecode_register: None,
        default_operand_size: OperandSize::Od,
    },
    expression: Expression {
        mnemonic: "div",
        operands: [Some(OperandType::Rm16), None],
    },
};

// DIV reg/mem32F7 /6
const DIV_RM32: Instruction = Instruction {
    encoding: EncodingRule {
        opecode: SVec::from_raw([0xf7, 0x00, 0x00], 1),
        modrm: Some(ModRmRule::Dight(6)),
        imm: None,
        opecode_register: None,
        default_operand_size: OperandSize::Od,
    },
    expression: Expression {
        mnemonic: "div",
        operands: [Some(OperandType::Rm32), None],
    },
};

// DIV reg/mem64F7 /6
const DIV_RM64: Instruction = Instruction {
    encoding: EncodingRule {
        opecode: SVec::from_raw([0xf7, 0x00, 0x00], 1),
        modrm: Some(ModRmRule::Dight(6)),
        imm: None,
        opecode_register: None,
        default_operand_size: OperandSize::Od,
    },
    expression: Expression {
        mnemonic: "div",
        operands: [Some(OperandType::Rm64), None],
    },
};

// IDIV reg/mem8F6 /7
const IDIV_RM8: Instruction = Instruction {
    encoding: EncodingRule {
        opecode: SVec::from_raw([0xf6, 0x00, 0x00], 1),
        modrm: Some(ModRmRule::Dight(7)),
        imm: None,
        opecode_register: None,
        default_operand_size: OperandSize::Od,
    },
    expression: Expression {
        mnemonic: "idiv",
        operands: [Some(OperandType::Rm8), None],
    },
};

// IDIV reg/mem16F7 /7
const IDIV_RM16: Instruction = Instruction {
    encoding: EncodingRule {
        opecode: SVec::from_raw([0xf7, 0x00, 0x00], 1),
        modrm: Some(ModRmRule::Dight(7)),
        imm: None,
        opecode_register: None,
        default_operand_size: OperandSize::Od,
    },
    expression: Expression {
        mnemonic: "idiv",
        operands: [Some(OperandType::Rm16), None],
    },
};


// IDIV reg/mem32F7 /7
const IDIV_RM32: Instruction = Instruction {
    encoding: EncodingRule {
        opecode: SVec::from_raw([0xf7, 0x00, 0x00], 1),
        modrm: Some(ModRmRule::Dight(7)),
        imm: None,
        opecode_register: None,
        default_operand_size: OperandSize::Od,
    },
    expression: Expression {
        mnemonic: "idiv",
        operands: [Some(OperandType::Rm32), None],
    },
};

// IDIV reg/mem64F7 /7
const IDIV_RM64: Instruction = Instruction {
    encoding: EncodingRule {
        opecode: SVec::from_raw([0xf7, 0x00, 0x00], 1),
        modrm: Some(ModRmRule::Dight(7)),
        imm: None,
        opecode_register: None,
        default_operand_size: OperandSize::Od,
    },
    expression: Expression {
        mnemonic: "idiv",
        operands: [Some(OperandType::Rm64), None],
    },
};

// IMUL reg/mem8F6 /5
// IMUL reg/mem16F7 /5
// IMUL reg/mem32F7 /5
// IMUL reg/mem64F7 /5
// IMUL reg16, reg/mem160F AF /r
// IMUL reg32, reg/mem320F AF /r
// IMUL reg64, reg/mem640F AF /r
// IMUL reg16, reg/mem16, imm86B /r ib
// IMUL reg32, reg/mem32, imm86B /r ib
// IMUL reg64, reg/mem64, imm86B /r ib
// IMUL reg16, reg/mem16, imm1669 /r iw
// IMUL reg32, reg/mem32, imm3269 /r id
// IMUL reg64, reg/mem64, imm3269 /r id
/*
IN AL, imm8E4 ibInput a byte from the port at the address specified by
imm8 and put it into the AL register.
IN AX, imm8E5 ibInput a word from the port at the address specified by
imm8 and put it into the AX register.
IN EAX, imm8E5 ibInput a doubleword from the port at the address
specified by imm8 and put it into the EAX register.
IN AL, DXECInput a byte from the port at the address specified by the
DX register and put it into the AL register.
IN AX, DXEDInput a word from the port at the address specified by
the DX register and put it into the AX register.
IN EAX, DXED

IN AL, imm8E4 ibInput a byte from the port at the address specified by
imm8 and put it into the AL register.
IN AX, imm8E5 ibInput a word from the port at the address specified by
imm8 and put it into the AX register.
IN EAX, imm8E5 ibInput a doubleword from the port at the address
specified by imm8 and put it into the EAX register.
IN AL, DXECInput a byte from the port at the address specified by the
DX register and put it into the AL register.
IN AX, DXEDInput a word from the port at the address specified by
the DX register and put it into the AX register.
IN EAX, DXED
*/

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
*/

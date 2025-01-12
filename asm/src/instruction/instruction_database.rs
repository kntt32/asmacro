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
    NEAR_CALL_REL16,
    NEAR_CALL_REL32,
    NEAR_CALL_RM64,
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

// NEAR CALL rel16off    E8 iw
const NEAR_CALL_REL16: Instruction = Instruction {
    encoding: EncodingRule {
        opecode: SVec::from_raw([0xe8, 0x00, 0x00], 1),
        modrm: None,
        imm: Some(ImmRule::Iw),
        opecode_register: None,
        default_operand_size: OperandSize::Oq,
    },
    expression: Expression {
        mnemonic: "call",
        operands: [Some(OperandType::Rel16), None],
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

/*
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
*/

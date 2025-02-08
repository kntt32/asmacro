use super::Instruction;

impl Instruction {
    pub const STANDARD: &str = "
            ADC al imm8,14 ib
            ADC ax imm16,15 iw
            ADC eax imm32,15 id
            ADC rax imm32,15 id
            ADC reg/mem8 imm8,80 /2 ib
            ADC reg/mem16 imm16,81 /2 iw
            ADC reg/mem32 imm32,81 /2 id
            ADC reg/mem64 imm32,81 /2 id
            ADC reg/mem16 imm8,83 /2 ib
            ADC reg/mem32 imm8,83 /2 ib
            ADC reg/mem64 imm8,83 /2 ib
            ADC reg/mem8 reg8,10 /r
            ADC reg/mem16 reg16,11 /r
            ADC reg/mem32 reg32,11 /r
            ADC reg/mem64 reg64,11 /r
            ADC reg8 reg/mem8,12 /r
            ADC reg16 reg/mem16,13 /r
            ADC reg32 reg/mem32, 13 /r
            ADC reg64 reg/mem64, 13 /r

            CALL rel16,E8 iw
            CALL rel32,E8 id
            CALL reg/mem64,FF /2 oq

            LEA reg16 reg/mem16,8D /r
            LEA reg32 reg/mem32,8D /r
            LEA reg64 reg/mem64,8D /r

            MOV reg/mem8 reg8,88 /r
            MOV reg/mem16 reg16,89 /r
            MOV reg/mem32 reg32,89 /r
            MOV reg/mem64 reg64,89 /r
            MOV reg8 reg/mem8,8A /r
            MOV reg16 reg/mem16,8B /r
            MOV reg32 reg/mem32,8B /r
            MOV reg64 reg/mem64,8B /r
            MOV reg8 imm8,B0 +rb ib
            MOV reg16 imm16,B8 +rw iw
            MOV reg32 imm32,B8 +rd id
            MOV reg64 imm64,B8 +rq iq
            MOV reg/mem8 imm8,C6 /0 ib
            MOV reg/mem16 imm16,C7 /0 iw
            MOV reg/mem32 imm32,C7 /0 id
            MOV reg/mem64 imm32,C7 /0 id

            RET, C3

            SYSCALL, 0F 05";

    /// 基本的なx64命令取得
    pub fn standard() -> Vec<Instruction> {
        let mut vec = Vec::new();
        for line in Self::STANDARD.lines() {
            if !line.trim().is_empty() {
                vec.push(line.parse().expect("internal error"));
            }
        }
        vec
    }
}

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

            ADD AL imm8,04 ib
            ADD AX imm16,05 iw
            ADD EAX imm32,05 id
            ADD RAX imm32,05 id
            ADD reg/mem8 imm8,80 /0 ib
            ADD reg/mem16 imm16,81 /0 iw
            ADD reg/mem32 imm32,81 /0 id
            ADD reg/mem64 imm32,81 /0 id
            ADD reg/mem16 imm8,83 /0 ib
            ADD reg/mem32 imm8,83 /0 ib
            ADD reg/mem64 imm8,83 /0 ib
            ADD reg/mem8 reg8,00 /r
            ADD reg/mem16 reg16,01 /r
            ADD reg/mem32 reg32,01 /r
            ADD reg/mem64 reg64,01 /r
            ADD reg8 reg/mem8,02 /r
            ADD reg16 reg/mem16,03 /r
            ADD reg32 reg/mem32,03 /r
            ADD reg64 reg/mem64,03 /r

            AND AL imm8,24 ib
            AND AX imm16,25 iw
            AND EAX imm32,25 id
            AND RAX imm32,25 id
            AND reg/mem8 imm8,80 /4 ib
            AND reg/mem16 imm16,81 /4 iw
            AND reg/mem32 imm32,81 /4 id
            AND reg/mem64 imm32,81 /4 id
            AND reg/mem16 imm8,83 /4 ib
            AND reg/mem32 imm8,83 /4 ib
            AND reg/mem64 imm8,83 /4 ib
            AND reg/mem8 reg8,20 /r
            AND reg/mem16 reg16,21 /r
            AND reg/mem32 reg32,21 /r
            AND reg/mem64 reg64,21 /r
            AND reg8 reg/mem8,22 /r
            AND reg16 reg/mem16,23 /r
            AND reg32 reg/mem32,23 /r
            AND reg64 reg/mem64,23 /r

            BSF reg16 reg/mem16,0F BC /r
            BSF reg32 reg/mem32,0F BC /r
            BSF reg64 reg/mem64,0F BC /r

            BSR reg16 reg/mem16,0F BD /r
            BSR reg32 reg/mem32,0F BD /r
            BSR reg64 reg/mem64,0F BD /r

            BSWAP reg32,0F C8 +rd
            BSWAP reg64,0F C8 +rq

            BT reg/mem16 reg16,0F A3 /r
            BT reg/mem32 reg32,0F A3 /r
            BT reg/mem64 reg64,0F A3 /r
            BT reg/mem16 imm8,0F BA /4 ib
            BT reg/mem32 imm8,0F BA /4 ib
            BT reg/mem64 imm8,0F BA /4 ib

            BTC reg/mem16 reg16,0F BB /r
            BTC reg/mem32 reg32,0F BB /r
            BTC reg/mem64 reg64,0F BB /r
            BTC reg/mem16 imm8,0F BA /7 ib
            BTC reg/mem32 imm8,0F BA /7 ib
            BTC reg/mem64 imm8,0F BA /7 ib

            BTR reg/mem16 reg16,0F B3 /r
            BTR reg/mem32 reg32,0F B3 /r
            BTR reg/mem64 reg64,0F B3 /r
            BTR reg/mem16 imm8,0F BA /6 ib
            BTR reg/mem32 imm8,0F BA /6 ib
            BTR reg/mem64 imm8,0F BA /6 ib

            BTS reg/mem16 reg16,0F AB /r
            BTS reg/mem32 reg32,0F AB /r
            BTS reg/mem64 reg64,0F AB /r
            BTS reg/mem16 imm8,0F BA /5 ib
            BTS reg/mem32 imm8,0F BA /5 ib
            BTS reg/mem64 imm8,0F BA /5 ib

            CALL rel16,E8 iw
            CALL rel32,E8 id
            CALL reg/mem64,FF /2 oq

            CDQE,98

            CQO,99

            CLC,F8

            CLD,FC

            CLFLUSH reg/mem8,0F AE /7
            CLFLUSHOPT reg/mem8,66 0F AE /7
            CLWB,66 0F AE /6
            CLZERO RAX,0F 01 FC
            CMC,F5

            CMOVO reg16 reg/mem16,0F 40 /r
            CMOVO reg32 reg/mem32,0F 40 /r
            CMOVO reg64 reg/mem64,0F 40 /r
            CMOVNO reg16 reg/mem16,0F 41 /r
            CMOVNO reg32 reg/mem32,0F 41 /r
            CMOVNO reg64 reg/mem64,0F 41 /r
            CMOVB reg16 reg/mem16,0F 42 /r
            CMOVB reg32 reg/mem32,0F 42 /r
            CMOVB reg64 reg/mem64,0F 42 /r
            CMOVC reg16 reg/mem16,0F 42 /r
            CMOVC reg32 reg/mem32,0F 42 /r
            CMOVC reg64 reg/mem64,0F 42 /r
            CMOVNAE reg16 reg/mem16,0F 42 /r
            CMOVNAE reg32 reg/mem32,0F 42 /r
            CMOVNAE reg64 reg/mem64,0F 42 /r
            CMOVNB reg16 reg/mem16,0F 43 /r
            CMOVNB reg32 reg/mem32,0F 43 /r
            CMOVNB reg64 reg/mem64,0F 43 /r
            CMOVNC reg16 reg/mem16,0F 43 /r
            CMOVNC reg32 reg/mem32,0F 43 /r
            CMOVNC reg64 reg/mem64,0F 43 /r

            CMP AL imm8,3C ib
            CMP AX imm16,3D iw
            CMP EAX imm32,3D id
            CMP RAX imm32,3D id
            CMP reg/mem8 imm8,80 /7 ib
            CMP reg/mem16 imm16,81 /7 iw
            CMP reg/mem32 imm32,81 /7 id
            CMP reg/mem64 imm32,81 /7 id
            CMP reg/mem16 imm8,83 /7 ib
            CMP reg/mem32 imm8,83 /7 ib
            CMP reg/mem64 imm8,83 /7 ib
            CMP reg/mem8 reg8,38 /r
            CMP reg/mem16 reg16,39 /r
            CMP reg/mem32 reg32,39 /r
            CMP reg/mem64 reg64,39 /r
            CMP reg8 reg/mem8,3A /r
            CMP reg16 reg/mem16,3B /r
            CMP reg32 reg/mem32,3B /r
            CMP reg64 reg/mem64,3B /r

            CMPS AL,A6
            CMPS AX,A7
            CMPS EAX,A7
            CMPS RAX,A7

            CPUID,0F A2

            CRC32 reg32 reg/mem8,F2 0F 38 F0 /r
            CRC32 reg32 reg/mem16,F2 0F 38 F1 /r
            CRC32 reg32 reg/mem32,F2 0F 38 F1 /r
            CRC32 reg64 reg/mem8,F2 0F 38 F0 /r
            CRC32 reg64 reg/mem64,F2 0F 38 F1 /r

            CMPXCHG reg/mem8 reg8,0F B0 /r
            CMPXCHG reg/mem16 reg16,0F B1 /r
            CMPXCHG reg/mem32 reg32,0F B1 /r
            CMPXCHG reg/mem64 reg64,0F B1 /r

            DEC reg/mem8,FE /1
            DEC reg/mem16,FF /1
            DEC reg/mem32,FF /1
            DEC reg/mem64,FF /1
            DEC reg16,48 +rw
            DEC reg32,48 +rd

            DIV reg/mem8,F6 /6
            DIV reg/mem16,F7 /6
            DIV reg/mem32,F7 /6
            DIV reg/mem64,F7 /6

            ENTER imm16 imm8,C8 iw ib

            IDIV reg/mem8,F6 /7
            IDIV reg/mem16,F7 /7
            IDIV reg/mem32,F7 /7
            IDIV reg/mem64,F7 /7

            IMUL reg/mem8,F6 /5
            IMUL reg/mem16,F7 /5
            IMUL reg/mem32,F7 /5
            IMUL reg/mem64,F7 /5
            IMUL reg16 reg/mem16,0F AF /r
            IMUL reg32 reg/mem32,0F AF /r
            IMUL reg64 reg/mem64,0F AF /r
            IMUL reg16 reg/mem16 imm8,6B /r ib

            IN AL imm8,E4 ib
            IN EAX imm8,E5 ib
            IN AL DX,EC
            IN AX DX,ED
            IN EAX DX,ED

            INC reg/mem8,FE /0
            INC reg/mem16,FF /0
            INC reg/mem32,FF /0
            INC reg/mem64,FF /0
            INC reg16,40 +rw
            INC reg32,40 +rd

            INS mem8 DX,6C
            INS mem16 DX,6D
            INS mem32 DX,6D
            INSB,6C
            INSW,6D
            INSD,6D

            INT imm8,CD ib

            JO rel8,70 ib
            JO rel16,0F 80 iw
            JO rel32,0F 80 id

            JNO rel8,71 ib
            JNO rel16,0F 81 iw
            JNO rel32,0F 81 id

            JB rel8,72 ib
            JB rel16,0F 82 iw
            JB rel32,0F 82 id

            JC rel8,72 cb
            JC rel16,0F 82 cw
            JC rel32,0F 82 cd

            JNAE rel8,72 cb
            JNAE rel16,0F 82 cw
            JNAE rel32,0F 82 cd

            JNB rel8, 73 cb
            JNB rel16,0F 83 cw
            JNB rel32,0F 83 cd

            JNC rel8off,73 cb
            JNC rel16off,0F 83 cw
            JNC rel32off,0F 83 cd

            JAE rel8off,73 cb
            JAE rel16off,0F 83 cw
            JAE rel32off,0F 83 cd

            JZ rel8off,74 cb
            JZ rel16off,0F 84 cw
            JZ rel32off,0F 84 cd

            JE rel8off,74 cb
            JE rel16off,0F 84 cw
            JE rel32off,0F 84 cd

            JNZ rel8off,75 cb
            JNZ rel16off,0F 85 cw
            JNZ rel32off,0F 85 cd

            JNE rel8off,75 cb
            JNE rel16off,0F 85 cw
            JNE rel32off,0F 85 cd

            JBE rel8off,76 cb
            JBE rel16off,0F 86 cw
            JBE rel32off,0F 86 cd

            JNA rel8off,76 cb
            JNA rel16off,0F 86 cw
            JNA rel32off,0F 86 cd

            JNBE rel8off,77 cb
            JNBE rel16off,0F 87 cw
            JNBE rel32off,0F 87 cd

            JA rel8off,77 cb
            JA rel16off,0F 87 cw
            JA rel32off,0F 87 cd

            JS rel8off,78 cb
            JS rel16off,0F 88 cw
            JS rel32off,0F 88 cd

            JNS rel8off,79 cb
            JNS rel16off,0F 89 cw
            JNS rel32off,0F 89 cd

            JP rel8off,7A cb
            JP rel16off,0F 8A cw
            JP rel32off,0F 8A cd

            JPE rel8off,7A cb
            JPE rel16off,0F 8A cw
            JPE rel32off,0F 8A cd

            JNP rel8off,7B cb
            JNP rel16off,0F 8B cw
            JNP rel32off,0F 8B cd

            JPO rel8off,7B cb
            JPO rel16off,0F 8B cw
            JPO rel32off,0F 8B cd

            JL rel8off,7C cb
            JL rel16off,0F 8C cw
            JL rel32off,0F 8C cd
            JNGE rel8off,7C cb
            JNGE rel16off,0F 8C cw
            JNGE rel32off,0F 8C cd

            JNL rel8off,7D cb
            JNL rel16off,0F 8D cw
            JNL rel32off,0F 8D cd

            JGE rel8off,7D cb
            JGE rel16off,0F 8D cw
            JGE rel32off,0F 8D cd

            JLE rel8off,7E cb
            JLE rel16off,0F 8E cw
            JLE rel32off,0F 8E cd

            JNG rel8off,7E cb
            JNG rel16off,0F 8E cw
            JNG rel32off,0F 8E cd

            JNLE rel8off,7F cb
            JNLE rel16off,0F 8F cw
            JNLE rel32off,0F 8F cd

            JG rel8off,7F cb
            JG rel16off,0F 8F cw
            JG rel32off,0F 8F cd

            JMP rel8off,EB
            JMP rel16off,E9 cw
            JMP rel32off,E9 cd
            JMP reg/mem16,FF /4
            JMP reg/mem32,FF /4
            JMP reg/mem64,FF /4

            LAHF,9F

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
                vec.push(
                    line.parse()
                        .expect(&format!("internal error: \"{}\"", line)),
                );
            }
        }
        vec
    }
}

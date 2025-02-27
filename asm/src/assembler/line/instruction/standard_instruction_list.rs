use super::Instruction;

impl Instruction {
    pub const STANDARD: &str = "
            ADC al imm8,14 ib
            ADC ax imm16,15 iw
            ADC eax imm32,15 id
            ADC rax imm32,15 id
            ADC reg/mem16 imm8,83 /2 ib
            ADC reg/mem32 imm8,83 /2 ib
            ADC reg/mem64 imm8,83 /2 ib
            ADC reg/mem8 imm8,80 /2 ib
            ADC reg/mem16 imm16,81 /2 iw
            ADC reg/mem32 imm32,81 /2 id
            ADC reg/mem64 imm32,81 /2 id
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
            ADD reg/mem16 imm8,83 /0 ib
            ADD reg/mem32 imm8,83 /0 ib
            ADD reg/mem64 imm8,83 /0 ib
            ADD reg/mem8 imm8,80 /0 ib
            ADD reg/mem16 imm16,81 /0 iw
            ADD reg/mem32 imm32,81 /0 id
            ADD reg/mem64 imm32,81 /0 id
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
            AND reg/mem16 imm8,83 /4 ib
            AND reg/mem32 imm8,83 /4 ib
            AND reg/mem64 imm8,83 /4 ib
            AND reg/mem8 imm8,80 /4 ib
            AND reg/mem16 imm16,81 /4 iw
            AND reg/mem32 imm32,81 /4 id
            AND reg/mem64 imm32,81 /4 id
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

            CALL rel16,E8 iw oq
            CALL rel32,E8 id oq
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
            CMP reg/mem16 imm8,83 /7 ib
            CMP reg/mem32 imm8,83 /7 ib
            CMP reg/mem64 imm8,83 /7 ib
            CMP reg/mem8 imm8,80 /7 ib
            CMP reg/mem16 imm16,81 /7 iw
            CMP reg/mem32 imm32,81 /7 id
            CMP reg/mem64 imm32,81 /7 id
            CMP reg/mem8 reg8,38 /r
            CMP reg/mem16 reg16,39 /r
            CMP reg/mem32 reg32,39 /r
            CMP reg/mem64 reg64,39 /r
            CMP reg8 reg/mem8,3A /r
            CMP reg16 reg/mem16,3B /r
            CMP reg32 reg/mem32,3B /r
            CMP reg64 reg/mem64,3B /r

            CMPS B,A6
            CMPS W,A7
            CMPS D,A7
            CMPS Q,A7

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

            ENTER imm16 imm8,C8 iw ib oq

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

            JO rel8,70 ib oq
            JO rel16,0F 80 iw oq
            JO rel32,0F 80 id oq

            JNO rel8,71 ib oq
            JNO rel16,0F 81 iw oq
            JNO rel32,0F 81 id oq

            JB rel8,72 ib oq
            JB rel16,0F 82 iw oq
            JB rel32,0F 82 id oq

            JC rel8,72 cb oq
            JC rel16,0F 82 cw oq
            JC rel32,0F 82 cd oq

            JNAE rel8,72 cb oq
            JNAE rel16,0F 82 cw oq
            JNAE rel32,0F 82 cd oq

            JNB rel8, 73 cb oq
            JNB rel16,0F 83 cw oq
            JNB rel32,0F 83 cd oq

            JNC rel8off,73 cb oq
            JNC rel16off,0F 83 cw oq
            JNC rel32off,0F 83 cd oq

            JAE rel8off,73 cb oq
            JAE rel16off,0F 83 cw oq
            JAE rel32off,0F 83 cd oq

            JZ rel8off,74 cb oq
            JZ rel16off,0F 84 cw oq
            JZ rel32off,0F 84 cd oq

            JE rel8off,74 cb oq
            JE rel16off,0F 84 cw oq
            JE rel32off,0F 84 cd oq

            JNZ rel8off,75 cb oq
            JNZ rel16off,0F 85 cw oq
            JNZ rel32off,0F 85 cd oq

            JNE rel8off,75 cb oq
            JNE rel16off,0F 85 cw oq
            JNE rel32off,0F 85 cd oq

            JBE rel8off,76 cb oq
            JBE rel16off,0F 86 cw oq
            JBE rel32off,0F 86 cd oq

            JNA rel8off,76 cb oq
            JNA rel16off,0F 86 cw oq
            JNA rel32off,0F 86 cd oq

            JNBE rel8off,77 cb oq
            JNBE rel16off,0F 87 cw oq
            JNBE rel32off,0F 87 cd oq

            JA rel8off,77 cb oq
            JA rel16off,0F 87 cw oq
            JA rel32off,0F 87 cd oq

            JS rel8off,78 cb oq
            JS rel16off,0F 88 cw oq
            JS rel32off,0F 88 cd oq

            JNS rel8off,79 cb oq
            JNS rel16off,0F 89 cw oq
            JNS rel32off,0F 89 cd oq

            JP rel8off,7A cb oq
            JP rel16off,0F 8A cw oq
            JP rel32off,0F 8A cd oq

            JPE rel8off,7A cb oq
            JPE rel16off,0F 8A cw oq
            JPE rel32off,0F 8A cd oq

            JNP rel8off,7B cb oq
            JNP rel16off,0F 8B cw oq
            JNP rel32off,0F 8B cd oq

            JPO rel8off,7B cb oq
            JPO rel16off,0F 8B cw oq
            JPO rel32off,0F 8B cd oq

            JL rel8off,7C cb oq
            JL rel16off,0F 8C cw oq
            JL rel32off,0F 8C cd oq

            JNGE rel8off,7C cb oq
            JNGE rel16off,0F 8C cw oq
            JNGE rel32off,0F 8C cd oq

            JNL rel8off,7D cb oq
            JNL rel16off,0F 8D cw oq
            JNL rel32off,0F 8D cd oq

            JGE rel8off,7D cb oq
            JGE rel16off,0F 8D cw oq
            JGE rel32off,0F 8D cd oq

            JLE rel8off,7E cb oq
            JLE rel16off,0F 8E cw oq
            JLE rel32off,0F 8E cd oq

            JNG rel8off,7E cb oq
            JNG rel16off,0F 8E cw oq
            JNG rel32off,0F 8E cd oq

            JNLE rel8off,7F cb oq
            JNLE rel16off,0F 8F cw oq
            JNLE rel32off,0F 8F cd oq

            JG rel8off,7F cb oq
            JG rel16off,0F 8F cw oq
            JG rel32off,0F 8F cd oq

            JMP rel8off,EB oq
            JMP rel16off,E9 cw oq
            JMP rel32off,E9 cd oq
            JMP reg/mem64,FF /4 oq

            LAHF,9F

            LEA reg16 reg/mem16,8D /r
            LEA reg32 reg/mem32,8D /r
            LEA reg64 reg/mem64,8D /r
            
            LEAVE,C9 oq

            LFENCE,0F AE E8

            LODS B,AC
            LODS W,AD
            LODS D,AD
            LODS Q,AD

            LOOP rel8off,E2 cb oq
            LOOPNE rel8off,E0 cb oq
            LOOPNZ rel8off,E0 cb oq
            LOOPZ rel8off,E1 cb oq

            LZCNT reg16 reg/mem16,F3 0F BD /r
            LZCNT reg32 reg/mem32,F3 0F BD /r
            LZCNT reg64 reg/mem64,F3 0F BD /r

            MCOMMIT,F3 0F 01 FA

            MFENCE,0F AE F0

            MONITORX,0F 01 FA

            MOV reg/mem8 reg8,88 /r
            MOV reg/mem16 reg16,89 /r
            MOV reg/mem32 reg32,89 /r
            MOV reg/mem64 reg64,89 /r
            MOV reg8 reg/mem8,8A /r
            MOV reg16 reg/mem16,8B /r
            MOV reg32 reg/mem32,8B /r
            MOV reg64 reg/mem64,8B /r
            MOV reg/mem8 imm8,C6 /0 ib
            MOV reg/mem16 imm16,C7 /0 iw
            MOV reg/mem32 imm32,C7 /0 id
            MOV reg/mem64 imm32,C7 /0 id
            MOV reg8 imm8,B0 +rb ib
            MOV reg16 imm16,B8 +rw iw
            MOV reg32 imm32,B8 +rd id
            MOV reg64 imm64,B8 +rq iq            

            MOVBE reg16 mem16,0F 38 F0 /r
            MOVBE reg32 mem32,0F 38 F0 /r
            MOVBE reg64 mem64,0F 38 F0 /r
            MOVBE mem16 reg16,0F 38 F1 /r
            MOVBE mem32 reg32,0F 38 F1 /r
            MOVBE mem64 reg64,0F 38 F1 /r

            MOVNTI mem32 reg32,0F C3 /r
            MOVNTI mem64 reg64,0F C3 /r

            MOVS B,A4
            MOVS W,A5
            MOVS D,A5
            MOVS Q,A5

            MOVSX reg16 reg/mem8,0F BE /r
            MOVSX reg32 reg/mem8,0F BE /r
            MOVSX reg64 reg/mem8,0F BE /r
            MOVSX reg32 reg/mem16,0F BF /r
            MOVSX reg64 reg/mem16,0F BF /r

            MOVSXD reg64 reg/mem32,63 /r

            MOVZX reg16 reg/mem8,0F B6 /r
            MOVZX reg32 reg/mem8,0F B6 /r
            MOVZX reg64 reg/mem8,0F B6 /r
            MOVZX reg32 reg/mem16,0F B7 /r
            MOVZX reg64 reg/mem16,0F B7 /r

            MUL reg/mem8,F6 /4
            MUL reg/mem16,F7 /4
            MUL reg/mem32,F7 /4
            MUL reg/mem64,F7 /4

            MWAITX,0F 01 FB

            NEG reg/mem8,F6 /3
            NEG reg/mem16,F7 /3
            NEG reg/mem32,F7 /3
            NEG reg/mem64,F7 /3

            NOP,90
            NOP reg/mem16,0F 1F /0
            NOP reg/mem32,0F 1F /0
            NOP reg/mem64,0F 1F /0

            NOT reg/mem8,F6 /2
            NOT reg/mem16,F7 /2
            NOT reg/mem32,F7 /2
            NOT reg/mem64,F7 /2

            OR AL imm8,0C ib
            OR AX imm16,0D iw
            OR EAX imm32,0D id
            OR RAX imm32,0D id
            OR reg/mem8 imm8,80 /1 ib
            OR reg/mem16 imm16,81 /1 iw
            OR reg/mem32 imm32,81 /1 id
            OR reg/mem64 imm32,81 /1 id
            OR reg/mem16 imm8,83 /1 ib
            OR reg/mem32 imm8,83 /1 ib
            OR reg/mem64 imm8,83 /1 ib
            OR reg/mem8 reg8,08 /r
            OR reg/mem16 reg16,09 /r
            OR reg/mem32 reg32,09 /r
            OR reg/mem64 reg64,09 /r

            OUT imm8 AL,E6 ib
            OUT imm8 AX,E7 ib
            OUT imm8 EAX,E7 ib
            OUT DX AL,EE
            OUT DX AX,EF
            OUT DX EAX,EF

            OUTS B,6E
            OUTS W,6F
            OUTS D,6F

            PAUSE,F3 90

            POP reg/mem64,8F /0 oq
            POP reg64,58 +rq oq

            POPCNT reg16 reg/mem16,F3 0F B8 /r
            POPCNT reg32 reg/mem32,F3 0F B8 /r
            POPCNT reg64 reg/mem64,F3 0F B8 /r

            POPF,9D
            POPF D,9D
            POPF Q,9D

            PREFETCH mem8,0F 0D /0
            PREFETCHW mem8,0F 0D /1

            PREFETCHNTA mem8,0F 18 /0
            PREFETCHT0 mem8,0F 18 /1
            PREFETCHT1 mem8,0F 18 /2
            PREFETCHT2 mem8,0F 18 /3

            PUSH reg/mem64,FF /6 oq
            PUSH reg64,50 +rq oq
            PUSH imm64,68 id

            PUSHF Q,9C oq

            RCL reg/mem8 imm8,C0 /2 ib
            RCL reg/mem16 imm8,C1 /2 ib
            RCL reg/mem32 imm8,C1 /2 ib
            RCL reg/mem64 imm8,C1 /2 ib

            RCR reg/mem8 imm8,C0 /3 ib
            RCR reg/mem16 imm8,C1 /3 ib
            RCR reg/mem32 imm8,C1 /3 ib
            RCR reg/mem64 imm8,C1 /3 ib

            RDFSBASE reg32,F3 0F AE /0
            RDFSBASE reg64,F3 0F AE /0
            RDGSBASE reg32,F3 0F AE /1
            RDGSBASE reg64,F3 0F AE /1

            RDPID,F3 0F C7 /7

            RDPRU,0F 01 FD

            RDRAND reg16,0F C7 /6
            RDRAND reg32,0F C7 /6
            RDRAND reg64,0F C7 /6

            RDSEED reg16,0F C7 /7
            RDSEED reg32,0F C7 /7
            RDSEED reg64,0F C7 /7

            RET,C3 oq
            RET imm16,C2 iw oq

            RETF,CB
            RETF imm16,CA iw

            ROL reg/mem8 imm8,C0 /0 ib
            ROL reg/mem16 imm8,C1 /0 ib
            ROL reg/mem32 imm8,C1 /0 ib
            ROL reg/mem64 imm8,C1 /0 ib

            ROR reg/mem8 imm8,C0 /1 ib
            ROR reg/mem16 imm8,C1 /1 ib
            ROR reg/mem32 imm8,C1 /1 ib
            ROR reg/mem64 imm8,C1 /1 ib

            SAHF,9E

            SAL reg/mem8 imm8,C0 /4 ib
            SAL reg/mem16 imm8,C1 /4 ib
            SAL reg/mem32 imm8,C1 /4 ib
            SAL reg/mem64 imm8,C1 /4 ib

            SHL reg/mem8 imm8,C0 /4 ib
            SHL reg/mem16 imm8,C1 /4 ib
            SHL reg/mem32 imm8,C1 /4 ib
            SHL reg/mem64 imm8,C1 /4 ib

            SAR reg/mem8 imm8,C0 /7 ib
            SAR reg/mem16 imm8,C1 /7 ib
            SAR reg/mem32 imm8,C1 /7 ib
            SAR reg/mem64 imm8,C1 /7 ib

            SBB AL imm8,1C ib
            SBB AX imm16,1D iw
            SBB EAX imm32,1D id
            SBB RAX imm32,1D id
            SBB reg/mem16 imm8,83 /3 ib
            SBB reg/mem32 imm8,83 /3 ib
            SBB reg/mem64 imm8,83 /3 ib
            SBB reg/mem8 imm8,80 /3 ib
            SBB reg/mem16 imm16,81 /3 iw
            SBB reg/mem32 imm32,81 /3 id
            SBB reg/mem64 imm32,81 /3 id
            SBB reg/mem8 reg8,18 /r
            SBB reg/mem16 reg16,19 /r
            SBB reg/mem32 reg32,19 /r
            SBB reg/mem64 reg64,19 /r
            SBB reg8 reg/mem8,1A /r
            SBB reg16 reg/mem16,1B /r
            SBB reg32 reg/mem32,1B /r
            SBB reg64 reg/mem64,1B /r

            SCAS B,AE
            SCAS W,AF
            SCAS D,AF
            SCAS Q,AF

            SETO reg/mem8,0F 90 /0
            SETNO reg/mem8,0F 91 /0
            SETB reg/mem8,0F 92 /0
            SETC reg/mem8,0F 92 /0
            SETNAE reg/mem8,0F 92 /0
            SETNB reg/mem8,0F 93 /0
            SETNC reg/mem8,0F 93 /0
            SETAE reg/mem8,0F 93 /0
            SETZ reg/mem8,0F 94 /0
            SETE reg/mem8,0F 94 /0
            SETNZ reg/mem8,0F 95 /0
            SETNE reg/mem8,0F 95 /0
            SETBE reg/mem8,0F 96 /0
            SETNA reg/mem8,0F 96 /0
            SETNBE reg/mem8,0F 97 /0
            SETA reg/mem8,0F 97 /0
            SETS reg/mem8,0F 98 /0
            SETNS reg/mem8,0F 99 /0
            SETPE reg/mem8,0F 9A /0
            SETNP reg/mem8,0F 9B /0
            SETPO reg/mem8,0F 9B /0
            SETL reg/mem8,0F 9C /0
            SETNGE reg/mem8,0F 9C /0
            SETNL reg/mem8,0F 9D /0
            SETGE reg/mem8,0F 9D /0
            SETLE reg/mem8,0F 9E /0
            SETNG reg/mem8,0F 9E /0
            SETNLE reg/mem8,0F 9F /0
            SETG reg/mem8,0F 9F /0

            SFENCE,0F AE F8

            SHLD reg/mem16 reg16 imm8,0F A4 /r ib
            SHLD reg/mem32 reg32 imm8,0F A4 /r ib
            SHLD reg/mem64 reg64 imm8,0F A4 /r ib

            SHR reg/mem8 imm8,C0 /5 ib
            SHR reg/mem16 imm8,C1 /5 ib
            SHR reg/mem32 imm8,C1 /5 ib
            SHR reg/mem64 imm8,C1 /5 ib

            SHRD reg/mem16 reg16 imm8,0F AC /r ib
            SHRD reg/mem32 reg32 imm8,0F AC /r ib
            SHRD reg/mem64 reg64 imm8,0F AC /r ib

            STC,F9

            STD,FD

            STOS B,AA
            STOS W,AB
            STOS D,AB
            STOS Q,AB

            SUB AL imm8,2C ib
            SUB AX imm16,2D iw
            SUB EAX imm32,2D id
            SUB RAX imm32,2D id
            SUB reg/mem16 imm8,83 /5 ib
            SUB reg/mem32 imm8,83 /5 ib
            SUB reg/mem64 imm8,83 /5 ib
            SUB reg/mem8 imm8,80 /5 ib
            SUB reg/mem16 imm16,81 /5 iw
            SUB reg/mem32 imm32,81 /5 id
            SUB reg/mem64 imm32,81 /5 id
            SUB reg/mem8 reg8,28 /r
            SUB reg/mem16 reg16,29 /r
            SUB reg/mem32 reg32,29 /r
            SUB reg/mem64 reg64,29 /r
            SUB reg8 reg/mem8,2A /r
            SUB reg16 reg/mem16,2B /r
            SUB reg32 reg/mem32,2B /r
            SUB reg64 reg/mem64,2B /r

            TEST AL imm8,A8 ib
            TEST AX imm16,A9 iw
            TEST EAX imm32,A9 id
            TEST RAX imm32,A9 id
            TEST reg/mem8 imm8,F6 /0 ib
            TEST reg/mem16 imm16,F7 /0 iw
            TEST reg/mem32 imm32,F7 /0 id
            TEST reg/mem64 imm32,F7 /0 id
            TEST reg/mem8 reg8,84 /r
            TEST reg/mem16 reg16,85 /r
            TEST reg/mem32 reg32,85 /r
            TEST reg/mem64 reg64,85 /r

            TZCNTreg16 reg/mem16,F3 0F BC /r
            TZCNTreg32 reg/mem32,F3 0F BC /r
            TZCNTreg64 reg/mem64,F3 0F BC /r

            UD0,0F FF
            UD1 reg/mem64 reg64,0F B9 /r
            UD2,0F 0B

            WRFSBASE reg32,F3 0F AE /2
            WRFSBASE reg64,F3 0F AE /2
            WRGSBASE reg32,F3 0F AE /3
            WRGSBASE reg64,F3 0F AE /3

            XADD reg/mem8 reg8,0F C0 /r
            XADD reg/mem16 reg16,0F C1 /r
            XADD reg/mem32 reg32,0F C1 /r
            XADD reg/mem64 reg64,0F C1 /r

            XCHG AX reg16,90 +rw
            XCHG reg16 AX,90 +rw
            XCHG EAX reg32,90 +rd
            XCHG reg32 EAX,90 +rd
            XCHG RAX reg64,90 +rq
            XCHG reg64 RAX,90 +rq
            XCHG reg/mem8 reg8,86 /r
            XCHG reg8 reg/mem8,86 /r
            XCHG reg/mem16 reg16,87 /r
            XCHG reg16 reg/mem16,87 /r
            XCHG reg/mem32 reg32,87 /r
            XCHG reg32 reg/mem32,87 /r
            XCHG reg/mem64 reg64,87 /r
            XCHG reg64 reg/mem64,87 /r

            XLAT B,D7

            XOR AL imm8,34 ib
            XOR AX imm16,35 iw
            XOR EAX imm32,35 id
            XOR RAX imm32,35 id
            XOR reg/mem8 imm8,80 /6 ib
            XOR reg/mem16 imm16,81 /6 iw
            XOR reg/mem32 imm32,81 /6 id
            XOR reg/mem64 imm32,81 /6 id
            XOR reg/mem16 imm8,83 /6 ib
            XOR reg/mem32 imm8,83 /6 ib
            XOR reg/mem64 imm8,83 /6 ib
            XOR reg/mem8 reg8,30 /r
            XOR reg/mem16 reg16,31 /r
            XOR reg/mem32 reg32,31 /r
            XOR reg/mem64 reg64,31 /r
            XOR reg8 reg/mem8,32 /r
            XOR reg16 reg/mem16,33 /r
            XOR reg32 reg/mem32,33 /r
            XOR reg64 reg/mem64,33 /r

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

use super::*;

//4c 89 24 24          	mov    %r12,(%rsp)
#[test]
fn mov_r12_ref_rsp() {
    assert_eq!(
        Ok(SVec::from([0x4c, 0x89, 0x24, 0x24])),
        encode(
            SVec::from([0x89]),
            Operand::Reg64Rm64(
                Register::R12,
                Rm::Ref {
                    base: Register::Rsp,
                    scale: 0,
                    index: Register::Rax,
                    disp: 0,
                }
            )
        )
    );
}

//48 89 fe             	mov    %rdi,%rsi
#[test]
fn mov_rsi_rdi() {
    assert_eq!(
        Ok(SVec::from([0x48, 0x89, 0xfe])),
        encode(
            SVec::from([0x89]),
            Operand::Reg64Rm64(Register::Rdi, Rm::Reg(Register::Rsi))
        )
    );
}

// 48 83 c4 08          	add    $0x8,%rsp
#[test]
fn add_rsp_8() {
    assert_eq!(
        Ok(SVec::from([0x48, 0x83, 0xc4, 0x08])),
        encode(
            SVec::from([0x83]),
            Operand::Rm64Imm8(Rm::Reg(Register::Rsp), 0x8)
        )
    );
}

// 48 8b 3d 06 2f 00 00 	mov    0x2f06(%rip),%rdi
#[test]
fn mov_rdi_ref_rip_2f06() {
    assert_eq!(
        Ok(SVec::from([0x48, 0x8b, 0x3d, 0x06, 0x2f, 0x00, 0x00])),
        encode(
            SVec::from([0x8b]),
            Operand::Reg64Rm64(Register::Rdi, Rm::Ref {
                base: Register::Rip,
                scale: 0,
                index: Register::Rax,
                disp: 0x2f06,
            })
        )
    )
}

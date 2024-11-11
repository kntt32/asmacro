use super::*;
use ml_generator::*;
use raw_encoder::*;
use registers::*;
use util::svec::SVec;

// ff d0                	call   *%rax
#[test]
fn call_rax() {
    assert_eq!(
        Ok(SVec::from([0xff, 0xd0])),
        raw_encode(
            SVec::from([0xff]),
            RexMode::None,
            ModRmMode::Dight(2, Rm::Reg(Register::Rax)),
            ImmMode::None,
            AddRegMode::None
        )
    )
}

// ff 35 a2 2f 00 00    	push   0x2fa2(%rip)
#[test]
fn push_ref_rip_0x2fa2() {
    assert_eq!(
        Ok(SVec::from([0xff, 0x35, 0xa2, 0x2f, 0x00, 0x00])),
        raw_encode(
            SVec::from([0xff]),
            RexMode::None,
            ModRmMode::Dight(
                6,
                Rm::Ref {
                    scale: 0, // 0, 1, 2, 4, 8
                    index: Register::Rax,
                    base: Register::Rip,
                    disp: 0x2fa2,
                }
            ),
            ImmMode::None,
            AddRegMode::None,
        )
    )
}

// 48 89 e2             	mov    %rsp,%rdx
// REX.W + 89 /rMOV r/m64,r64
#[test]
fn mov_rdx_rsp() {
    assert_eq!(
        Ok(SVec::from([0x48, 0x89, 0xe2])),
        raw_encode(
            SVec::from([0x89]),
            RexMode::RexW,
            ModRmMode::R(Register::Rsp, Rm::Reg(Register::Rdx)),
            ImmMode::None,
            AddRegMode::None,
        )
    )
}

//  48 83 e4 f0          	and    $0xfffffffffffffff0,%rsp
// REX.W + 83 /4 ib   AND r/m64,imm8
#[test]
fn and_rsp_imm() {
    assert_eq!(
        Ok(SVec::from([0x48, 0x83, 0xe4, 0xf0])),
        raw_encode(
            SVec::from([0x83]),
            RexMode::RexW,
            ModRmMode::Dight(4, Rm::Reg(Register::Rsp)),
            ImmMode::Ib(-16),
            AddRegMode::None,
        )
    )
}

// 48 85 c0             	test   %rax,%rax
// REX.W + 85 /rTEST r/m64,r64
#[test]
fn test_rax_rax() {
    assert_eq!(
        Ok(SVec::from([0x48, 0x85, 0xc0])),
        raw_encode(
            SVec::from([0x85]),
            RexMode::RexW,
            ModRmMode::R(Register::Rax, Rm::Reg(Register::Rax)),
            ImmMode::None,
            AddRegMode::None,
        )
    )
}

//

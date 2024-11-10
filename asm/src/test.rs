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

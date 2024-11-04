use asm::ml_generator::*;
use util::dyn_fn::DynFn;
use util::svec::SVec;

fn main() {
    let code: &[u8] = &[
        0b01001000, 0xb8, 123, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, // mov rax, 123
        0xc3, // ret
    ];
    println!("{:?}", code);

    let dynfn = DynFn::<(), u64>::new(code);
    unsafe {
        println!("{}", dynfn.call(()));
    }
    /*
    let mut code = Encoder::new();

    code.rex_prefix.enable();
    code.rex_prefix.set_w(true);
    code.rex_prefix.set_r(false);
    code.opecode.push(0xb8 + 0);
    code.imm = Imm::Imm64(123);

    println!("code: {}", code.encode());

    let mut opds = OpDescpritor::new(true, SVec::<3, u8>::from(&[0x8b][0 .. 1]), Operand::R64Rm64(Reg64::Rax, Rm64::R64(Reg64::Rdi))); // mov rax, rdi
    let mut ml = opds.encode().unwrap().as_vec();
    opds = OpDescpritor::new(true, SVec::<3, u8>::from(&[0x03][0 .. 1]), Operand::R64Rm64(Reg64::Rax, Rm64::R64(Reg64::Rsi))); // add rax, rsi
    let mlvec = [ml, opds.encode().unwrap().as_vec()].concat();

    opds = OpDescpritor::new(false, SVec::<3, u8>::from(&[0xc3][0 .. 1]), Operand::None); // ret
    let mlvec = [mlvec, opds.encode().unwrap().as_vec()].concat();
    println!("{:?}", mlvec);

    let dynfn = DynFn::<(u64, u64), u64>::new(&mlvec);
    unsafe {
        println!("{}", dynfn.call((1, 3))); // 4
    }
    */

    let mut ml_gen = MlGen::new();
    ml_gen.rex_prefix.enable();
    ml_gen.rex_prefix.set_w(true);
    ml_gen.opecode.set(SVec::from([0xb8]), Some(0));
    ml_gen.imm = Imm::Imm64(123);
    println!("code: {}", ml_gen.encode());
}

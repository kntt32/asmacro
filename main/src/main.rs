use util::dyn_fn::DynFn;
use asm::encoder::{Code, Imm};

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

    let mut code = Code::new();

    code.rex_prefix.enable();
    code.rex_prefix.set_w(true);
    code.rex_prefix.set_r(false);
    code.opecode.push(0xb8 + 0);
    code.imm = Imm::Imm64(123);

    println!("code: {}", code.encode());
}

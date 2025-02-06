//use asm::assembler::Asm;
// use util::dyn_fn::DynFn;
use asm::assembler::{line::instruction::Instruction, line::Line};
use asm::linker::object::Object;

fn main() {
    let ins: Instruction = "ADC reg/mem64 reg64 , 11 /r".parse().unwrap();
    let ret: Instruction = "RET,C3".parse().unwrap();
    println!("{:?}", ins);
    let code = "ret";
    let line = Line::Instruction(code);
    println!("{:?}, {:?}", line.mnemonic(), line.operands());
    println!("{:?}", ret.match_with(&Line::Instruction(code)));

    let mut object = Object::new();
    println!("{:?}", line.encode(&mut object, &[], &[ins, ret]));
    for i in object.code {
        print!("{:x} ", i);
    }
    println!("");
}

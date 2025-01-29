//use asm::assembler::Asm;
// use util::dyn_fn::DynFn;
use asm::line::instruction::Instruction;
use asm::line::Line;

fn main() {
    let ins: Instruction = "ADC reg/mem64 reg64 , 11 /r".parse().unwrap();
    println!("{:?}", ins);
    let code = "adc 5[rax]q rdi";
    let line = Line::Instruction(code);
    println!("{:?}, {:?}", line.mnemonic(), line.operands());
    println!("{:?}", ins.match_with(&Line::Instruction(code)));
}

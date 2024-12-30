use asm::instruction::OperandType;
use asm::parser::Parser;

fn main() {
    let source = "
.text
main:
    push rbp
    mov rbp rsp
    
    push 1
    mov [rsp] 0
    pop rax

    mov rsp rbp
    pop rbp
    ret";

    let parser = Parser::new(source);

    for line in parser {
        if let Some(n) = line.get_instruction() {
            println!("{:?}", n.mnemonic());
        } else {
            println!("");
        }
    }
}

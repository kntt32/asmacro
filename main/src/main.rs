use asm::parser::*;
use util::functions::*;

fn main() {
    let parse = Parser::new(
        "
main:
    push rbp
    mov rbp, rsp

    mov rax, 0
    
    mov rsp, rbp
    pop rbp
    ret
    ",
    );

    for i in parse {
        println!("{:?}", i);
    }
}

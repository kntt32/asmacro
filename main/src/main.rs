use asm::line::operators;
use asm::parser::*;
use util::functions::*;

fn main() {
    let parse = Parser::new(
        "
main:
    push rbp
    mov rbp, rsp

    push 1
    push 2
    mov rdi, -1[rsp]
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

use asm::line::operators;
use asm::parser::*;
use util::functions::*;

fn main() {
    let parse = Parser::new(
        "
main:
    push rbp
    mov rbp, rsp

    mov rdi, -1[rsp]
    mov rax, 0
    
    mov rsp, rbp
    pop rbp
    ret
    ",
    );

    for line in parse {
        println!("{:?}", line.unwrap().1.modrm_rm_ref_base());
    }
}

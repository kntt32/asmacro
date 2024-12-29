use asm::parser::Parser;

fn main() {
    let source = "
.text
main:
    push rbp
    mov rbp, rsp
    
    mov rax, 0
    mov rsp, rbp
    pop rbp
    ret";

    let parser = Parser::new(source);

    for line in parser {
        println!("{:?}", line);
    }
}

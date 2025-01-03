use asm::parser::Parser;
use util::svec::SVec;
fn main() {
    /*
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
    */
    //bf 02 00 00 00       	mov    $0x2,%edi
    let code = "mov edi 0x2";
    let parser = Parser::new(code);
    for line in parser {
        println!("{}", line.imm());
        println!("{:x}", line.machine_code());
    }
}

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

    // 48 89 7c 24 10       	mov    %rdi,0x10(%rsp)
    let code = "mov 0x10[rsp] rdi";
    let parser = Parser::new(code);
    for line in parser {
        //println!("{:?}", line.modrm_scale());
        println!("{:x}", line.machine_code());
    }
}

use asm::assembler::Asm;
use asm::parser::Parser;

fn main() {
    let source = "
    main:
        push rbp
        mov rbp rsp

        push 1
        mov [rsp]q 0
        pop rax

        mov rsp rbp
        pop rbp
        ret
    runtime:
        call main";

    let asm = Asm::new(source);
    println!("{:?}", asm.labels());
    print!("[");
    for i in asm.assemble().unwrap() {
        print!("{:x}, ", i);
    }
    println!("]");
    //    println!("{:x}", asm.assemble().unwrap());

    // 48 89 7c 24 10       	mov    %rdi,0x10(%rsp)
    let code = "mov 0x10[rsp]q rdi";
    let parser = Parser::new(code);
    for line in parser {
        //println!("{:?}", line.modrm_scale());
        println!("{:x}", line.machine_code(&[], 0).unwrap());
    }
}

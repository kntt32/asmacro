use asm::{
    assembler::{
        line::{instruction::Instruction, pseudo::Pseudo},
        parser::Parser,
    },
    linker::object::Object,
};
use compiler::tokenizer::TokenTree;
use std::{env::args, fs::File, io::Write, path::Path, process::Command};

fn main() {
    let tt = TokenTree::new(
        "
    fn square(x: u32@eax, y: u32@edi) -> u32@eax {
        x * y
    }
    ",
    );
    println!("{:?}", tt);
}

#[allow(unused)]
fn command_interpreter() {
    let mut args = args();
    args.next();
    let Some(command) = args.next() else {
        println!(
            "
      ####     #####   # # #      ####     ###    #  ###     ###
          #   #        ## # ##        #   #   ##   ##   #   #   #
     ######    #####   #  #  #   ######  #         #       #     #
    #     #         #  #  #  #  #     #   #   ##   #        #   #
     ##### #  ######   #  #  #   ##### #   ###     #         ###

            asmacro's bootstrap preprocessor and assembler

            Usages:

                asmacro prep [file] .. ([-m path])
                asmacro asm [file] .. ([-o path])
                asmacro run [file] .. ([-o path] [-m path])
            "
        );
        return;
    };

    match &*command {
        "prep" => panic!(),
        "asm" => asm_demo(),
        "run" => {
            panic!();
            asm_demo();
        }
        _ => panic!(),
    }
}
/*
#[allow(unused)]
fn preproc_demo() {
    let code = "
    #start 12hjb asdiuer #end #cat
    #start
        test $0
    #end
    #start
        macro $
    #end
    #def

    macro a
    ";
    let macros = TokenTree::standard_macros();
    let mut tokentree = TokenTree::new(code);
    println!("{:?}", tokentree);
    tokentree.process(&macros);
    println!("{:?}", tokentree);
}*/

#[allow(unused)]
fn asm_demo() {
    let code = "
    message:
    .utf8 \"Hello, World!\\n\"
    main:
    mov rax 1
    mov rdi 1
    lea rsi message[rip]q
    mov rdx 14
    syscall
    
    mov rax 60
    mov rdi 0
    syscall
    .global main";

    let mut object = Object::new();
    let instruction = Instruction::standards();
    let pseudo = Pseudo::standards();
    for line in Parser::new(code) {
        line.encode(&mut object, &pseudo, &instruction)
            .expect("error");
    }
    for i in &object.code {
        print!("{:x} ", i);
    }
    println!("////");
    println!("{:?}", object);
    let elf = object.elf("main", false).expect("error");
    let elf_vec = elf.as_vec();
    for i in &elf_vec {
        print!("{:x} ", i);
    }
    println!("////");
    let path = Path::new("./a.out");
    let mut file = File::create(path).expect("error");
    file.write(&elf_vec).expect("error");

    Command::new("chmod")
        .args(["u+x", "a.out"])
        .output()
        .expect("failed to execute chmod");
}

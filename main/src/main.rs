use asm::{
    assembler::{
        line::{instruction::Instruction, pseudo::Pseudo},
        parser::Parser,
        register::Register,
    },
    linker::object::Object,
};
use compiler::compile;
use std::{
    env::{args, Args},
    fs::File,
    io::Read,
    io::Write,
    path::Path,
    process::Command,
    rc::Rc,
};
use util::{parser::Parser as UParser, Offset};

fn main() {
    command_interpreter();
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

            asmacro's bootstrap compiler and assembler

            Usages:

                asmacro compile [file]

                asmacro asm [file]

                asmacro build [file]

                asmacro run [file]
            "
        );
        return;
    };

    match &*command {
        "compile" => compile_cmd(args),
        "asm" => asm_demo(),
        "build" => todo!(),
        "run" => {
            panic!();
            asm_demo();
        }
        _ => panic!(),
    }
}

fn compile_cmd(mut args: Args) {
    let Some(file_name) = args.next() else {
        eprintln!("error: missing file name");
        return;
    };

    let mut file = match File::open(&file_name) {
        Ok(f) => f,
        Err(e) => {
            eprintln!("error: {:?}", e);
            return;
        }
    };
    let mut source_code = String::new();
    let Ok(_) = file.read_to_string(&mut source_code) else {
        eprintln!("error: invalid char code in the file");
        return;
    };

    match compile(&source_code) {
        Ok(asm) => {
            let asm_file_name = format!("{}.s", &file_name);
            let mut asm_file = File::create(&asm_file_name)
                .expect(&format!("failed creating \"{}\"", &asm_file_name));
            asm_file
                .write_all(asm.as_bytes())
                .expect(&format!("failed writing code to \"{}\"", &asm_file_name));
        }
        Err(e) => {
            for elm in &e {
                let (offset, msg) = elm;
                eprintln!(
                    "error: {}:{}:{}: {}",
                    &file_name, offset.row, offset.column, &msg
                );
            }
        }
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

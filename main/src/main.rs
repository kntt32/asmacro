//use asm::assembler::Asm;
// use util::dyn_fn::DynFn;
use asm::assembler::{
    line::{instruction::Instruction, pseudo::Pseudo},
    parser::Parser,
};
use asm::linker::object::Object;
use std::fs::File;
use std::io::Write;
use std::path::Path;
use std::process::Command;

fn main() {
    /*
    ; システムコールの準備
    mov rax, 1             ; 'write' システムコールのシステムコール番号（1）を設定
    mov rdi, 1             ; 標準出力へのファイルディスクリプタ（1）を設定
    mov rsi, message       ; 出力するメッセージのアドレスを設定
    mov rdx, len           ; 出力するメッセージの長さを設定

    ; システムコールの実行
    syscall
    */
    let code = "
    message:
    .utf8 \"Hello\"
    main:
    mov rax 1
    mov rdi 1
    lea rsi message[rip]q
    mov rdx 5
    syscall
    
    mov rax 60
    mov rdi 0
    syscall
    .global main";

    let mut object = Object::new();
    let instruction = Instruction::standard();
    let pseudo = Pseudo::standard();
    for line in Parser::new(code) {
        line.encode(&mut object, &pseudo, &instruction)
            .expect("error");
    }
    for i in &object.code {
        print!("{:x} ", i);
    }
    println!("////");
    println!("{:?}", object);
    let elf = object.elf("main").expect("error");
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

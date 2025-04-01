use asm::{
    assembler::{
        line::{instruction::Instruction, pseudo::Pseudo},
        parser::Parser,
        register::Register,
    },
    linker::object::Object,
};
use compiler::parser::{Parser as CParser, Token};
use compiler::syntax_analyzer::{
    Data, Lifetime, MovingStorageExpr, NumberLiteral, SyntaxNode, SyntaxTree, Variable,
    VariableAssignment, VariableDeclaration,
};
use std::{env::args, fs::File, io::Write, path::Path, process::Command};
use util::Offset;

fn main() {
    let data = Data::Some {
        r#type: "i32".to_string(),
        storage: vec![Register::Eax],
    };
    let lifetime = Lifetime::new(Offset { row: 0, column: 0 }, None);
    let variable = Variable::new("a".to_string(), data, true, lifetime);
    let expr: Box<dyn SyntaxNode> = Box::new(NumberLiteral::new(
        "1327".to_string(),
        Offset { row: 0, column: 10 },
    ));
    let variable_declaration =
        VariableDeclaration::new(variable, expr, Offset { row: 0, column: 0 });
    let variable_assignment = VariableAssignment::new(
        "a".to_string(),
        Box::new(NumberLiteral::new(
            "2927".to_string(),
            Offset { row: 10, column: 0 },
        )),
        Offset { row: 5, column: 0 },
    );
    let data_b = Data::Some {
        r#type: "i32".to_string(),
        storage: vec![Register::Edx],
    };
    let lifetime_b = Lifetime::new(Offset { row: 15, column: 0 }, None);
    let variable_b = Variable::new("b".to_string(), data_b, false, lifetime_b);
    let expr_b_literal: Box<dyn SyntaxNode> = Box::new(NumberLiteral::new(
        "32".to_string(),
        Offset { row: 15, column: 5 },
    ));
    let expr_b: Box<dyn SyntaxNode> = Box::new(MovingStorageExpr::new(
        expr_b_literal,
        vec![Register::Edx],
        Offset { row: 15, column: 5 },
    ));
    let variable_declaration_b =
        VariableDeclaration::new(variable_b, expr_b, Offset { row: 15, column: 0 });

    let tree: Vec<Box<dyn SyntaxNode>> = vec![
        Box::new(variable_declaration),
        Box::new(variable_assignment),
        Box::new(NumberLiteral::new(
            "123456".to_string(),
            Offset { row: 10, column: 0 },
        )),
        Box::new(variable_declaration_b),
    ];
    let mut syntaxtree = SyntaxTree::new(tree);
    println!("{:?}", syntaxtree.compile());
}

#[allow(unused)]
fn compiler_demo(mut parser: CParser) {
    for t in parser {
        match t {
            Token::Block {
                r#type: _,
                parser: p,
                offset: _,
            } => {
                println!("{:?}\n", t);
                compiler_demo(p);
                println!("\n");
            }
            _ => println!("{:?}", t),
        }
    }
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

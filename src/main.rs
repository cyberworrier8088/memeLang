mod ast;
mod interpreter;
mod lexer;
mod parser;
mod token;

use std::env;
use std::fs;

use interpreter::Interpreter;
use lexer::Lexer;
use parser::Parser;

fn main() {
    println!("Enter an expression:");

    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        eprintln!("Usage: memeLang <file>");
        return;
    }
    let source = fs::read_to_string(&args[1]).expect("failed to read file");


    let source = source.trim();
    if source.is_empty() {
        eprintln!("no input provided");
        return;
    }

    let source = if source.ends_with(';') {
        source.to_string()
    } else {
        format!("{source};")
    };

    let mut lexer = Lexer::new(&source);
    let tokens = lexer.tokenize().expect("lexing failed");

    let mut parser = Parser::new(tokens);
    let program = parser.parse_program().expect("parsing failed");

    let mut interpreter = Interpreter::new();
    let result = interpreter.run(&program).expect("runtime error");
    println!("result: {result:?}");
}

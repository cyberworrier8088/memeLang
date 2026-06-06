// src\main.rs


//-----------------------------------------------------------------------------------------------------------------------------------------------------------
// this is a simple and funny programming lang
// Warning: MemeLang is not designed for serious production use. It is designed for learning, experimentation, and absolute chaos.
// enojoy my code
// Thankyou
//------------------------------------------------------------------------------------------------------------------------------------------------------------

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
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        eprintln!("Usage: memeLang <file>");
        return;
    }
    let source = fs::read_to_string(&args[1]).expect("failed to read file");

    let mut lexer = Lexer::new(&source);
    let tokens = lexer.tokenize().expect("lexing failed");

    let mut parser = Parser::new(tokens);
    let program = parser.parse_program().expect("parsing failed");

    let mut interpreter = Interpreter::new();
    interpreter.run(&program).expect("runtime error");
}



///------------------------------------------------
/// END OF main.rs
///------------------------------------------------
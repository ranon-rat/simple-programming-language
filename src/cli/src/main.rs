use interpreter::Interpreter;
use lexer::{tokenize, tokens::LexerError};
use parser::{self, parse};
use std::env::args;
use std::fs;
fn main() {
    let arguments: Vec<String> = args().collect();
    if arguments.len() < 2 {
        println!("not enough arguments");
        return;
    }
    let filename = &arguments[1];
    let program = match fs::read_to_string(filename) {
        Ok(v) => v,
        Err(_) => {
            println!("the program has failed to open this file: {filename}");
            return;
        }
    };
    println!("program output:\n{program}");
    match tokenize(&program) {
        Err(v) => match v {
            LexerError::OutOfBounds => println!("Out of bounds "),
            LexerError::UnexpectedOperator(c) => println!("unexpected operator {c}"),
            LexerError::Unreachable => println!("what!?"),
        },
        Ok(v) => {
            //for i in 0..v.len() {
            //    println!("{i} {:?}", v[i]);
            //}
            println!();
            let mut index = 0;
            let statements = parse(&v, &mut index);
            dbg!(&statements);
            println!();
            let mut interpreter = Interpreter::new();
            let (ret, _) = interpreter.eval_statement(&statements);
            println!("OUT {:?}", ret);
        }
    }
}

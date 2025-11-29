use interpreter::Interpreter;
use lexer::{tokenize, tokens::LexerError};
use parser::{self, parse_expression};
fn main() {
    let program = r#"
(2*3+1)>=1
    "#
    .to_string();
    match tokenize(&program) {
        Err(v) => match v {
            LexerError::OutOfBounds => println!("Out of bounds "),
            LexerError::UnexpectedOperator(c) => println!("unexpected operator {c}"),
            LexerError::Unreachable => println!("what!?"),
        },
        Ok(v) => {
            for i in 0..v.len() {
                println!("{i} {:?}", v[i]);
            }

            let mut index = 0;
            let (expression, is_bool) = parse_expression(&v, &mut index);
            dbg!(&expression);
            let mut interpreter = Interpreter::new();
            
            println!("{:?}", interpreter.eval_expression(&expression, is_bool));
        }
    }
    println!("Hello, world!");
}

use lexer::{tokenize, tokens::LexerError};

use parser::parse_expression;
fn main() {
    let program = r#"
1+2+3+parse(1,2,func());
    "#
    .to_string();
    match tokenize(&program) {
        Err(v) => match v {
            LexerError::OutOfBounds => println!("Out of bounds "),
            LexerError::UnexpectedOperator(c) => println!("unexpected operator {c}"),
            LexerError::Unreachable => println!("what!?"),
        },
        Ok(v) => {
            dbg!(&v);
            let mut index = 0;
            dbg!(parse_expression(&v, &mut index));
        }
    }
    println!("Hello, world!");
}

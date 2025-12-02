use interpreter::Interpreter;
use lexer::{tokenize, tokens::LexerError};
use parser::{self, parse};
fn main() {
    let program = r#"
    a=12;
    (2*3+1)>=1;
    a==12;
    a/=2;
    if(a==6){
      print a+"\n";
    }
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
            let statements = parse(&v, &mut index);
            dbg!(&statements);
            let interpreter = Interpreter::new_ctx();
            let (ret, _) = Interpreter::eval_statement(
                &mut interpreter.borrow_mut(),
                &interpreter,
                &statements,
            );
            println!("OUT {:?}", ret);
        }
    }
    println!("Hello, world!");
}

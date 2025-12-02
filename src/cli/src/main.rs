use interpreter::Interpreter;
use lexer::{tokenize, tokens::LexerError};
use parser::{self, parse};

fn main() {
    let program = r#"
    a=6;
    if(a!=6){
      print a+" sup dude\n";
    }elif(a++){
      print a;
    }else{
        print "nnigga"
    }
    for(i=0;i<10;i++){
        print i+" sup dude\n";    
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

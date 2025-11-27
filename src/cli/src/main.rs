use lexer::{tokenize, tokens::LexerError};

use parser::{self, parse};
fn main() {
    let program = r#"
define a(a,b,c){
    print a+b+c;
    return a+b+c;
}
define a(a){
    internal "cos" (a,b,c);
}

for (i=0; i<iterations; i++) {
   if (!i % 2) {
      print "value is pair\n";
      continue; 
   } 
   print "value is not pair\n";
    break;
   // break continue, halt are avaible here
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
            dbg!(parse(&v, &mut index));
        }
    }
    println!("Hello, world!");
}

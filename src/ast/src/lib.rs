use std::fmt::Debug;
#[derive(Debug, Clone)]
pub enum Expr {
    Number(f64),
    String(String),
    //
    Add,
    Subtract,
    Multiply,
    Divide,
    Mod,
    Increment {
        // ++
        name: String,
    },
    Decrement {
        name: String,
    },
    AddTo {
        name: String,
        value: Box<Expr>,
    },
    SubtractTo {
        name: String,
        value: Box<Expr>,
    },
    MultiplyTo {
        name: String,
        value: Box<Expr>,
    },
    DivideTo {
        name: String,
        value: Box<Expr>,
    },
    ModTo {
        name: String,
        value: Box<Expr>,
    },
    // boolean operators
    Equals,         // ==
    BiggerThan,     // >=
    BiggerOrEqual,  // >=
    SmallerThan,    // <
    SmallerOrEqual, // <=
    Different,      // !=
    // logical operators
    OR,  // ||
    AND, // &&
    NOT, // !
    // Unrelated
    Read,
    Operations {
        instructions: Vec<Expr>,
        is_bool: bool,
    }, // operations inside of a parenthesis
    VarCall {
        // a
        name: String,
    },
    FuncCall {
        // func(a,b,c)
        name: String,
        arguments: Vec<Expr>,
    },
    Internal {
        // internal "cos" ( a,b,c,d)
        name: String,
        arguments: Vec<Expr>,
    },
    VarAssign {
        // a=(1+2)
        name: String,
        value: Box<Expr>,
    },
}
#[derive(Debug, Clone)]
pub enum Stmt {
    Expression(Expr),
    Print(Expr),  // print "", print x, print func(), print 12+1, print (1+2+3)
    Return(Expr), // return, return "all good", return 123, return a==b, etc
    Continue,
    Break,
    Block {
        body: Vec<Stmt>,
    },

    FuncAssign {
        // define a (){}
        name: String,
        arguments: Vec<String>, // okay this doesnt seems bad so that is enough
        body: Vec<Stmt>,
    },
    // conditionals
    Elif {
        condition: Vec<Expr>,
        condition_bool: bool,
        then: Vec<Stmt>,
    },
    If {
        // if (a){}
        condition: Vec<Expr>,
        condition_bool: bool,
        if_then: Vec<Stmt>,
        elif_then: Vec<Stmt>, // i will handle various iffs
        else_then: Vec<Stmt>,
    },
    // loops
    ForLoop {
        // for(x=1;x<10;x++)
        init: Vec<Expr>,
        condition: Vec<Expr>,
        is_bool: bool, 
        increment: Vec<Expr>,
        body: Vec<Stmt>,
    },
    WhileLoop {
        // while(x)
        condition: Vec<Expr>,
        is_bool:bool,
        body: Vec<Stmt>,
    },
    // okay now i will make something simple
}

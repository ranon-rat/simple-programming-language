use std::fmt::Debug;
#[derive(Debug, Clone)]
pub struct VarCalling {
    pub name: String,
}
#[derive(Debug, Clone)]
pub struct ModifyingOperation {
    pub name: String,
    pub value: ExprOperations,
}

#[derive(Debug, Clone)]
pub struct ExprOperations {
    pub instructions: Vec<Expr>,
    pub is_bool: bool,
}

#[derive(Debug, Clone)]
pub struct FuncCall {
    pub name: String,
    pub arguments: Vec<ExprOperations>,
}

#[derive(Debug, Clone)]
pub struct VarAssign {
    pub name: String,
    pub value: ExprOperations,
}

#[derive(Debug, Clone)]
pub struct ArrayCall {
    pub name: String,
    pub at: ExprOperations,
}

#[derive(Debug, Clone)]
pub struct ArrayCallMod {
    pub name: String,
    pub at: ExprOperations,
    pub new_value: ExprOperations,
}

#[derive(Debug, Clone)]
pub enum Expr {
    Number(f64),
    String(String),
    Array(Vec<ExprOperations>),
    ArrayCall(ArrayCall),
    ArrayCallMod(ArrayCallMod),
    //
    Add,
    Subtract,
    Multiply,
    Divide,
    Mod,
    Increment(VarCalling),
    Decrement(VarCalling),
    AddTo(ModifyingOperation),
    SubtractTo(ModifyingOperation),
    MultiplyTo(ModifyingOperation),
    DivideTo(ModifyingOperation),
    ModTo(ModifyingOperation),
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
    Operations(ExprOperations), // operations inside of a parenthesis
    VarCall(VarCalling),
    FuncCall(FuncCall),
    Internal(FuncCall),
    VarAssign(VarAssign),
}

#[derive(Debug, Clone)]
pub struct FuncAssign {
    // define a (){}
    pub name: String,
    pub arguments: Vec<String>, // okay this doesnt seems bad so that is enough
    pub body: Vec<Stmt>,
}
#[derive(Debug, Clone)]
pub struct Elif {
    pub condition: Vec<Expr>,
    pub condition_bool: bool,
    pub then: Vec<Stmt>,
}
#[derive(Debug, Clone)]
pub struct If {
    pub condition: Vec<Expr>,
    pub condition_bool: bool,
    pub if_then: Vec<Stmt>,
    pub elif_then: Vec<Elif>, // i will handle various iffs
    pub else_then: Vec<Stmt>,
}
#[derive(Debug, Clone)]
pub struct ForLoop {
    pub init: Vec<Expr>,
    pub condition: Vec<Expr>,
    pub is_bool: bool,
    pub increment: Vec<Expr>,
    pub body: Vec<Stmt>,
}
#[derive(Debug, Clone)]
pub struct WhileLoop {
    // while(x)
    pub condition: Vec<Expr>,
    pub is_bool: bool,
    pub body: Vec<Stmt>,
}
#[derive(Debug, Clone)]
pub enum Stmt {
    Expression(ExprOperations),
    Print(ExprOperations), // print "", print x, print func(), print 12+1, print (1+2+3)
    Return(ExprOperations), // return, return "all good", return 123, return a==b, etc
    Continue,
    Break,
    Block(Vec<Stmt>),

    FuncAssign(FuncAssign),
    // conditionals
    If(If),
    // loops
    ForLoop(ForLoop),
    WhileLoop(WhileLoop),
    // okay now i will make something simple
}

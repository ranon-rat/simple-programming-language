use std::fmt::Debug;

#[derive(Debug, PartialEq)]
pub enum Tokens {
    Statement(String), // In case they write a string for a variable or anything else
    String(String),    // "example"
    // punctuators
    OpenParenthesis,  // (
    CloseParenthesis, // )
    OpenSquaredBrackets,
    CloseSquaredBrackets,
    OpenCurlyBrackets,  // {
    CloseCurlyBrackets, // }
    Comma,              // ,
    Equals,             // =

    // arithmetic operations
    Add,        // +
    Subtract,   // -
    Multiply,   // *
    Divide,     // /
    Mod,        // %
    AddTo,      // +=
    SubtractTo, // -=
    DivideTo,   // /=
    MultiplyTo, // *=
    ModTo,      // %=
    SemmiColon,
    Increment, // ++
    Decrement, // --
    // logic operators
    EqualsTo,          // ==
    IsDifferent,       // !=
    SmallerThan,       // <
    SmallerOrEqual,    // <=
    BiggerThan,        // >
    BiggerThanOrEqual, // >=
    // Boolean operators
    AndBool, // &&
    OrBool,  // ||
    NotBool, // !
    // BITWISE
    AndBitwise, // &
    OrBitwise,  // |

    // debugging
    Unkown,
}

#[derive(Debug)]
pub enum LexerError {
    UnexpectedOperator(char),
    OutOfBounds,
    Unreachable,
}

pub fn get_single_token(cell: char) -> Tokens {
    match cell {
        '(' => Tokens::OpenParenthesis,
        ')' => Tokens::CloseParenthesis,
        '{' => Tokens::OpenCurlyBrackets,
        '}' => Tokens::CloseCurlyBrackets,
        ',' => Tokens::Comma,
        '[' => Tokens::OpenSquaredBrackets,
        ']' => Tokens::CloseSquaredBrackets,
        _ => panic!("You shouldnt use this in this case \"{cell}\""),
    }
}

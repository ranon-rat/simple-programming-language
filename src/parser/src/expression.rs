use ast::{
    ArrayCall, ArrayCallMod, Expr, ExprOperations, FuncCall, ModifyingOperation, VarAssign,
    VarCalling,
};
use lexer::tokens::Tokens;

pub fn parse_argument_functions(
    program_tokens: &Vec<Tokens>,
    index: &mut usize,
) -> Vec<ExprOperations> {
    let mut arguments: Vec<ExprOperations> = Vec::new();
    while *index < program_tokens.len() {
        let current = &program_tokens[*index];
        match current {
            Tokens::CloseParenthesis => {
                return arguments;
            }
            _ => {
                let (arg, is_bool) = parse_expression(program_tokens, index);
                arguments.push(ExprOperations {
                    instructions: arg,
                    is_bool: (is_bool),
                });
            }
        }
        if program_tokens[*index] == Tokens::CloseParenthesis {
            return arguments;
        }
        *index += 1;
    }

    return arguments;
}
pub fn parse_argument_array(
    program_tokens: &Vec<Tokens>,
    index: &mut usize,
) -> Vec<ExprOperations> {
    let mut arguments = Vec::new();
    while *index < program_tokens.len() {
        let current = &program_tokens[*index];
        match current {
            Tokens::CloseSquaredBrackets => return arguments,
            _ => {
                let (arg, is_bool) = parse_expression(program_tokens, index);
                arguments.push(ExprOperations {
                    instructions: arg,
                    is_bool: (is_bool),
                });
            }
        }
        if program_tokens[*index] == Tokens::CloseSquaredBrackets {
            return arguments;
        }
        *index += 1;
    }
    return arguments;
}
#[derive(PartialEq, Eq)]
enum ComingFrom {
    SquaredBrackets,
    Parenthesis,
    None,
}
fn parse_statement_expr(
    variable_name: &String,
    program_tokens: &Vec<Tokens>,
    index: &mut usize,
    out: &mut Vec<Expr>,
) -> (bool, ComingFrom) {
    match variable_name.as_str() {
        "read" => {
            out.push(Expr::Read);
        }
        "internal" => {
            if *index + 1 >= program_tokens.len() {
                return (true, ComingFrom::None);
            }
            *index += 1;

            if let Tokens::String(func_name) = &program_tokens[*index] {
                *index += 1;
                match &program_tokens[*index] {
                    Tokens::OpenParenthesis => {
                        let arguments = parse_argument_functions(program_tokens, index);
                        out.push(Expr::Internal(FuncCall {
                            name: func_name.to_string(),
                            arguments: arguments,
                        }));
                    }
                    _ => return (true, ComingFrom::None),
                }
            }
        }

        _ => {
            let initial: char = variable_name.as_bytes()[0] as char;
            match initial {
                '0'..'9' => {
                    let number: f64 = variable_name.parse().unwrap();
                    out.push(Expr::Number(number));
                }

                _ => {
                    if *index + 1 >= program_tokens.len() {
                        out.push(Expr::VarCall(VarCalling {
                            name: variable_name.to_string(),
                        }));
                        return (true, ComingFrom::None);
                    }
                    let next = &program_tokens[*index + 1];
                    match next {
                        Tokens::OpenParenthesis => {
                            // x(
                            *index += 2;

                            let arguments = parse_argument_functions(program_tokens, index);
                            out.push(Expr::FuncCall(FuncCall {
                                name: variable_name.to_string(),
                                arguments: arguments,
                            }));
                            return (false, ComingFrom::Parenthesis);
                        }
                        Tokens::OpenSquaredBrackets => {
                            *index += 2;
                            let (at, is_bool) = parse_expression(program_tokens, index);
                            let has_next = *index + 2 < program_tokens.len();
                            if !has_next {
                                out.push(Expr::ArrayCall(ArrayCall {
                                    name: variable_name.to_string(),
                                    at: ExprOperations {
                                        instructions: at.clone(),
                                        is_bool: is_bool,
                                    },
                                }));
                            }
                            let next = *index + 1;
                            match &program_tokens[next] {
                                Tokens::Equals => {
                                    *index += 2;
                                    let (new_val, new_val_bool) =
                                        parse_expression(program_tokens, index);
                                    out.push(Expr::ArrayCallMod(ArrayCallMod {
                                        name: variable_name.to_string(),
                                        at: ExprOperations {
                                            instructions: at.clone(),
                                            is_bool: is_bool,
                                        },
                                        new_value: ExprOperations {
                                            instructions: new_val.clone(),
                                            is_bool: new_val_bool,
                                        },
                                    }));
                                    return (true, ComingFrom::None);
                                }
                                _ => {
                                    out.push(Expr::ArrayCall(ArrayCall {
                                        name: variable_name.to_string(),
                                        at: ExprOperations {
                                            instructions: at.clone(),
                                            is_bool: is_bool,
                                        },
                                    }));
                                    return (false, ComingFrom::SquaredBrackets);
                                }
                            }
                        }
                        Tokens::AddTo => {
                            // +=

                            *index += 2;
                            let (operations, operation_bool) =
                                parse_expression(program_tokens, index);
                            out.push(Expr::AddTo(ModifyingOperation {
                                name: variable_name.to_string(),
                                value: ExprOperations {
                                    instructions: operations,
                                    is_bool: operation_bool,
                                },
                            }));
                            return (true, ComingFrom::None);
                        }
                        Tokens::SubtractTo => {
                            *index += 2;
                            let (operations, operation_bool) =
                                parse_expression(program_tokens, index);
                            out.push(Expr::SubtractTo(ModifyingOperation {
                                name: variable_name.to_string(),
                                value: ExprOperations {
                                    instructions: operations,
                                    is_bool: operation_bool,
                                },
                            }));
                            return (true, ComingFrom::None);
                        }
                        Tokens::MultiplyTo => {
                            *index += 2;
                            let (operations, operation_bool) =
                                parse_expression(program_tokens, index);
                            out.push(Expr::MultiplyTo(ModifyingOperation {
                                name: variable_name.to_string(),
                                value: ExprOperations {
                                    instructions: operations,
                                    is_bool: operation_bool,
                                },
                            }));
                            return (true, ComingFrom::None);
                        }
                        Tokens::DivideTo => {
                            *index += 2;
                            let (operations, operation_bool) =
                                parse_expression(program_tokens, index);
                            out.push(Expr::DivideTo(ModifyingOperation {
                                name: variable_name.to_string(),
                                value: ExprOperations {
                                    instructions: operations,
                                    is_bool: operation_bool,
                                },
                            }));
                            return (true, ComingFrom::None);
                        }
                        Tokens::ModTo => {
                            *index += 2;
                            let (operations, operation_bool) =
                                parse_expression(program_tokens, index);
                            out.push(Expr::ModTo(ModifyingOperation {
                                name: variable_name.to_string(),
                                value: ExprOperations {
                                    instructions: operations,
                                    is_bool: operation_bool,
                                },
                            }));
                            return (true, ComingFrom::None);
                        }

                        // increment decrement
                        Tokens::Increment => {
                            // ++
                            *index += 1;
                            out.push(Expr::Increment(VarCalling {
                                name: variable_name.to_string(),
                            }));
                        }
                        Tokens::Decrement => {
                            // --
                            *index += 1;

                            out.push(Expr::Decrement(VarCalling {
                                name: variable_name.to_string(),
                            }));
                        }

                        _ => {
                            if *index + 1 >= program_tokens.len() {
                                out.push(Expr::VarCall(VarCalling {
                                    name: variable_name.to_string(),
                                }));
                            }
                            match &program_tokens[*index + 1] {
                                Tokens::Equals => {
                                    *index += 2; // a = x

                                    let (expr_out, is_bool) =
                                        parse_expression(program_tokens, index);
                                    out.push(Expr::VarAssign(VarAssign {
                                        name: variable_name.to_string(),
                                        value: ExprOperations {
                                            instructions: expr_out,
                                            is_bool,
                                        },
                                    }));
                                }
                                _ => {
                                    out.push(Expr::VarCall(VarCalling {
                                        name: variable_name.to_string(),
                                    }));
                                }
                            }
                        }
                    }
                } //   single_statement(cell, program_tokens, out, index);
            }
        }
    }
    return (false, ComingFrom::None);
}
// a*b +c= (a*b)a+c
pub fn continue_until_another_op(input: &Vec<Expr>, index: &mut usize) -> Vec<Expr> {
    let mut out = Vec::new();
    while *index < input.len() {
        let current = &input[*index];

        match current {
            Expr::Add
            | Expr::Subtract
            | Expr::Equals
            | Expr::Different
            | Expr::BiggerThan
            | Expr::BiggerOrEqual
            | Expr::SmallerThan
            | Expr::SmallerOrEqual
            | Expr::OR
            | Expr::AND
            | Expr::NOT => {
                break;
            }

            _ => {}
        }
        out.push(current.clone());
        *index += 1;
    }
    return out;
}
pub fn normalize_parse_expression(input: &Vec<Expr>) -> Vec<Expr> {
    let mut out = Vec::new();
    let mut i: usize = 0;
    while i < input.len() {
        let mut we_coming_operation = false;
        let current = &input[i];
        match current {
            Expr::String(_) | Expr::VarCall(_) | Expr::Number(_) | Expr::FuncCall(_) => {
                if i + 1 >= input.len() {
                    out.push(current.clone());
                    break;
                }
                let next = &input[i + 1];
                match next {
                    Expr::Divide | Expr::Multiply | Expr::Mod => {
                        let op = continue_until_another_op(input, &mut i);
                        out.push(Expr::Operations(ExprOperations {
                            instructions: op,
                            is_bool: false,
                        }));
                        we_coming_operation = true;
                    }
                    _ => {
                        out.push(current.clone());
                    }
                }
            }

            _ => {
                out.push(current.clone());
            }
        }
        if we_coming_operation && i < input.len() {
            out.push(input[i].clone());
        }

        i += 1;
    }
    return out;
}
pub fn parse_expression(program_tokens: &Vec<Tokens>, index: &mut usize) -> (Vec<Expr>, bool) {
    let mut out: Vec<Expr> = Vec::new();
    let mut is_bool = false;
    while *index < program_tokens.len() {
        let current = &program_tokens[*index];
        let mut coming_from = ComingFrom::None;

        match current {
            // arithmetic operations
            Tokens::Add => {
                out.push(Expr::Add);
            }
            Tokens::Subtract => {
                out.push(Expr::Subtract);
            }
            Tokens::Multiply => {
                out.push(Expr::Multiply);
            }
            Tokens::Divide => {
                out.push(Expr::Divide);
            }
            Tokens::Mod => {
                out.push(Expr::Mod);
            }
            // arithmetic self modifying operations

            // boolean operations
            // ==
            Tokens::EqualsTo => {
                is_bool = true;

                out.push(Expr::Equals);
            }
            // !=
            Tokens::IsDifferent => {
                is_bool = true;

                out.push(Expr::Different);
            }
            // >
            Tokens::BiggerThan => {
                is_bool = true;
                out.push(Expr::BiggerThan);
            }
            // >=
            Tokens::BiggerThanOrEqual => {
                is_bool = true;

                out.push(Expr::BiggerOrEqual);
            }
            // <
            Tokens::SmallerThan => {
                is_bool = true;

                out.push(Expr::SmallerThan);
            }
            // <=
            Tokens::SmallerOrEqual => {
                is_bool = true;
                out.push(Expr::SmallerOrEqual);
            }

            // boolean operations
            Tokens::NotBool => {
                is_bool = true;
                out.push(Expr::NOT);
            }
            Tokens::OrBool => {
                is_bool = true;

                out.push(Expr::OR);
            }
            Tokens::AndBool => {
                is_bool = true;
                out.push(Expr::AND);
            }
            // ()
            Tokens::OpenParenthesis => {
                *index += 1;
                let (operations, operation_bool) = parse_expression(program_tokens, index);

                out.push(Expr::Operations(ExprOperations {
                    instructions: operations,
                    is_bool: operation_bool,
                }));
                coming_from = ComingFrom::Parenthesis;
            }
            Tokens::String(v) => {
                out.push(Expr::String(v.to_string()));
            }
            Tokens::Statement(variable_name) => {
                let (should_break, coming) =
                    parse_statement_expr(variable_name, program_tokens, index, &mut out);
                if should_break {
                    return (normalize_parse_expression(&out), is_bool);
                }
                coming_from = coming;
            }
            Tokens::OpenSquaredBrackets => {
                *index += 1;
                let arguments = parse_argument_array(program_tokens, index);
                out.push(Expr::Array(arguments.clone()));
                coming_from = ComingFrom::SquaredBrackets;
            }
            Tokens::CloseParenthesis
            | Tokens::CloseSquaredBrackets
            | Tokens::SemmiColon
            | Tokens::Comma
            | _ => {
                // },{}
                return (normalize_parse_expression(&out), is_bool);
            }
        }
        if *index < program_tokens.len() {
            match &program_tokens[*index] {
                Tokens::SemmiColon | Tokens::Comma | Tokens::CloseCurlyBrackets => {
                    // },{}
                    break;
                }
                Tokens::CloseSquaredBrackets => {
                    if coming_from != ComingFrom::SquaredBrackets {
                        break;
                    }
                }
                Tokens::CloseParenthesis => {
                    if coming_from != ComingFrom::Parenthesis {
                        break;
                    }
                }
                _ => {}
            }
        }
        *index += 1;
    }

    return (normalize_parse_expression(&out), is_bool);
}

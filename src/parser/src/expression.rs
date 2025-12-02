use ast::{Expr, ExprOperations, FuncCall, ModifyingOperation, VarAssign, VarCalling};
use lexer::tokens::Tokens;

pub fn parse_argument_functions(program_tokens: &Vec<Tokens>, index: &mut usize) -> Vec<ExprOperations> {
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
fn parse_statement_expr(
    variable_name: &String,
    program_tokens: &Vec<Tokens>,
    index: &mut usize,
    out: &mut Vec<Expr>,
) -> bool {
    match variable_name.as_str() {
        "read" => {
            out.push(Expr::Read);
        }
        "internal" => {
            if *index + 1 >= program_tokens.len() {
                return true;
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
                    _ => return true,
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
                        return true;
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
                            return true;
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
                            return true;
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
                            return true;
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
                            return true;
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
                            return true;
                        }

                        // increment decrement
                        Tokens::Increment => {
                            // ++
                            out.push(Expr::Increment(VarCalling {
                                name: variable_name.to_string(),
                            }));
                            return true;
                        }
                        Tokens::Decrement => {
                            // --
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
    false
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

            _ => {},
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
        let mut we_coming_from_parenthesis = false;

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
                we_coming_from_parenthesis = true;
            }
            Tokens::String(v) => {
                out.push(Expr::String(v.to_string()));
            }
            Tokens::Statement(variable_name) => {
                if parse_statement_expr(variable_name, program_tokens, index, &mut out) {
                    return (normalize_parse_expression(&out), is_bool);
                }
            }
            Tokens::CloseParenthesis | Tokens::SemmiColon | Tokens::Comma | _ => {
                // },{}
                return (normalize_parse_expression(&out), is_bool);
            }
        }
        if *index < program_tokens.len() {
            match &program_tokens[*index] {
                Tokens::SemmiColon | Tokens::Comma => {
                    // },{}
                    break;
                }
                Tokens::CloseParenthesis => {
                    if !we_coming_from_parenthesis {
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

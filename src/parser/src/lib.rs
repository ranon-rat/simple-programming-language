use crate::ast::Stmt;
use ast::{self, Expr};
use lexer::{self, tokens::Tokens};

fn parse_argument_functions(program_tokens: &Vec<Tokens>, index: &mut usize) -> Vec<Expr> {
    let mut arguments: Vec<Expr> = Vec::new();
    while *index < program_tokens.len() {
        let current = &program_tokens[*index];
        match current {
            Tokens::CloseParenthesis => {
                return arguments;
            }
            _ => {
                let (arg, is_bool) = parse_expression(program_tokens, index);
                arguments.push(Expr::Operations {
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
pub fn parse_expression(program_tokens: &Vec<Tokens>, index: &mut usize) -> (Vec<Expr>, bool) {
    let mut out: Vec<Expr> = Vec::new();
    let mut previous: &Tokens = &Tokens::Unkown;
    let mut is_bool = false;
    while *index < program_tokens.len() {
        let current = &program_tokens[*index];
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
            Tokens::AddTo => {
                // +=
                if let Tokens::Statement(var_name) = previous {
                    *index += 1;
                    let (operations, operation_bool) = parse_expression(program_tokens, index);
                    out.push(Expr::AddTo {
                        name: var_name.to_string(),
                        value: Box::from(Expr::Operations {
                            instructions: operations,
                            is_bool: operation_bool,
                        }),
                    });
                    return (out, is_bool);
                }
            }
            Tokens::SubtractTo => {
                if let Tokens::Statement(var_name) = previous {
                    *index += 1;
                    let (operations, operation_bool) = parse_expression(program_tokens, index);
                    out.push(Expr::SubtractTo {
                        name: var_name.to_string(),
                        value: Box::from(Expr::Operations {
                            instructions: operations,
                            is_bool: operation_bool,
                        }),
                    });
                    return (out, is_bool);
                }
            }
            Tokens::MultiplyTo => {
                if let Tokens::Statement(var_name) = previous {
                    *index += 1;
                    let (operations, operation_bool) = parse_expression(program_tokens, index);
                    out.push(Expr::MultiplyTo {
                        name: var_name.to_string(),
                        value: Box::from(Expr::Operations {
                            instructions: operations,
                            is_bool: operation_bool,
                        }),
                    });
                    return (out, is_bool);
                }
            }
            Tokens::DivideTo => {
                if let Tokens::Statement(var_name) = previous {
                    *index += 1;
                    let (operations, operation_bool) = parse_expression(program_tokens, index);
                    out.push(Expr::DivideTo {
                        name: var_name.to_string(),
                        value: Box::from(Expr::Operations {
                            instructions: operations,
                            is_bool: operation_bool,
                        }),
                    });
                    return (out, is_bool);
                }
            }
            Tokens::ModTo => {
                if let Tokens::Statement(var_name) = previous {
                    *index += 1;
                    let (operations, operation_bool) = parse_expression(program_tokens, index);
                    out.push(Expr::ModTo {
                        name: var_name.to_string(),
                        value: Box::from(Expr::Operations {
                            instructions: operations,
                            is_bool: operation_bool,
                        }),
                    });
                    return (out, is_bool);
                }
            }

            // increment decrement
            Tokens::Increment => {
                // ++
                if let Tokens::Statement(var_name) = previous {
                    out.push(Expr::Increment {
                        name: var_name.to_string(),
                    });
                    return (out, is_bool);
                }
            }
            Tokens::Decrement => {
                // --
                if let Tokens::Statement(var_name) = previous {
                    out.push(Expr::Decrement {
                        name: var_name.to_string(),
                    });
                    return (out, is_bool);
                }
            }
            // boolean operations
            // ==
            Tokens::EqualsTo => {
                out.push(Expr::Equals);
            }
            // !=
            Tokens::IsDifferent => {
                out.push(Expr::Different);
            }
            // >
            Tokens::BiggerThan => {
                out.push(Expr::BiggerThan);
            }
            // >=
            Tokens::BiggerThanOrEqual => {
                out.push(Expr::BiggerOrEqual);
            }
            // <
            Tokens::SmallerThan => {
                out.push(Expr::SmallerThan);
            }
            // <=
            Tokens::SmallerOrEqual => {
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
                out.push(Expr::Operations {
                    instructions: operations,
                    is_bool: operation_bool,
                });
            }
            Tokens::String(v) => {
                out.push(Expr::String(v.to_string()));
            }
            Tokens::Statement(variable_name) => {
                let initial: char = variable_name.as_bytes()[0] as char;
                match initial {
                    '0'..'9' => {
                        let number: f64 = variable_name.parse().unwrap();
                        out.push(Expr::Number(number));
                    }
                    _ => {
                        if *index + 1 >= program_tokens.len() {
                            out.push(Expr::VarCall {
                                name: variable_name.to_string(),
                            });
                            return (out, is_bool);
                        }
                        let next = &program_tokens[*index + 1];
                        match next {
                            Tokens::OpenParenthesis => {
                                // x(
                                *index += 2;

                                let arguments = parse_argument_functions(program_tokens, index);
                                out.push(Expr::FuncCall {
                                    name: variable_name.to_string(),
                                    arguments: arguments,
                                })
                            }
                            _ => {
                                out.push(Expr::VarCall {
                                    name: variable_name.to_string(),
                                });
                            }
                        }
                    } //   single_statement(cell, program_tokens, out, index);
                }
            }
            Tokens::CloseParenthesis | Tokens::SemmiColon | Tokens::Comma | _ => {
                // },{}
                return (out, is_bool);
            }
        }
        *index += 1;
        previous = current;
    }

    return (out, is_bool);
}
fn parse_if_statement(program_tokens: &Vec<Tokens>, out: &mut Vec<Stmt>, index: &mut usize) {
    if *index + 2 >= program_tokens.len() {
        return;
    }

    *index += 2;
    // if (
    let (if_expr, if_expr_bool) = &parse_expression(program_tokens, index);
    if *index + 2 >= program_tokens.len() {
        // ends with ){
        return;
    }

    *index += 2;
    // i handle the if then statement
    let if_then = parse(program_tokens, index);
    // then i check if i have a then statement

    if *index + 1 >= program_tokens.len() {
        // ends with }
        return out.push(Stmt::If {
            condition_bool: *if_expr_bool,
            condition: *if_expr,
            if_then,
            else_then: Vec::new(),
            elif_then: Vec::new(),
        });
    }

    *index += 1;
    let mut elif_then: Vec<Stmt> = Vec::new();
    let mut else_then: Vec<Stmt> = Vec::new();
    while *index < program_tokens.len() {
        match &program_tokens[*index] {
            Tokens::Statement(v) => match v.as_str() {
                "else" => {
                    if *index + 1 >= program_tokens.len() {
                        break;
                    }
                    *index += 1;
                    else_then = parse(program_tokens, index);
                    break;
                }
                "elif" => {
                    // elif, then i jump to (
                    if *index + 2 >= program_tokens.len() {
                        break;
                    }
                    *index += 2;
                    let (elif_condition, is_bool) = parse_expression(program_tokens, index);
                    if *index + 2 >= program_tokens.len() {
                        // ){
                        break;
                    }
                    let elif_body = parse(program_tokens, index);
                    elif_then.push(Stmt::Elif {
                        condition_bool: is_bool,
                        condition: elif_condition,
                        then: elif_body,
                    });
                }
                _ => break,
            },
            _ => out.push(Stmt::If {
                condition_bool: *if_expr_bool,
                condition: *if_expr,
                if_then,
                elif_then: Vec::new(),
                else_then: Vec::new(),
            }),
        }
        *index += 1;
    }
}
fn handle_statement(
    cell: &Tokens,
    program_tokens: &Vec<Tokens>,
    out: &mut Vec<Stmt>,
    index: &mut usize,
) {
    if let Tokens::Statement(current) = cell {
        match current.as_str() {
            "if" => {
                parse_if_statement(program_tokens, out, index);
            }
            "while" => {}
            "for" => {}
            "return" => {
                let has_next = *index + 1 < program_tokens.len();
                if !has_next {
                    return out.push(Stmt::Return(Expr::Operations {
                        instructions: Vec::new(),
                        is_bool: false,
                    }));
                }
                *index += 1;
                let (returning, is_bool) = parse_expression(program_tokens, index);
                return out.push(Stmt::Return(Expr::Operations {
                    instructions: returning,
                    is_bool: is_bool,
                }));
            }
            "define" => todo!("function definitions"),
            "break" => out.push(Stmt::Break),
            "continue" => out.push(Stmt::Continue),
            "print" => {
                let has_next = *index + 1 < program_tokens.len();
                if !has_next {
                    return out.push(Stmt::Print(Expr::Operations {
                        instructions: Vec::new(),
                        is_bool: false,
                    }));
                }
                *index += 1;
                let (returning, is_bool) = parse_expression(program_tokens, index);
                return out.push(Stmt::Print(Expr::Operations {
                    instructions: returning,
                    is_bool: is_bool,
                }));
            }
            "" => panic!("empty statement this shouldnt happen"),
            _ => {
                let has_next = *index + 1 < program_tokens.len();
                if !has_next {
                    return out.push(Stmt::Expression(Expr::Operations {
                        instructions: Vec::new(),
                        is_bool: false,
                    }));
                }
                let next = &program_tokens[*index + 1];
                match next {
                    Tokens::Equals => {
                        *index += 1;
                        let (expr_out, is_bool) = parse_expression(program_tokens, index);
                        out.push(Stmt::VarAssign {
                            name: current.to_string(),
                            value: Expr::Operations {
                                instructions: expr_out,
                                is_bool,
                            },
                        });
                    }
                    _ => {
                        let (expr_out, is_bool) = parse_expression(program_tokens, index);
                        out.push(Stmt::Expression(Expr::Operations {
                            instructions: expr_out,
                            is_bool,
                        }));
                    }
                }
            }
        }
    }
}
pub fn parse(tokens: &Vec<Tokens>, index: &mut usize) -> Vec<Stmt> {
    let mut out = Vec::new();

    while *index < tokens.len() {
        let cell = &tokens[*index];
        match cell {
            Tokens::CloseCurlyBrackets => {
                return out;
            }
            Tokens::String(_) => {
                let (operations, is_bool) = parse_expression(tokens, index);
                out.push(Stmt::Expression(Expr::Operations {
                    instructions: operations,
                    is_bool: is_bool,
                }));
            }
            Tokens::Statement(_) => {
                handle_statement(cell, tokens, &mut out, index);
            }

            _ => {}
        }
        *index += 1;
    }
    out
}

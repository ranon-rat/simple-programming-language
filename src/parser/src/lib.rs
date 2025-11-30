use crate::ast::Stmt;
use ast::{
    self, Elif, Expr, ExprOperations, ForLoop, FuncAssign, FuncCall, If, ModifyingOperation,
    VarAssign, VarCalling, WhileLoop,
};
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
                arguments.push(Expr::Operations(ExprOperations {
                    instructions: arg,
                    is_bool: (is_bool),
                }));
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
fn continue_until_another_op(input: &Vec<Expr>, index: &mut usize) -> (Vec<Expr>, bool) {
    let mut out = Vec::new();
    let mut is_bool = false;
    while *index < input.len() {
        let current = &input[*index];

        match current {
            Expr::Add | Expr::Subtract => {
                break;
            }
          
            _ => match current {
                Expr::Equals
                | Expr::Different
                | Expr::BiggerThan
                | Expr::BiggerOrEqual
                | Expr::SmallerThan
                | Expr::SmallerOrEqual
                | Expr::OR
                | Expr::AND
                | Expr::NOT => {
                    is_bool = true;
                }
                _ => {}
            },
        }
        out.push(current.clone());
        *index += 1;
    }
    return (out, is_bool);
}
fn normalize_parse_expression(input: &Vec<Expr>) -> Vec<Expr> {
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
                       
                        let (op, is_bool) = continue_until_another_op(input, &mut i);
                        out.push(Expr::Operations(ExprOperations {
                            instructions: op,
                            is_bool: is_bool,
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
        println!("{:?} {}", current, *index);

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
fn parse_if_statement(program_tokens: &Vec<Tokens>, index: &mut usize, out: &mut Vec<Stmt>) {
    if *index + 2 >= program_tokens.len() {
        return;
    }

    // if (  expr

    *index += 2;
    // if (
    let (if_expr, if_expr_bool) = parse_expression(program_tokens, index);
    if *index + 2 >= program_tokens.len() {
        // ends with ){
        return;
    }

    *index += 2; // 
    // i handle the if then statement
    let if_then = parse(program_tokens, index);

    // then i check if i have a then statement

    if *index + 1 >= program_tokens.len() {
        // ends with }
        return out.push(Stmt::If(If {
            condition_bool: if_expr_bool,
            condition: if_expr.to_vec(),
            if_then,
            else_then: Vec::new(),
            elif_then: Vec::new(),
        }));
    }

    *index += 1;
    let mut elif_then: Vec<Elif> = Vec::new();
    let mut else_then: Vec<Stmt> = Vec::new();
    while *index < program_tokens.len() {
        match &program_tokens[*index] {
            Tokens::Statement(v) => match v.as_str() {
                "else" => {
                    if *index + 2 >= program_tokens.len() {
                        break;
                    }
                    *index += 2;
                    else_then = parse(program_tokens, index);
                    break;
                }
                "elif" => {
                    // elif, then i jump to (
                    // elif (
                    if *index + 2 >= program_tokens.len() {
                        break;
                    }
                    *index += 2;
                    let (elif_condition, is_bool) = parse_expression(program_tokens, index);
                    if *index + 2 >= program_tokens.len() {
                        // ){
                        break;
                    }
                    *index += 2;
                    let elif_body = parse(program_tokens, index);
                    elif_then.push(Elif {
                        condition_bool: is_bool,
                        condition: elif_condition,
                        then: elif_body,
                    });
                }
                _ => break,
            },
            _ => break,
        }
        *index += 1;
    }
    out.push(Stmt::If(If {
        condition_bool: if_expr_bool,
        condition: if_expr.to_vec(),
        if_then: if_then.to_vec(),
        elif_then: elif_then.to_vec(),
        else_then: else_then.to_vec(),
    }))
}
fn parse_def_args_function(program_tokens: &Vec<Tokens>, index: &mut usize) -> Vec<String> {
    let mut arguments: Vec<String> = Vec::new();
    while *index < program_tokens.len() {
        match &program_tokens[*index] {
            Tokens::Statement(v) => {
                arguments.push(v.to_string());
            }
            Tokens::CloseParenthesis => return arguments,
            _ => {}
        }
        *index += 1;
    }
    return arguments;
}
fn parse_def_function(program_tokens: &Vec<Tokens>, index: &mut usize, out: &mut Vec<Stmt>) {
    match &program_tokens[*index] {
        Tokens::Statement(func_name) => {
            // func (
            dbg!(&index, &program_tokens[*index]);

            *index += 2;
            dbg!(&index, &program_tokens[*index]);

            let arguments = parse_def_args_function(program_tokens, index);
            dbg!(&index, &program_tokens[*index]);

            *index += 2;
            dbg!(&index, &program_tokens[*index]);

            let body = parse(program_tokens, index);

            out.push(Stmt::FuncAssign(FuncAssign {
                name: func_name.to_string(),
                arguments,
                body,
            }));
        }
        _ => {}
    }
}
// we start from ( x
fn parse_init_for_loop(program_tokens: &Vec<Tokens>, index: &mut usize) -> Vec<Expr> {
    let mut out: Vec<Expr> = Vec::new();
    while *index < program_tokens.len() {
        match &program_tokens[*index] {
            Tokens::SemmiColon | Tokens::CloseParenthesis => {
                return out;
            }
            _ => {
                let (new, is_bool) = parse_expression(program_tokens, index);
                out.push(Expr::Operations(ExprOperations {
                    instructions: new,
                    is_bool: is_bool,
                }));
            }
        }
        match &program_tokens[*index] {
            Tokens::SemmiColon | Tokens::CloseParenthesis => {
                return out;
            }
            _ => {}
        }
        *index += 1;
    }
    return out;
}

fn parse_for_loop(program_tokens: &Vec<Tokens>, index: &mut usize, out: &mut Vec<Stmt>) {
    // we start after the for(
    // i=1

    let init = parse_init_for_loop(program_tokens, index);
    // ;
    *index += 1;
    // i<10
    let (condition, is_bool) = parse_expression(program_tokens, index);
    // ;
    *index += 1;
    // i++
    let increment = parse_init_for_loop(program_tokens, index);
    // )
    *index += 2;
    let body = parse(program_tokens, index);
    out.push(Stmt::ForLoop(ForLoop {
        init,
        condition: condition,
        is_bool,
        increment: increment,
        body: body,
    }));
}
fn parse_while_loop(program_tokens: &Vec<Tokens>, index: &mut usize, out: &mut Vec<Stmt>) {
    // so while(
    let (condition, is_bool) = parse_expression(program_tokens, index);
    *index += 2; // ){
    let body = parse(program_tokens, index);
    out.push(Stmt::WhileLoop(WhileLoop {
        condition: condition,
        body,
        is_bool,
    }));
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
                parse_if_statement(program_tokens, index, out);
            }
            "for" => {
                *index += 2;
                parse_for_loop(program_tokens, index, out);
            }
            "while" => {
                *index += 2;
                parse_while_loop(program_tokens, index, out);
            }
            "define" => {
                *index += 1;
                parse_def_function(program_tokens, index, out);
            }
            "break" => out.push(Stmt::Break),
            "continue" => out.push(Stmt::Continue),
            "return" => {
                let has_next = *index + 1 < program_tokens.len();
                if !has_next {
                    return out.push(Stmt::Return(Expr::Operations(ExprOperations {
                        instructions: Vec::new(),
                        is_bool: false,
                    })));
                }
                *index += 1;
                let (returning, is_bool) = parse_expression(program_tokens, index);
                return out.push(Stmt::Return(Expr::Operations(ExprOperations {
                    instructions: returning,
                    is_bool: is_bool,
                })));
            }

            "print" => {
                let has_next = *index + 1 < program_tokens.len();
                if !has_next {
                    return out.push(Stmt::Print(Expr::Operations(ExprOperations {
                        instructions: Vec::new(),
                        is_bool: false,
                    })));
                }
                *index += 1;
                let (returning, is_bool) = parse_expression(program_tokens, index);
                return out.push(Stmt::Print(Expr::Operations(ExprOperations {
                    instructions: returning,
                    is_bool: is_bool,
                })));
            }
            "" => panic!("empty statement this shouldnt happen"),
            _ => {
                let has_next = *index + 1 < program_tokens.len();
                if !has_next {
                    return out.push(Stmt::Expression(Expr::Operations(ExprOperations {
                        instructions: Vec::new(),
                        is_bool: false,
                    })));
                }

                let (expr_out, is_bool) = parse_expression(program_tokens, index);
                out.push(Stmt::Expression(Expr::Operations(ExprOperations {
                    instructions: expr_out,
                    is_bool,
                })));
            }
        }
    }
}
pub fn parse(tokens: &Vec<Tokens>, index: &mut usize) -> Vec<Stmt> {
    let mut out = Vec::new();

    while *index < tokens.len() {
        let cell = &tokens[*index];
        match cell {
            Tokens::OpenCurlyBrackets => {
                *index += 1;
                let body = parse(tokens, index);
                out.push(Stmt::Block(body))
            }
            Tokens::CloseCurlyBrackets => {
                return out;
            }
            Tokens::String(_) => {
                let (operations, is_bool) = parse_expression(tokens, index);
                out.push(Stmt::Expression(Expr::Operations(ExprOperations {
                    instructions: operations,
                    is_bool: is_bool,
                })));
            }
            Tokens::OpenParenthesis => {
                let (operations, is_bool) = parse_expression(tokens, index);
                out.push(Stmt::Expression(Expr::Operations(ExprOperations {
                    instructions: operations,
                    is_bool,
                })));
            }
            Tokens::Statement(_) => {
                handle_statement(cell, tokens, &mut out, index);
            }

            _ => {}
        }

        *index += 1;
    }
    return out;
}

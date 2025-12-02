pub mod expression;
use crate::ast::Stmt;
use crate::expression::parse_expression;

use ast::{self, Elif, Expr, ExprOperations, ForLoop, FuncAssign, If, WhileLoop};
use lexer::{self, tokens::Tokens};

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

    let mut elif_then: Vec<Elif> = Vec::new();
    let mut else_then: Vec<Stmt> = Vec::new();
    let mut i = *index + 1; // exploring
    while i < program_tokens.len() {
        println!("if statement{:?}", &program_tokens[i]);
        match &program_tokens[i] {
            Tokens::Statement(v) => match v.as_str() {
                "else" => {
                    if i + 2 >= program_tokens.len() {
                        break;
                    }
                    i += 2;
                    println!("else statement {:?}", program_tokens[i]);

                    else_then = parse(program_tokens, &mut i);
                    i+=1;
                    break;
                }
                "elif" => {
                    // elif, then i jump to (
                    // elif (
                    if i + 2 >= program_tokens.len() {
                        break;
                    }
                    i += 2;
                    println!("elif statement {:?}", program_tokens[i]);

                    let (elif_condition, is_bool) = parse_expression(program_tokens, &mut i);
                    println!("elif statement {:?}", program_tokens[i]);

                    if i + 2 >= program_tokens.len() {
                        // ){
                        break;
                    }
                    i += 2;
                    println!("elif statement {:?}", program_tokens[i]);

                    let elif_body = parse(program_tokens, &mut i);

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
        i += 1;
    }
    *index = i - 1;

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
        let current = &program_tokens[*index];
        println!("init for loop {:?}", current);
        match current {
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
    println!("CHECK THIS{:?}", &program_tokens[*index]);

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
                    return out.push(Stmt::Print(ExprOperations {
                        instructions: Vec::new(),
                        is_bool: false,
                    }));
                }
                let mut i = *index + 1;
                let (returning, is_bool) = parse_expression(program_tokens, &mut i);
                *index = i - 1;
                return out.push(Stmt::Print(ExprOperations {
                    instructions: returning,
                    is_bool: is_bool,
                }));
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
        println!("{:?}", cell);
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

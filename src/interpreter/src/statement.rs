use crate::types::{ Interpreter, ReasonsForStopping, Types};
use ast::{Expr, ForLoop, If, Stmt, WhileLoop};
use std::cell::RefCell;
use std::rc::Rc;

impl Interpreter {
    fn eval_expression_change_output(&mut self, out: &mut Types, eval: &Expr) {
        if let Expr::Operations(v) = eval {
            *out = self.eval_expression(&v.instructions, v.is_bool);
        }
    }

    pub fn eval_block(&mut self, lines: &Vec<Stmt>) -> (Types, ReasonsForStopping) {
        let mut interpreter = self.new_context();
        return interpreter.eval_statement(lines);
    }

    pub fn eval_if_statement(&mut self, if_stmt: &If) -> (Types, ReasonsForStopping) {
        let condition = self.eval_expression(&if_stmt.condition, if_stmt.condition_bool);
        if condition.to_number() > 0.0 {
            let mut interpreter = self.new_context();

            return interpreter.eval_statement(&if_stmt.if_then);
        }
        for elif in &if_stmt.elif_then {
            let condition = self.eval_expression(&elif.condition, elif.condition_bool);
            if condition.to_number() > 0.0 {
                let mut interpreter = self.new_context();
                return interpreter.eval_statement(&elif.then);
            }
        }
        let mut interpreter = self.new_context();
        return interpreter.eval_statement(&if_stmt.else_then);
    }
    pub fn eval_while_loop(&mut self, while_loop: &WhileLoop) -> (Types, ReasonsForStopping) {
        let mut interpreter = self.new_context();

        let mut condition = self.eval_expression(&while_loop.condition, while_loop.is_bool);
        while condition.to_number() > 0.0 {
            let (out, reason) = interpreter.eval_statement(&while_loop.body);
            match &reason {
                ReasonsForStopping::Finished | ReasonsForStopping::ContinueStatement => {}
                _ => return (out, reason),
            }
            condition = self.eval_expression(&while_loop.condition, while_loop.is_bool);
        }

        return (Types::Number(0.0), ReasonsForStopping::Finished);
    }
    pub fn eval_for_loop(&mut self, for_loop: &ForLoop) -> (Types, ReasonsForStopping) {
        let mut interpreter = self.new_context();
    
        interpreter.eval_expression(&for_loop.init, false);
        while interpreter.eval_expression(&for_loop.condition, for_loop.is_bool).to_number() > 0.0 {
            let (out, reason) = interpreter.eval_statement(&for_loop.body);
            match &reason {
                ReasonsForStopping::Finished | ReasonsForStopping::ContinueStatement => {}
                _ => return (out, reason),
            }
            interpreter.eval_expression(&for_loop.increment, false);
        }

        return (Types::Number(0.0), ReasonsForStopping::Finished);
    }

    pub fn eval_statement(&mut self, lines: &Vec<Stmt>) -> (Types, ReasonsForStopping) {
        let mut out = Types::Number(0.0);
        let mut i = 0;

        while i < lines.len() {
            let current = &lines[i];
            match current {
                Stmt::FuncAssign(func_assign) => {
                    self.functions.insert(
                        func_assign.name.to_string(),
                        Rc::new(RefCell::new(func_assign.clone())),
                    );
                }
                Stmt::Expression(eval) => {
                    self.eval_expression_change_output(&mut out, eval);
                }
                Stmt::Return(eval) => {
                    self.eval_expression_change_output(&mut out, eval);
                    return (out, ReasonsForStopping::ReturnStatement);
                }
                Stmt::Break => {
                    return (out, ReasonsForStopping::BreakStatement);
                }
                Stmt::Continue => {
                    return (out, ReasonsForStopping::ContinueStatement);
                }
                Stmt::Print(eval_expr) => {
                    out = self.eval_expression(&eval_expr.instructions, eval_expr.is_bool);
                    match &out {
                        Types::Number(v) => {
                            print!("{}", v);
                        }
                        Types::String(v) => {
                            print!("{}", v)
                        }
                    }
                }
                Stmt::Block(_block) => {
                    let (value, reason) = self.eval_block(lines);
                    match &reason {
                        ReasonsForStopping::Finished => {
                            out = value.clone();
                        }
                        _ => return (value, reason),
                    }
                }
                Stmt::If(if_stmt) => {
                    let (exit, reason) = self.eval_if_statement(if_stmt);
                    match &reason {
                        ReasonsForStopping::Finished => {}
                        _ => return (exit, reason),
                    }
                }
                Stmt::WhileLoop(while_loop) => {
                    let (exit, reason) = self.eval_while_loop(while_loop);
                    match &reason {
                        ReasonsForStopping::ReturnStatement => return (exit, reason),
                        _ => {}
                    }
                }
                Stmt::ForLoop(for_loop) => {
                    let (exit, reason) = self.eval_for_loop(for_loop);
                    match &reason {
                        ReasonsForStopping::ReturnStatement => return (exit, reason),
                        _ => {}
                    }
                }
            }

            i += 1;
        }
        return (out, ReasonsForStopping::Finished);
    }
}

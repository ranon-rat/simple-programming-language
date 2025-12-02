use crate::types::{Ctx, Interpreter, ReasonsForStopping, Types};
use ast::{Expr, ForLoop, If, Stmt, WhileLoop};
use std::cell::RefCell;
use std::rc::Rc;

impl Interpreter {
    fn eval_expression_change_output(&mut self, ctx: &Ctx, out: &mut Types, eval: &Expr) {
        if let Expr::Operations(v) = eval {
            *out = self.eval_expression(ctx, &v.instructions, v.is_bool);
        }
    }
    pub fn eval_block(&mut self, ctx: &Ctx, lines: &Vec<Stmt>) -> (Types, ReasonsForStopping) {
        let new_ctx = self.new_context(&ctx);
        return new_ctx.borrow_mut().eval_statement(ctx, lines);
    }

    pub fn eval_if_statement(&mut self, ctx: &Ctx, if_stmt: &If) -> (Types, ReasonsForStopping) {
        let condition = self.eval_expression(ctx, &if_stmt.condition, if_stmt.condition_bool);
        if condition.to_number() > 0.0 {
            let new_ctx = self.new_context(ctx);
            return new_ctx
                .borrow_mut()
                .eval_statement(&new_ctx, &if_stmt.if_then);
        }
        for elif in &if_stmt.elif_then {
            let condition = self.eval_expression(ctx, &elif.condition, elif.condition_bool);
            if condition.to_number() > 0.0 {
                let new_ctx = self.new_context(ctx);
                return new_ctx.borrow_mut().eval_statement(&new_ctx, &elif.then);
            }
        }
        let new_ctx = self.new_context(ctx);
        return new_ctx
            .borrow_mut()
            .eval_statement(&new_ctx, &if_stmt.else_then);
    }
    pub fn eval_while_loop(
        &mut self,
        ctx: &Ctx,
        while_loop: &WhileLoop,
    ) -> (Types, ReasonsForStopping) {
        let new_ctx = self.new_context(ctx);
        let mut interpreter = new_ctx.borrow_mut();
        let mut condition = self.eval_expression(ctx, &while_loop.condition, while_loop.is_bool);
        while condition.to_number() > 0.0 {
            let (out, reason) = interpreter.eval_statement(ctx, &while_loop.body);
            match &reason {
                ReasonsForStopping::Finished | ReasonsForStopping::ContinueStatement => {}
                _ => return (out, reason),
            }
            condition = self.eval_expression(ctx, &while_loop.condition, while_loop.is_bool);
        }
        return (Types::Number(0.0), ReasonsForStopping::Finished);
    }
    pub fn eval_for_loop(&mut self, ctx: &Ctx, for_loop: &ForLoop) -> (Types, ReasonsForStopping) {
        let new_ctx = self.new_context(ctx);
        let mut interpreter = new_ctx.borrow_mut();
        // first once
        self.eval_expression(ctx, &for_loop.init, false);
        let condition = self.eval_expression(ctx, &for_loop.condition, for_loop.is_bool);
        while condition.to_number() > 0.0 {
            let (out, reason) = interpreter.eval_statement(ctx, &for_loop.body);
            match &reason {
                ReasonsForStopping::Finished | ReasonsForStopping::ContinueStatement => {}
                _ => return (out, reason),
            }
            self.eval_expression(ctx, &for_loop.condition, for_loop.is_bool);
            self.eval_expression(ctx, &for_loop.increment, false);
        }
        return (Types::Number(0.0), ReasonsForStopping::Finished);
    }

    pub fn eval_statement(&mut self, ctx: &Ctx, lines: &Vec<Stmt>) -> (Types, ReasonsForStopping) {
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
                    self.eval_expression_change_output(ctx, &mut out, eval);
                }
                Stmt::Return(eval) => {
                    self.eval_expression_change_output(ctx, &mut out, eval);
                    return (out, ReasonsForStopping::ReturnStatement);
                }
                Stmt::Break => {
                    return (out, ReasonsForStopping::BreakStatement);
                }
                Stmt::Continue => {
                    return (out, ReasonsForStopping::ContinueStatement);
                }
                Stmt::Print(eval_expr) => {
                    println!("printing");
                    self.eval_expression_change_output(ctx, &mut out, eval_expr);
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
                    let (value, reason) = self.eval_block(ctx, lines);
                    match &reason {
                        ReasonsForStopping::Finished => {
                            out = value.clone();
                        }
                        _ => return (value, reason),
                    }
                }
                Stmt::If(if_stmt) => {
                    let (exit, reason) = self.eval_if_statement(ctx, if_stmt);
                    match &reason {
                        ReasonsForStopping::Finished => {}
                        _ => return (exit, reason),
                    }
                }
                Stmt::WhileLoop(while_loop) => {
                    let (exit, reason) = self.eval_while_loop(ctx, while_loop);
                    match &reason {
                        ReasonsForStopping::ReturnStatement => return (exit, reason),
                        _ => {}
                    }
                }
                Stmt::ForLoop(for_loop) => {
                    let (exit,reason) = self.eval_for_loop(ctx, for_loop);
                    match &reason {
                        ReasonsForStopping::ReturnStatement => return (exit, reason),
                        _ => {}
                    }
                }
            }
            println!("{:?}", out);

            i += 1;
        }
        return (out, ReasonsForStopping::Finished);
    }
}

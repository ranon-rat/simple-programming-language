use crate::types::{Interpreter, Types};
use ast::{Expr, Stmt};

impl Interpreter {
    fn eval_expression_change_output(&mut self, out: &mut Types, eval: &Expr) {
        if let Expr::Operations(v) = eval {
            *out = self.eval_expression(&v.instructions, v.is_bool);
        }
    }
    pub fn eval_statement(&mut self, lines: &Vec<Stmt>) -> Types {
        let mut out = Types::Number(0.0);
        let mut i = 0;
        while i < lines.len() {
            let current = &lines[i];
            match current {
                Stmt::Expression(eval) => {
                    self.eval_expression_change_output(&mut out, eval);
                }
                Stmt::Return(eval) => {
                    self.eval_expression_change_output(&mut out, eval);
                    return out;
                }
                Stmt::If(_if_stmt) => todo!("implement a if statement evaluator"),
                Stmt::ForLoop(_for_loop) => todo!("implement a for loop evaluator"),
                Stmt::Block(_) => todo!("block"),

                Stmt::Print(eval_expr) => {
                    self.eval_expression_change_output(&mut out, eval_expr);
                    match &out {
                        Types::Number(v) => {
                            print!("{}", v);
                        }
                        Types::String(v) => {
                            print!("{}", v)
                        }
                    }
                }
                _ => {}
            }
            println!("{:?} {:?}", current, out);

            i += 1;
        }
        return out;
    }
}

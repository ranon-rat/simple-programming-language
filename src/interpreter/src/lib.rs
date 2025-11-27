use ast::{self, Expr, VarCalling};
use std::collections::HashMap;
#[derive(Debug, Clone, PartialEq)]
pub enum Types {
    Number(f64),
    String(String),
    // i should later have something else
}

pub struct Interpreter {
    pub variables: HashMap<String, Types>,
    pub functions: HashMap<String, ast::FuncAssign>,
    pub internal_functions: HashMap<String, fn(Vec<Types>) -> Types>,
    pub previous_context: Option<Box<Interpreter>>,
    pub global_context: Option<Box<Interpreter>>,
}

impl Interpreter {
    pub fn get_var(&mut self, var_name: &String) -> Option<&mut Types> {
        if let Some(v) = self.variables.get_mut(var_name) {
            return Some(v);
        }
        // No mover previous_context → usar as_mut()
        if let Some(prev) = self.previous_context.as_mut() {
            return prev.get_var(var_name);
        }
        return None;
    }
    pub fn get_func(&mut self, function: &String) -> Option<&ast::FuncAssign> {
        if let Some(f) = self.functions.get(function) {
            return Some(f);
        }
        if let Some(v) = self.previous_context.as_mut() {
            match v.get_func(function) {
                Some(f) => return Some(f),
                None => {}
            };
        }

        return None;
    }
    pub fn get_internal(&mut self, internal_function: &String) -> Option<fn(Vec<Types>) -> Types> {
        if let Some(v) = self.global_context.as_mut() {
            return v.get_internal(internal_function);
        }
        // Primero busca en el scope actual
        if let Some(f) = self.internal_functions.get(internal_function) {
            return Some(*f); // los punteros a fn son Copy
        }
        return None;
    }
    fn eval_previous_expression(
        &mut self,
        expression: Vec<Expr>,
        is_bool: bool,
        index: &mut usize,
        prev: &Types,
        
    ) ->Types{
    }
    pub fn eval_expression(&mut self, expression: Vec<Expr>, is_bool: bool) -> Types {
        let mut default_output = Types::Number(0.0);
        let mut previous_operation: Option<Expr> = None;
        let mut i: usize = if is_bool { expression.len() } else { 0 };
        while (i < expression.len() && !is_bool) || (i != 0 && is_bool) {
            let current = &expression[i];
            match current {
                Expr::VarCall(var) => match previous_operation {
                    None => {
                        if let Some(value) = self.get_var(&var.name) {
                            default_output = value;
                        }
                    }
                    _ => {}
                },
                Expr::Number(value) => {}
                _ => {}
            }
            if is_bool {
                i -= 1;
            } else {
                i += 1;
            }
        }

        return default_output;
    }
}

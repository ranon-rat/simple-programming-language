use crate::types::Types;
use ast::{self, Expr, VarCalling};
use std::collections::HashMap;
mod types;
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

    fn eval_individual_comparing_parts(&mut self, current: &Expr) -> Types {
        match current {
            Expr::VarCall(var) => {
                return self
                    .get_var(&var.name)
                    .map(|v| v.clone())
                    .unwrap_or(Types::Number(0.0));
            }
            Expr::Operations(operations) => {
                return self.eval_expression(&operations.instructions, operations.is_bool);
            }
            Expr::FuncCall(_) => todo!("later to be implemented"),
            _ => {
                return Types::Number(0.0);
            }
        }
    }
    fn eval_boolean_operation(
        &mut self,
        expression: &Vec<Expr>,
        index: &mut usize,
    ) -> Option<Types> {
        // i should check first if i have a next point :)
        if *index + 2 >= expression.len() {
            return None;
        }
        match expression[*index + 1] {
            Expr::Equals
            | Expr::Different
            | Expr::BiggerThan
            | Expr::BiggerOrEqual
            | Expr::SmallerThan
            | Expr::SmallerOrEqual => {}
            _ => {
                return None;
            }
        }
        let comparing_a = &expression[*index];
        *index += 1;
        let token = &expression[*index];
        *index += 1;
        let comparing_b = &expression[*index];
        let value_a = self.eval_individual_comparing_parts(comparing_a);
        let value_b = self.eval_individual_comparing_parts(comparing_b);
        let result = match token {
            Expr::Equals => value_a == value_b,
            Expr::Different => value_a != value_b,
            Expr::BiggerThan => value_a.to_number() > value_b.to_number(),
            Expr::BiggerOrEqual => value_a.to_number() >= value_b.to_number(),
            Expr::SmallerThan => value_a.to_number() < value_b.to_number(),
            Expr::SmallerOrEqual => value_a.to_number() <= value_b.to_number(),
            _ => {
                return None;
            }
        };
        Some(Types::Number(if result { 1.0 } else { 0.0 }))
    }
    fn eval_previous_expression(
        &mut self,
        _expression: &Vec<Expr>,
        is_bool: bool,
        _index: &mut usize,
        _out: &mut Types,
        _previous_op: Expr,
    ) {
        if is_bool {}
    }

    pub fn eval_expression(&mut self, expression: &Vec<Expr>, is_bool: bool) -> Types {
        let mut out = Types::Number(0.0);
        let mut previous_operation: Option<Expr> = None;
        let mut i: usize = 0;
        while i < expression.len() {
            let current = &expression[i];
            match current {
                Expr::VarCall(var) => match previous_operation {
                    None => {
                        if let Some(value) = self.get_var(&var.name) {
                            out = value.to_owned();
                        }
                    }
                    Some(previous_op) => {
                        self.eval_previous_expression(
                            expression,
                            is_bool,
                            &mut i,
                            &mut out,
                            previous_op,
                        );
                    }
                },
                Expr::Number(value) => {
                    println!("{value}");
                }
                _ => {}
            }
            match current {
                Expr::OR
                | Expr::AND
                | Expr::NOT
                | Expr::Add
                | Expr::Subtract
                | Expr::Multiply
                | Expr::Divide
                | Expr::Mod => {
                    previous_operation = Some(current.clone());
                }
                _ => {
                    previous_operation = None;
                }
            }
            i += 1;
        }
        if is_bool {
            match out {
                Types::String(v) => return Types::Number(if v.len() > 0 { 1.0 } else { 0.0 }),
                Types::Number(v) => return Types::Number(if v > 0.0 { 1.0 } else { 0.0 }),
            }
        }
        return out;
    }
}

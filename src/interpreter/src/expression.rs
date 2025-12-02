use crate::types::{ Interpreter, ReasonsForStopping, Types};
use ast::{Expr, FuncCall, ModifyingOperation};
use std::cell::RefCell;
use std::rc::Rc;

impl Interpreter {
    fn eval_function(&mut self, func_call: &FuncCall) -> Types {
        let mut interpreter = self.new_context();

        let func_opt = self.get_func(&func_call.name);
        let function = match &func_opt {
            Some(f) => f.borrow(),
            None => return Types::Number(0.0),
        };
        let mut i = 0;
        while i < function.arguments.len() && i < func_call.arguments.len() {
            let var_name = &function.arguments[i];
            let expr = &func_call.arguments[i];
            let eval = self.eval_expression(&expr.instructions, expr.is_bool);
            interpreter
                .variables
                .insert(var_name.to_string(), Rc::new(RefCell::new(eval.clone())));
            i += 1;
        }
        let (out, stop_reason) = interpreter.eval_statement( &function.body);
        match &stop_reason {
            ReasonsForStopping::ReturnStatement => return out,
            _ => return Types::Number(0.0),
        }
    }
    fn eval_value_parts(&mut self, current: &Expr) -> Types {
        match current {
            Expr::VarCall(var) => self
                .get_var(&var.name)
                .map(|cell| cell.borrow_mut().clone())
                .unwrap_or(Types::Number(0.0)),

            Expr::Operations(operations) => {
                self.eval_expression(&operations.instructions, operations.is_bool)
            }

            Expr::FuncCall(func_call) => self.eval_function(func_call),
            Expr::String(v) => Types::String(v.to_string()),
            Expr::Number(v) => Types::Number(*v),
            _ => {
                return Types::Number(0.0);
            }
        }
    }

    fn next_if_not(&mut self, expression: &Vec<Expr>, index: &mut usize, not: bool) -> Types {
        let current = &expression[*index];
        match current {
            Expr::NOT => {
                if *index + 1 >= expression.len() {
                    return Types::Number(0.0);
                }
                *index += 1;
                return self.next_if_not(expression, index, not);
            }

            Expr::VarCall(_)
            | Expr::FuncCall(_)
            | Expr::Number(_)
            | Expr::Operations(_)
            | Expr::String(_) => {
                let value = self.eval_value_parts(current);
                return self.eval_not(&value, not);
            }
            _ => {
                return Types::Number(0.0);
            }
        }
    }
    fn eval_boolean_operation(
        &mut self,
        expression: &Vec<Expr>,
        index: &mut usize,
        not: bool,
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
        //
        let value_a = self.next_if_not(expression, index, not);

        *index += 1;
        let token = &expression[*index];

        *index += 1;
        let value_b = self.next_if_not(expression, index, false);

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
    fn eval_boolean_expression(&self, value_a: &Types, value_b: &Types, operation: &Expr) -> Types {
        let bool_a = value_a.to_number() > 0.0;
        let bool_b = value_b.to_number() > 0.0;
        let result = match &operation {
            Expr::OR => bool_a || bool_b,
            Expr::AND => bool_a && bool_b,
            _ => false,
        };
        return Types::Number(if result { 1.0 } else { 0.0 });
    }
    fn eval_not(&self, value: &Types, not: bool) -> Types {
        if not {
            return Types::Number(if value.to_number() > 0.0 { 0.0 } else { 1.1 });
        }
        return value.clone();
    }
    fn eval_arithmetic_expression(
        &self,
        value_a: &Types,
        value_b: &Types,
        operation: &Expr,
    ) -> Types {
        match operation {
            Expr::Add => match (value_a, value_b) {
                (Types::Number(a), Types::Number(b)) => return Types::Number(a + b),
                (Types::Number(a), Types::String(b)) => return Types::String(a.to_string() + b),
                (Types::String(a), Types::Number(b)) => {
                    return Types::String(a.to_owned() + &b.to_string());
                }
                (Types::String(a), Types::String(b)) => {
                    return Types::String(a.to_owned() + &b.to_owned());
                }
            },
            Expr::Subtract => match (value_a, value_b) {
                (Types::Number(a), Types::Number(b)) => return Types::Number(a - b),
                _ => {}
            },
            Expr::Multiply => match (value_a, value_b) {
                (Types::Number(a), Types::Number(b)) => return Types::Number(a * b),
                (Types::Number(a), Types::String(b)) => {
                    return Types::String(b.as_str().repeat(*a as usize));
                }
                (Types::String(a), Types::Number(b)) => {
                    return Types::String(a.as_str().repeat(*b as usize));
                }
                _ => {}
            },
            Expr::Divide => match (value_a, value_b) {
                (Types::Number(a), Types::Number(b)) => return Types::Number(a / b),
                _ => {}
            },
            Expr::Mod => match (value_a, value_b) {
                (Types::Number(a), Types::Number(b)) => return Types::Number(a % b),
                _ => {}
            },

            _ => {}
        }
        return Types::Number(0.0);
    }
    fn eval_previous_expression(
        &mut self,
        expression: &Vec<Expr>,
        is_bool: bool,
        index: &mut usize,
        out: &mut Types,
        operation: &Expr,
        not: bool,
    ) {
        if is_bool {
            // if is bool i should only handle some of the basic operations here
            if let Some(value_b) = self.eval_boolean_operation(expression, index, not) {
                match operation {
                    Expr::OR | Expr::AND => {
                        *out = self.eval_boolean_expression(&out, &value_b, operation);
                        return;
                    }
                    _ => {
                        *out = self.eval_arithmetic_expression(&out, &value_b, operation);
                        return;
                    }
                }
            }
        }
        let value_b = self.eval_value_parts(&expression[*index]);
        *out = self.eval_arithmetic_expression(&out, &value_b, operation);
    }

    pub fn eval_self_modifying_operation(
        &mut self,

        modifying: &ModifyingOperation,
        operation: &Expr,
        out: &mut Types,
    ) {
        let eval = self.eval_expression(&modifying.value.instructions, modifying.value.is_bool);
        let current_val = match self.get_var(&modifying.name) {
            Some(cell) => {
                let borrowed = cell.borrow();
                borrowed.clone()
            }
            None => {
                return;
            }
        };

        let new_value = self.eval_arithmetic_expression(&current_val, &eval, operation);

        if let Some(cell) = self.get_var(&modifying.name) {
            let mut var = cell.borrow_mut();
            *var = new_value;
            *out = var.clone();
        }
    }
    pub fn eval_modifying_expression(&mut self, current: &Expr, out: &mut Types) {
        match current {
            Expr::Increment(v) => {
                if let Some(cell) = self.get_var(&v.name) {
                    let mut var = cell.borrow_mut();
                    if let Types::Number(n) = &mut *var {
                        *n += 1.0;
                        *out = var.clone(); // var es un RefMut<Types>, clonamos el Types interno
                    }
                }
            }

            Expr::Decrement(v) => {
                if let Some(cell) = self.get_var(&v.name) {
                    let mut var = cell.borrow_mut();
                    if let Types::Number(n) = &mut *var {
                        *n -= 1.0; // ← FIX: ahora sí decrementa
                        *out = var.clone();
                    }
                }
            }

            Expr::AddTo(modifying) => {
                self.eval_self_modifying_operation(modifying, &Expr::Add, out);
            }
            Expr::SubtractTo(modifying) => {
                self.eval_self_modifying_operation(modifying, &Expr::Subtract, out);
            }
            Expr::MultiplyTo(modifying) => {
                self.eval_self_modifying_operation(modifying, &Expr::Multiply, out);
            }
            Expr::DivideTo(modifying) => {
                self.eval_self_modifying_operation(modifying, &Expr::Divide, out);
            }
            Expr::ModTo(modifying) => {
                self.eval_self_modifying_operation(modifying, &Expr::Mod, out);
            }
            Expr::VarAssign(var_assign) => {
                let eval =
                    self.eval_expression(&var_assign.value.instructions, var_assign.value.is_bool);
                match self.get_var(&var_assign.name) {
                    Some(cell) => {
                        let mut var = cell.borrow_mut();
                        *var = eval.clone();
                        *out = eval.clone();
                    }
                    None => {
                        self.variables.insert(
                            var_assign.name.to_string(),
                            Rc::new(RefCell::new(eval.clone())),
                        );
                        *out = eval.clone();
                        return;
                    }
                };
            }
            _ => {}
        }
    }
    pub fn eval_expression(&mut self, expression: &Vec<Expr>, is_bool: bool) -> Types {
        let mut out = Types::Number(0.0);
        let mut previous_operation: Option<&Expr> = None;
        let mut i: usize = 0;
        let mut is_not = false;
        while i < expression.len() {
            let current = &expression[i];
            match current {
                Expr::VarCall(_)
                | Expr::FuncCall(_)
                | Expr::Number(_)
                | Expr::Operations(_)
                | Expr::String(_) => {
                    match previous_operation {
                        None => {
                            // okay i obviously should avoid doing this and just
                            // check if the next will be a boolean operation
                            if let Some(value) =
                                self.eval_boolean_operation(expression, &mut i, is_not)
                            {
                                out = value.clone();
                                i += 1;
                                continue;
                            }
                            let value = self.eval_value_parts(current);

                            out = self.eval_not(&value, is_not);
                        }
                        Some(previous_op) => {
                            self.eval_previous_expression(
                                expression,
                                is_bool,
                                &mut i,
                                &mut out,
                                previous_op,
                                is_not,
                            );
                        }
                    }
                    is_not = false;
                }
                Expr::Increment(_)
                | Expr::Decrement(_)
                | Expr::AddTo(_)
                | Expr::SubtractTo(_)
                | Expr::MultiplyTo(_)
                | Expr::DivideTo(_)
                | Expr::ModTo(_)
                | Expr::VarAssign(_) => {
                    self.eval_modifying_expression(current, &mut out);
                }
                Expr::NOT => {
                    is_not = true;
                }
                Expr::Read=>{
                    todo!("read");
                }

                _ => {
                    is_not = false;
                }
            }
            match current {
                Expr::OR
                | Expr::AND
                | Expr::Add
                | Expr::Subtract
                | Expr::Multiply
                | Expr::Divide
                | Expr::Mod => {
                    previous_operation = Some(current);
                }
                _ => {
                    if !is_not {
                        previous_operation = None;
                    }
                }
            }
            i += 1;
        }
        if is_bool {
            return Types::Number(out.to_number());
        }
        return out;
    }
}

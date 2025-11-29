use std::cell::Cell;
use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub enum Types {
    Number(f64),
    String(String),
    // i should later have something else
}
impl Types {
    pub fn to_number(&self) -> f64 {
        match self {
            Types::Number(n) => *n,
            Types::String(s) => s.len() as f64,
        }
    }
}

pub struct Interpreter {
    pub variables: HashMap<String, Cell<Types>>,
    pub functions: HashMap<String, ast::FuncAssign>,
    pub internal_functions: HashMap<String, fn(&Vec<Types>) -> Types>,
    pub previous_context: Option<Box<Interpreter>>,
    pub global_context: Option<Box<Interpreter>>,
}

impl Interpreter {
    pub fn new() -> Interpreter {
        Interpreter {
            variables: HashMap::new(),
            functions: HashMap::new(),
            internal_functions: HashMap::new(),
            previous_context: None,
            global_context: None,
        }
    }

    pub fn get_var(&mut self, var_name: &str) -> Option<&mut Cell<Types>> {
        if let Some(v) = self.variables.get_mut(var_name) {
            return Some(v);
        }
        if let Some(prev) = self.previous_context.as_mut() {
            return prev.get_var(var_name);
        }
        None
    }

    pub fn get_func(&self, function: &String) -> Option<&ast::FuncAssign> {
        if let Some(f) = self.functions.get(function) {
            return Some(f);
        }
        if let Some(v) = self.previous_context.as_ref() {
            match v.get_func(function) {
                Some(f) => return Some(f),
                None => {}
            };
        }

        return None;
    }
    pub fn get_internal(&self, internal_function: &String) -> Option<fn(&Vec<Types>) -> Types> {
        if let Some(v) = self.global_context.as_ref() {
            return v.get_internal(internal_function);
        }
        // Primero busca en el scope actual
        if let Some(f) = self.internal_functions.get(internal_function) {
            return Some(*f); // los punteros a fn son Copy
        }
        return None;
    }
}

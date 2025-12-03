use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub enum Types {
    Number(f64),
    String(String),
    Array(Rc<RefCell<Vec<Types>>>),
    // i should later have something else
}
impl Types {
    pub fn to_number(&self) -> f64 {
        match self {
            Types::Number(n) => *n,
            Types::String(s) => s.len() as f64,
            Types::Array(a) => a.borrow().len() as f64,
        }
    }
    pub fn add(&self, other: &Types) -> Types {
        match (self, other) {
            (Types::Number(a), Types::Number(b)) => return Types::Number(a + b),
            (Types::Number(a), Types::String(b)) => return Types::String(a.to_string() + b),
            (Types::String(a), Types::Number(b)) => {
                return Types::String(a.to_owned() + &b.to_string());
            }
            (Types::String(a), Types::String(b)) => {
                return Types::String(a.to_owned() + &b.to_owned());
            }
            _ => match self {
                Types::Array(a) => {
                    let mut new = a.borrow().clone();
                    new.push(other.clone());
                    return Types::Array(Rc::new(RefCell::new(new)));
                }
                _ => match other {
                    Types::Array(_) => {
                        // this transforms it into an array
                        let n = Types::Array(Rc::new(RefCell::new(vec![self.clone()])));
                        return n.add(other);
                    }
                    _ => {
                        return Types::Number(0.0);
                    }
                },
            },
        }
    }
}

pub enum ReasonsForStopping {
    Error(String),
    BreakStatement,
    ContinueStatement,
    ReturnStatement,
    Finished,
}
pub type Ctx = *mut Interpreter;
#[derive(Debug, Clone)]
pub struct Interpreter {
    pub variables: HashMap<String, Rc<RefCell<Types>>>,
    pub functions: HashMap<String, Rc<RefCell<ast::FuncAssign>>>,
    pub internal_functions: HashMap<String, fn(&Vec<Types>) -> Types>,
    pub previous_context: Option<Ctx>,
    pub global_context: Option<Ctx>,
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

    pub fn get_var(&self, var_name: &str) -> Option<Rc<RefCell<Types>>> {
        if let Some(v) = self.variables.get(var_name) {
            return Some(v.clone());
        }
        if let Some(prev) = self.previous_context.as_ref() {
            unsafe {
                match prev.as_ref() {
                    Some(i) => return i.get_var(var_name),
                    None => {
                        return None;
                    }
                }
            }
        }
        None
    }

    pub fn get_func(&self, function: &String) -> Option<Rc<RefCell<ast::FuncAssign>>> {
        if let Some(f) = self.functions.get(function) {
            return Some(f.clone());
        }
        if let Some(v) = self.previous_context.as_ref() {
            unsafe {
                match v.as_ref() {
                    Some(i) => return i.get_func(function),
                    None => {
                        return None;
                    }
                }
            }
        }

        return None;
    }
    pub fn get_internal(&self, internal_function: &String) -> Option<fn(&Vec<Types>) -> Types> {
        if let Some(v) = self.global_context.as_ref() {
            unsafe {
                match v.as_ref() {
                    Some(i) => return i.get_internal(internal_function),
                    None => {
                        return None;
                    }
                }
            }
        }
        // Primero busca en el scope actual
        if let Some(f) = self.internal_functions.get(internal_function) {
            return Some(*f); // los punteros a fn son Copy
        }
        return None;
    }
    pub fn new_context(&mut self) -> Interpreter {
        let ctx: Ctx = self;
        let mut interpreter = Interpreter::new();
        interpreter.global_context = self.global_context.clone().or(Some(ctx));
        interpreter.previous_context = Some(ctx);
        return interpreter;
    }
}

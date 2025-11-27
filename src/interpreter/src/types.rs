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

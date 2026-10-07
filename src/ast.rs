#[derive(Debug, Clone)]
pub enum Expr {
    Number(f64), 
    Variable(String), 
    Binary {
        op: char, 
        lhs: Box<Expr>, 
        rhs: Box<Expr>,
    },

    Call {
        callee: String, 
        args: Vec<Expr>,
    }
}

#[derive(Debug, Clone)]
pub struct Prototype {
    pub name: String, 
    pub params: Vec<String>,
}

impl Prototype {
    pub fn new(name: impl Into<String>, params: Vec<String>) -> Self {
        Self {name: name.into(), params}
    }
}

pub struct Function {
    pub proto: Prototype, 
    pub body: Expr, 
}

impl Function {
    pub fn new(proto: Prototype, body: Expr) -> Self {
        Self {proto, body}
    }
}
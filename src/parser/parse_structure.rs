/// defining the grammatical structure for the parsing process

use super::parse_context::Ident;

#[derive(Debug,Clone,PartialEq)]
/// sturct for a whole program
/// at the moment only consists of Statements
pub struct Program {
    pub statements: Vec<Statement>,
}

impl Program{
    pub fn new()->Self{
        return Program { statements: Vec::new() }
    }
}

#[derive(Debug,Clone,PartialEq)]
/// Statement types that can be used in this grammer
pub enum Statement {
    Let {
        name: Ident,
        value: Option<Expr>,
    },
    Assign {
        name: Ident,
        value: Expr,
    },
    Loop {
        var: Ident,
        body: Vec<Statement>,
    },
    Print {
        name: Ident,
    },
    Empty
}
#[derive(Debug,Clone,PartialEq)]
/// Expression of this grammer
pub enum Expr {
    Number(usize),
    Ident(Ident),
    Binary {
        left: Box<Expr>,
        op: BinOp,
        right: Box<Expr>,
    },
}

#[derive(Debug,Clone,PartialEq)]
// Binary Operator
pub enum BinOp {
    Add,
    Sub,
}
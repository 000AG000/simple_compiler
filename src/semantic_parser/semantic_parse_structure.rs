//! defining the grammatical structure for the parsing process

use crate::lexer::Span;

use super::semantic_parse_context::Ident;

#[derive(Debug, Clone, PartialEq, Default)]
/// struct for a whole program
/// at the moment only consists of Statements
pub struct Program {
    pub statements: Vec<Statement>,
}

#[derive(Debug, Clone, PartialEq)]
/// Provides the span context for a generic type T
/// Used for Parsed Types
pub struct Spanned<T> {
    pub node: T,
    pub span: Span,
}

impl<T> Spanned<T> {
    pub fn new(node: T, span: Span) -> Self {
        Self { node, span }
    }
}

impl<T> Spanned<T> {
    pub fn lexeme<'a>(&self, input_str: &'a str) -> &'a str {
        debug_assert!(
            (input_str.len() >= self.span.start) | (input_str.len() >= self.span.end),
            "spanned element out of bounds"
        );
        &input_str[self.span.start..self.span.end]
    }
}

pub type Statement = Spanned<StatementKind>;

impl Statement {
    pub fn pretty_print(&self, input_str: &str) -> String {
        match &self.node {
            StatementKind::Let { name, value } => match value {
                Some(val) => format!("Let {} = {}", name.lexeme(input_str), val.lexeme(input_str)),
                None => format!("Let {}", name.lexeme(input_str)),
            },
            StatementKind::Assign { name, value } => format!(
                "Assign {} = {}",
                name.lexeme(input_str),
                value.lexeme(input_str)
            ),
            StatementKind::Loop { var, body: _ } => format!("Loop over {}", var.lexeme(input_str)),
            StatementKind::Print { name } => format!("Print {}", name.lexeme(input_str)),
            StatementKind::Empty => "Empty".to_string(),
        }
    }
}
pub type Expr = Spanned<ExprKind>;
pub type BinOp = Spanned<BinOpKind>;

#[derive(Debug, Clone, PartialEq)]
/// Statement types that can be used in this grammar
pub enum StatementKind {
    Let { name: Ident, value: Option<Expr> },
    Assign { name: Ident, value: Expr },
    Loop { var: Ident, body: Vec<Statement> },
    Print { name: Ident },
    Empty,
}
#[derive(Debug, Clone, PartialEq)]
/// Expression of this grammar
pub enum ExprKind {
    Number(usize),
    Ident(Ident),
    Binary {
        left: Box<Expr>,
        op: BinOp,
        right: Box<Expr>,
    },
}

#[derive(Debug, Clone, PartialEq)]
// Binary Operator
pub enum BinOpKind {
    Add,
    Sub,
}

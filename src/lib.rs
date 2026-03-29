pub mod lexer;
pub mod sem_parser;
pub mod interpreter;

pub use lexer::lex;
pub use sem_parser::parse;
pub use interpreter::exec;
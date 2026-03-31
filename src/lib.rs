pub mod lexer;
pub mod sem_parser;
pub mod interpreter;
pub mod error;

pub use lexer::lex_ascii;
pub use sem_parser::parse;
pub use interpreter::exec;
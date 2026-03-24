/// Lexer module for providing lexical analysis

mod token;
mod lex_table;
mod lex_error;
mod lexer;

pub use token::{Token,Span,TokenKind};
pub use lex_error::{LexError,LexErrorKind};
pub use lexer::lex;
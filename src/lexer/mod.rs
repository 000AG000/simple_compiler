/// Table-driven lexer:
/// Design decisions:
/// - operates on ASCII input only
/// - uses table-driven classification for performance
/// - uses state machine for token construction
/// - stores spans instead of copying lexemesuse std::fmt;

mod token;
mod lex_table;
mod lex_error;
mod lexer;

pub use token::{Token,Span,TokenKind};
pub use lex_error::{LexError,LexErrorKind};
pub use lexer::lex;
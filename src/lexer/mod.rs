//! Lexer module for providing lexical analysis

mod token;
mod lex_table;
mod lex;

pub use token::{Token,Span,TokenKind};
pub use crate::error::{ErrorKind,GlobalError,LexErrorKind}; 
pub use lex::lex_ascii;
//! Lexer module for providing lexical analysis

mod lex;
mod lex_table;
mod token;

pub use crate::error::{ErrorKind, GlobalError, LexErrorKind};
pub use lex::lex_ascii;
pub use token::{Span, Token, TokenKind};

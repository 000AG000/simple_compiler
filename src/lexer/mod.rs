//! Lexer module for providing lexical analysis

mod main_lexer;
mod lex_table;
mod token;

pub use crate::error::{ErrorKind, GlobalError, LexErrorKind};
pub use main_lexer::lex_ascii;
pub use token::{Span, Token, TokenKind};

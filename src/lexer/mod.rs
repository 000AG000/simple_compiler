//! Lexer module for providing lexical analysis

mod lex_table;
mod main_lexer;
mod token;

pub use crate::error::{ErrorKind, GlobalError, LexErrorKind};
pub use main_lexer::lex_ascii;
pub use token::{Span, Token, TokenKind};

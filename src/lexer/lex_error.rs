/// lex_error.rs
/// Defines the LexError struct that saves the error type and the Span associated with the error

use std::{ error::Error, fmt, fmt::Display};
use super::Span;

#[derive(Debug, Clone)]
/// Errors that can occur during the lexanizer process
pub enum LexErrorKind {
    UnknownCharacter(char),
    UnexpectedCharacter(char),
    /// Conversion error expected first type (first string) and god last string
    ConversionError(String, String),
}

#[derive(Debug, Clone)]
/// LexError struct with kind of error and span that it refers to
pub struct LexError {
    pub kind: LexErrorKind,
    pub span: Span,
}

impl Display for LexError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Lexical Analysis error on position {} to {}: {:?}",
            self.span.start, self.span.end, self.kind
        )
    }
}

impl Error for LexError {}


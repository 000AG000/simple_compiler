/// lex_error.rs
/// Defines the LexError struct that saves the error type and the Span associated with the error
use std::{ error::Error, fmt, fmt::Display};
use super::Span;

/// number of characters to visualize ahead when showing an error
const LOOKAHEAD:usize = 20;
/// number of characters to visualize afterwards when showing an error
const LOOKAFTER:usize = 20;


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

impl LexError{
    /// Generate error message enriched with input information
    /// Used to better locate message and use ParseError span information
    pub fn generate_error_msg(&self, input: &str) -> String {
        let str_before = &input[self.span.start.saturating_sub(LOOKAHEAD)..self.span.start];
        let str_content = &input[self.span.start..self.span.end];
        let str_after = &input[self.span.end..if self.span.end + LOOKAFTER < input.len() {
            self.span.end + LOOKAFTER
        } else {
            input.len()
        }];
        format!(
            "error occurred: ...{str_before} here >>> {str_content} <<<{str_after}...\n {}",
            self
        )
    }
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


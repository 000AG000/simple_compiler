pub use crate::lexer::Span;
use crate::lexer::TokenKind;
use std::{error::Error, fmt, fmt::Display};

/// number of characters to visualize ahead when showing an error
pub const LOOK_AHEAD: usize = 20;
/// number of characters to visualize afterwards when showing an error
pub const LOOK_AFTER: usize = 20;

#[derive(Debug, Clone)]
/// storing error context occurring
/// Used for managing a global error with a unified api
pub struct GlobalError {
    pub kind: ErrorKind,
    /// span the error is referring for the given input string of the program to interpret
    pub span: Span,
}

#[derive(Debug, Clone)]
pub enum ErrorKind {
    Lex(LexErrorKind),
    Parse(ParseErrorKind),
    Runtime(RuntimeErrorKind),
}

impl GlobalError {
    /// get the location information about the error
    /// Used for showing the user the specific location where the error occurs
    fn get_location_msg(&self, input_str: &str) -> String {
        let str_before = &input_str[self.span.start.saturating_sub(LOOK_AHEAD)..self.span.start];
        let str_content = &input_str[self.span.start..self.span.end];
        let str_after = &input_str[self.span.end
            ..if self.span.end + LOOK_AFTER < input_str.len() {
                self.span.end + LOOK_AFTER
            } else {
                input_str.len()
            }];
        format!("...{str_before} here >>> {str_content} <<<{str_after}...")
    }
    /// Generate error message enriched with input information
    /// Used to better locate message and use ParseError span information
    pub fn generate_error_msg(&self, input_str: &str) -> String {
        let mut error_string = format!(
            "--- error occurred---\n{}\n",
            self.get_location_msg(input_str)
        );

        error_string.push_str(&format!("{}", self));

        error_string
    }
}

impl Display for GlobalError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self.kind {
            ErrorKind::Lex(lex_error_kind) => write!(
                f,
                "Lexical Analysis error on position {} to {}: {:?}",
                self.span.start, self.span.end, lex_error_kind
            ),
            ErrorKind::Parse(parse_error) => match &parse_error {
                ParseErrorKind::NonExpectedToken(token_expect, token_gotten) => write!(
                    f,
                    "Parsing error: Got non expected token {:?}, expected on of these tokens: {:?}",
                    token_gotten, token_expect
                ),
                ParseErrorKind::IdentifierAlreadyUsed(ident_name, _) => write!(
                    f,
                    "Parsing error: Identifier \"{}\" already in use.",
                    ident_name
                ),
                ParseErrorKind::UnexpectedEOF(token_expect) => write!(
                    f,
                    "Parsing error: Got non unexpected end of file, expected one of these tokens: {:?}",
                    token_expect
                ),
                ParseErrorKind::InternalError(error_string) => {
                    write!(f, "Internal Parser Error: {}", error_string)
                }
                ParseErrorKind::IdentifierNotKnown(ident_str) => {
                    write!(f, "Parsing error: Identifier not defined: {}", ident_str,)
                }
                ParseErrorKind::UnclosedLoop => write!(f, "Parsing error: Unclosed loop",),
                ParseErrorKind::UnexpectedEnd => write!(f, "Parsing error: Unexpected loop end",),
            },
            ErrorKind::Runtime(runtime_error_kind) => match &runtime_error_kind {
                RuntimeErrorKind::InternalError(error_str) => {
                    write!(f, "Runtime error: {}", error_str)
                }
                RuntimeErrorKind::VariableAlreadyDefined => {
                    write!(f, "Runtime error: Variable already defined")
                }
            },
        }
    }
}

impl Error for GlobalError {}

// ----------------------------------------------
// Specific error kinds
// ----------------------------------------------

#[derive(Debug, Clone)]
/// Errors that can occur during the tokenization process
pub enum LexErrorKind {
    UnknownCharacter(char),
    UnexpectedCharacter(char),
    /// Conversion error expected first type (first string) and god last string
    ConversionError(String, String),
}

#[derive(Debug, Clone)]
pub enum ParseErrorKind {
    /// NonExpectedToken(expected TokenKinds, gotten TokenKind)
    NonExpectedToken(Vec<TokenKind>, TokenKind),
    /// UnexpectedEOF(expected TokenKinds)
    UnexpectedEOF(Vec<TokenKind>),
    /// Identifier is already used
    /// - String for Identification
    /// - Span: where Identifier was already defined
    IdentifierAlreadyUsed(String, Span),
    /// Identifier  is not known
    /// - String for Identification
    IdentifierNotKnown(String),
    /// Unclosed loop
    UnclosedLoop,
    /// Unexpected End of loop
    UnexpectedEnd,
    InternalError(String),
}

#[derive(Debug, Clone)]
/// Errors that can occur during the interpreting process
pub enum RuntimeErrorKind {
    InternalError(String),
    VariableAlreadyDefined,
}

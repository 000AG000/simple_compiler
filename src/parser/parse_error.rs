use crate::lexer::Span;
use crate::lexer::TokenKind;
/// parse_error
/// Defines the Parse Error struct that saves the error type and the Span associated with the error
use std::{error::Error, fmt, fmt::Display};

const LOOKAHEAD: usize = 20;
const LOOKAFTER: usize = 20;

#[derive(Debug, Clone)]
/// Errors that can occur during the parsing process
pub enum ParseErrorKind {
    /// NonExpectedToken(expected TokenKinds, gotten TokenKind)
    NonExpectedToken(Vec<TokenKind>, TokenKind),
    /// UnexpectedEOF(expected TokenKinds)
    UnexpectedEOF(Vec<TokenKind>),
    /// Inditificator is already used
    /// - String for Identification
    /// - Span: where Identificator was aready defined
    IdentificatorAlreadyUsed(String, Span),
    /// Identificator is not kown
    /// - String for Identification
    IdentificatorNotKnown(String),
    /// Unclosed loop
    UnclosedLoop,
    /// Unexpected End of loop
    UnexpectedEnd,
    InternalError(String),
}

#[derive(Debug, Clone)]
/// LexError struct with kind of error and tokens that it refers to
pub struct ParseError {
    pub kind: ParseErrorKind,
    /// assotiated span where the error occures
    pub span: Span,
}

impl ParseError {
    /// Generate error message enriched with input information
    /// Used to better locate message and use ParseError span information
    pub fn generate_error_msg(&self, input: &str) -> String {
        let str_before = &input[if self.span.start > LOOKAHEAD {
            self.span.start - LOOKAHEAD
        } else {
            0
        }..self.span.start];
        let str_content = &input[self.span.start..self.span.end];
        let str_after = &input[self.span.end..if self.span.end + LOOKAFTER < input.len() {
            self.span.end + LOOKAFTER
        } else {
            input.len()
        }];
        format!(
            "error occurred: ...{str_before} here >>> -{str_content} <<<{str_after}...\n {}",
            self
        )
    }
}

impl Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self.kind {
            ParseErrorKind::NonExpectedToken(token_expect, token_gotten) => write!(
                f,
                "Parsing error: Got non expected token {:?}, expected on of these tokens: {:?}",
                token_gotten, token_expect
            ),
            ParseErrorKind::IdentificatorAlreadyUsed(ident_name, _) => write!(
                f,
                "Parsing error: Identificator \"{}\" already in use.",
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
            ParseErrorKind::IdentificatorNotKnown(ident_str) => {
                write!(f, "Parsing error: Identificator not defined: {}", ident_str,)
            }
            ParseErrorKind::UnclosedLoop => write!(f, "Parsing error: Unclosed loop",),
            ParseErrorKind::UnexpectedEnd => write!(f, "Parsing error: Unexpeted loop end",),
        }
    }
}

impl Error for ParseError {}

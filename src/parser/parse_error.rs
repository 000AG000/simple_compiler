use crate::lexer::Span;
use crate::lexer::{Token, TokenKind};
/// parse_error
/// Defines the Parse Error struct that saves the error type and the Span associated with the error
use std::{error::Error, fmt, fmt::Display};

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
    /// - Identification of loop
    UnclosedLoop(Span),
    /// Unexpected End of loop
    /// - Span of End token   
    UnexpectedEnd(Span),
    InternalError(String),
}

#[derive(Debug, Clone)]
/// LexError struct with kind of error and tokens that it refers to
pub struct ParseError {
    pub kind: ParseErrorKind,
    /// assotiated tokens where error occurs
    pub associated_tokens: Vec<Token>,
}

impl Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let start_position = self.associated_tokens.first().map(|t| t.span.start);
        let end_position = self.associated_tokens.last().map(|t| t.span.end);

        let start = match start_position {
            Some(position) => position.to_string(),
            None => "(not known)".to_string(),
        };

        let end = match end_position {
            Some(position) => position.to_string(),
            None => "(not known)".to_string(),
        };

        match &self.kind {
            ParseErrorKind::NonExpectedToken(token_expect, token_gotten) => write!(
                f,
                "Parsing error: Got non expected token {:?} at position {} to  {} expected on of these tokens: {:?}",
                token_gotten, start, end, token_expect
            ),
            ParseErrorKind::IdentificatorAlreadyUsed(ident_name, span) => write!(
                f,
                "Parsing error: Identificator \"{}\" newly defined at position {} to {} already in use. First defined at {} to {}",
                ident_name, start, end, span.start, span.end
            ),
            ParseErrorKind::UnexpectedEOF(token_expect) => write!(
                f,
                "Parsing error: Got non unexpected end of file, expected one of these tokens: {:?}",
                token_expect
            ),
            ParseErrorKind::InternalError(error_string) => write!(
                f,
                "Internal Parser Error: {}",
                error_string
            ),
            ParseErrorKind::IdentificatorNotKnown(ident_str) => write!(
                f,
                "Parsing error: Identificator not defined: {}",
                ident_str,
            ),
            ParseErrorKind::UnclosedLoop(span) => write!(
                f,
                "Parsing error: Unclosed loop start at position {} to {}",
                span.start,span.end,
            ),
            ParseErrorKind::UnexpectedEnd(span) => write!(
                f,
                "Parsing error: Unexpeted loop end at position {} to {}",
                span.start,span.end,
            ),
                    }
    }
}

impl Error for ParseError {}

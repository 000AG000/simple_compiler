use crate::{
    lexer::{Span, TokenKind},
    sem_parser::{
        GlobalError,ErrorKind, ParseErrorKind,
    },
};



/// Small helper function for creating NonExpectedToken Errors
pub fn give_non_expected_token_error(
    got_token_kind: &TokenKind,
    expected_token_kinds: Vec<TokenKind>,
    associated_span: Span,
) -> GlobalError{
    GlobalError {
        kind: ErrorKind::Parse(ParseErrorKind::NonExpectedToken(expected_token_kinds, *got_token_kind)),
        span: associated_span,
    }
}
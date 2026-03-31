use std::collections::HashMap;

use log::trace;

use super::lex_table::{LexTable, LexTableEntry};
use super::token::{Span, Token, TokenKind, get_keyword_map};
use super::{ErrorKind, GlobalError, LexErrorKind};

/// tokenization function for transforming string into token vector
/// Assumes ASCII input only.
/// Non-ASCII input will lead to error
///
/// when file does not follow its function you get an LexError
///
/// example usage:
/// ```
/// use simple_interpreter::lexer::lex_ascii;
/// let lex_input = "let x = 0;";
/// let token_vec = lex_ascii(lex_input).unwrap();
/// ```
///
pub fn lex_ascii(input_str: &str) -> Result<Vec<Token>, GlobalError> {
    let mut lexed_tokens: Vec<Token> = Vec::new();
    let lex_table = LexTable::new();
    let keyword_map = get_keyword_map();

    #[derive(Debug)]
    enum LexState {
        Normal,
        SpaceNeeding,
        /// Number with the corresponding span
        Number(Span),
        /// Identifier with the corresponding span
        Ident(Span),
    }

    fn handle_string_end(
        state: LexState,
        token_vec: &mut Vec<Token>,
        keyword_map: &HashMap<&str, TokenKind>,
        input_str: &str,
    ) -> Result<(), GlobalError> {
        match state {
            LexState::Number(span) => {
                let number_str = span.lexeme(input_str);
                let token = Token {
                    kind: TokenKind::Number(match number_str.parse::<usize>() {
                        Ok(number) => number,
                        Err(_) => {
                            return Err(GlobalError {
                                kind: ErrorKind::Lex(LexErrorKind::ConversionError(
                                    "number".to_string(),
                                    number_str.to_string(),
                                )),
                                span,
                            });
                        }
                    }),
                    span,
                };

                trace!("token added: {:?}", &token);
                token_vec.push(token);
            }
            LexState::Ident(span) => {
                let token = match keyword_map.get(span.lexeme(input_str) as &str) {
                    Some(token) => Token { kind: *token, span },
                    None => Token {
                        kind: TokenKind::Ident,
                        span,
                    },
                };
                trace!("token added: {:?}", &token);
                token_vec.push(token);
            }
            _ => (),
        }
        Ok(())
    }
    let mut lex_state = LexState::Normal;
    // iterate over file buffer
    for (position, next_elem) in input_str.bytes().enumerate() {
        // return non ASCII characters
        if !next_elem.is_ascii() {
            return Err(GlobalError {
                kind: ErrorKind::Lex(LexErrorKind::UnknownCharacter(next_elem as char)),
                span: Span {
                    start: position,
                    end: position + 1,
                },
            });
        }

        let lex_entry = lex_table.classify(next_elem);

        lex_state = match (lex_state, lex_entry) {
            (LexState::Normal, LexTableEntry::Split) => LexState::Normal,
            (LexState::Normal, LexTableEntry::Token(token)) => {
                trace!("token added: {:?}", *token);
                lexed_tokens.push(Token {
                    kind: *token,
                    span: Span {
                        start: position,
                        end: position + 1,
                    },
                });
                LexState::Normal
            }
            (LexState::Normal, LexTableEntry::SpaceNeedingToken(token)) => {
                trace!("token added: {:?}", *token);
                lexed_tokens.push(Token {
                    kind: *token,
                    span: Span {
                        start: position,
                        end: position + 1,
                    },
                });
                LexState::SpaceNeeding
            }
            (LexState::Normal, LexTableEntry::Alphabetic) => LexState::Ident(Span {
                start: position,
                end: position + 1,
            }),
            (LexState::Ident(Span { start, .. }), LexTableEntry::Alphabetic) => {
                LexState::Ident(Span {
                    start,
                    end: position + 1,
                })
            }
            (_, LexTableEntry::Alphabetic) => {
                return Err(GlobalError {
                    kind: ErrorKind::Lex(LexErrorKind::UnexpectedCharacter(next_elem as char)),
                    span: Span {
                        start: position,
                        end: position + 1,
                    },
                });
            }
            (LexState::Normal, LexTableEntry::Numeric) => LexState::Number(Span {
                start: position,
                end: position + 1,
            }),
            (LexState::Ident(Span { start, .. }), LexTableEntry::Numeric) => {
                LexState::Ident(Span {
                    start,
                    end: position + 1,
                })
            }
            (LexState::Number(Span { start, .. }), LexTableEntry::Numeric) => {
                handle_string_end(
                    LexState::Number(Span {
                        start,
                        end: position + 1,
                    }),
                    &mut lexed_tokens,
                    &keyword_map,
                    input_str,
                )?;
                LexState::Normal
            }
            (_, LexTableEntry::Numeric) => {
                return Err(GlobalError {
                    kind: ErrorKind::Lex(LexErrorKind::UnexpectedCharacter(next_elem as char)),
                    span: Span {
                        start: position,
                        end: position + 1,
                    },
                });
            }
            (_, LexTableEntry::Undefined) => {
                return Err(GlobalError {
                    kind: ErrorKind::Lex(LexErrorKind::UnknownCharacter(next_elem as char)),
                    span: Span {
                        start: position,
                        end: position + 1,
                    },
                });
            }
            (LexState::SpaceNeeding, LexTableEntry::Split) => LexState::Normal,
            (LexState::SpaceNeeding, _) => {
                return Err(GlobalError {
                    kind: ErrorKind::Lex(LexErrorKind::UnexpectedCharacter(next_elem as char)),
                    span: Span {
                        start: position,
                        end: position + 1,
                    },
                });
            }
            (LexState::Number(span), LexTableEntry::Split) => {
                handle_string_end(
                    LexState::Number(span),
                    &mut lexed_tokens,
                    &keyword_map,
                    input_str,
                )?;
                LexState::Normal
            }
            (LexState::Ident(span), LexTableEntry::Split) => {
                handle_string_end(
                    LexState::Ident(span),
                    &mut lexed_tokens,
                    &keyword_map,
                    input_str,
                )?;
                LexState::Normal
            }
            (LexState::Number(span), LexTableEntry::Token(token_kind)) => {
                handle_string_end(
                    LexState::Number(span),
                    &mut lexed_tokens,
                    &keyword_map,
                    input_str,
                )?;
                trace!("token added: {:?}", *token_kind);
                lexed_tokens.push(Token {
                    kind: *token_kind,
                    span: Span {
                        start: position,
                        end: position + 1,
                    },
                });
                LexState::Normal
            }
            (LexState::Number(span), LexTableEntry::SpaceNeedingToken(token_kind)) => {
                handle_string_end(
                    LexState::Number(span),
                    &mut lexed_tokens,
                    &keyword_map,
                    input_str,
                )?;
                trace!("token added: {:?}", *token_kind);
                lexed_tokens.push(Token {
                    kind: *token_kind,
                    span: Span {
                        start: position,
                        end: position + 1,
                    },
                });
                LexState::SpaceNeeding
            }
            (LexState::Ident(Span { start, .. }), LexTableEntry::Token(token_kind)) => {
                handle_string_end(
                    LexState::Ident(Span {
                        start,
                        end: position,
                    }),
                    &mut lexed_tokens,
                    &keyword_map,
                    input_str,
                )?;
                trace!("token added: {:?}", *token_kind);
                lexed_tokens.push(Token {
                    kind: *token_kind,
                    span: Span {
                        start: position,
                        end: position + 1,
                    },
                });
                LexState::Normal
            }
            (LexState::Ident(Span { start, .. }), LexTableEntry::SpaceNeedingToken(token_kind)) => {
                handle_string_end(
                    LexState::Ident(Span {
                        start,
                        end: position,
                    }),
                    &mut lexed_tokens,
                    &keyword_map,
                    input_str,
                )?;
                trace!("token added: {:?}", *token_kind);
                lexed_tokens.push(Token {
                    kind: *token_kind,
                    span: Span {
                        start: position,
                        end: position + 1,
                    },
                });
                LexState::SpaceNeeding
            }
        };

        // position is byte index (ASCII-only assumption)
    }

    // handle end state
    handle_string_end(lex_state, &mut lexed_tokens, &keyword_map, input_str)?;

    // add end of file token
    trace!("token added: {:?}", TokenKind::EOF);
    lexed_tokens.push(Token {
        kind: TokenKind::EOF,
        span: Span {
            start: input_str.len(),
            end: input_str.len(),
        },
    });
    Ok(lexed_tokens)
}

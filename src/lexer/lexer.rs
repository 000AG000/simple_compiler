use std::collections::HashMap;

use super::lex_table::{LexTable,LexTableEntry};
use super::lex_error::{LexError,LexErrorKind};
use super::token::{Token,TokenKind,Span,get_keyword_map};


/// lexanizer function transfor file into token vector
/// Assumes ASCII input only.
/// Non-ASCII input will lead to error
///
/// when file does not follow its function you get an LexError
///
/// example usage:
/// ```
/// let lex_input = std::fs::read_to_string("tests/example_files/simple_test.ms").unwrap();
/// let token_vec = lex(file)?;
/// ```
///
pub fn lex(lex_input: &str) -> Result<Vec<Token>, LexError> {
    let mut lexed_tokens: Vec<Token> = Vec::with_capacity(1000);
    let lex_table = LexTable::new();
    let keyword_map = get_keyword_map();

    #[derive(Debug)]
    enum LexState {
        Normal,
        SpaceNeeding,
        /// Number(start position, string)
        Number(usize, String),
        /// Ident(start position, string)
        Ident(usize, String),
    }

    fn handle_string_end(
        state: LexState,
        token_vec: &mut Vec<Token>,
        keyword_map: &HashMap<&str, TokenKind>,
        end_position: usize,
    ) -> Result<(), LexError> {
        match state {
            LexState::Number(start_position, number_str) => token_vec.push(Token {
                kind: TokenKind::Number(match number_str.parse::<usize>() {
                    Ok(number) => number,
                    Err(_) => {
                        return Err(LexError {
                            kind: LexErrorKind::ConversionError(
                                "number".to_string(),
                                number_str.to_string(),
                            ),
                            span: Span {
                                start: start_position,
                                end: end_position,
                            },
                        });
                    }
                }),
                span: Span {
                    start: start_position,
                    end: end_position,
                },
            }),
            LexState::Ident(start_position, ident) => match keyword_map.get(&ident as &str) {
                Some(token) => token_vec.push(Token {
                    kind: *token,
                    span: Span {
                        start: start_position,
                        end: end_position,
                    },
                }),
                None => token_vec.push(Token {
                    kind: TokenKind::Ident,
                    span: Span {
                        start: start_position,
                        end: end_position,
                    },
                }),
            },
            _ => (),
        }
        Ok(())
    }
    let mut lex_state = LexState::Normal;
    let mut position = 0;
    // iterate over file buffer
    for next_elem in lex_input.bytes() {

        // return non ASCII characters
        if ! next_elem.is_ascii(){
            return Err(LexError { kind: LexErrorKind::UnknownCharacter(next_elem as char), span: Span { start: position, end: position+1 } })
        }

        let lex_entry = lex_table.classify(next_elem);

        lex_state = match (lex_state, lex_entry) {
            (LexState::Normal, LexTableEntry::Split) => LexState::Normal,
            (LexState::Normal, LexTableEntry::Token(token)) => {
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
                lexed_tokens.push(Token {
                    kind: *token,
                    span: Span {
                        start: position,
                        end: position + 1,
                    },
                });
                LexState::SpaceNeeding
            }
            (LexState::Normal, LexTableEntry::Alphabetic) => {
                let mut string = String::with_capacity(8);
                string.push(next_elem as char);
                LexState::Ident(position, string)
            }
            (LexState::Ident(start_postion, mut string), LexTableEntry::Alphabetic) => {
                string.push(next_elem as char);
                LexState::Ident(start_postion, string)
            }
            (_, LexTableEntry::Alphabetic) => {
                return Err(LexError {
                    kind: LexErrorKind::UnexpectedCharacter(next_elem as char),
                    span: Span {
                        start: position,
                        end: position + 1,
                    },
                });
            }
            (LexState::Normal, LexTableEntry::Numeric) => {
                let mut string = String::with_capacity(8);
                string.push(next_elem as char);
                LexState::Number(position, string)
            }
            (LexState::Ident(start_position, mut string), LexTableEntry::Numeric) => {
                string.push(next_elem as char);
                LexState::Ident(start_position, string)
            }
            (_, LexTableEntry::Numeric) => {
                return Err(LexError {
                    kind: LexErrorKind::UnexpectedCharacter(next_elem as char),
                    span: Span {
                        start: position,
                        end: position + 1,
                    },
                });
            }
            (_, LexTableEntry::Undefined) => {
                return Err(LexError {
                    kind: LexErrorKind::UnknownCharacter(next_elem as char),
                    span: Span {
                        start: position,
                        end: position + 1,
                    },
                });
            }
            (LexState::SpaceNeeding, LexTableEntry::Split) => LexState::Normal,
            (LexState::SpaceNeeding, _) => {
                return Err(LexError {
                    kind: LexErrorKind::UnexpectedCharacter(next_elem as char),
                    span: Span {
                        start: position,
                        end: position + 1,
                    },
                });
            }
            (LexState::Number(start_position, in_string), LexTableEntry::Split) => {
                handle_string_end(
                    LexState::Number(start_position, in_string),
                    &mut lexed_tokens,
                    &keyword_map,
                    position,
                )?;
                LexState::Normal
            }
            (LexState::Ident(start_position, in_string), LexTableEntry::Split) => {
                handle_string_end(
                    LexState::Ident(start_position, in_string),
                    &mut lexed_tokens,
                    &keyword_map,
                    position,
                )?;
                LexState::Normal
            }
            (LexState::Number(start_position, string), LexTableEntry::Token(token_kind)) => {
                handle_string_end(
                    LexState::Number(start_position, string),
                    &mut lexed_tokens,
                    &keyword_map,
                    position,
                )?;
                lexed_tokens.push(Token {
                    kind: *token_kind,
                    span: Span {
                        start: position,
                        end: position + 1,
                    },
                });
                LexState::Normal
            }
            (
                LexState::Number(start_position, string),
                LexTableEntry::SpaceNeedingToken(token_kind),
            ) => {
                handle_string_end(
                    LexState::Number(start_position, string),
                    &mut lexed_tokens,
                    &keyword_map,
                    position,
                )?;
                lexed_tokens.push(Token {
                    kind: *token_kind,
                    span: Span {
                        start: position,
                        end: position + 1,
                    },
                });
                LexState::SpaceNeeding
            }
            (LexState::Ident(start_position, string), LexTableEntry::Token(token_kind)) => {
                handle_string_end(
                    LexState::Ident(start_position, string),
                    &mut lexed_tokens,
                    &keyword_map,
                    position,
                )?;
                lexed_tokens.push(Token {
                    kind: *token_kind,
                    span: Span {
                        start: position,
                        end: position + 1,
                    },
                });
                LexState::Normal
            }
            (
                LexState::Ident(start_position, string),
                LexTableEntry::SpaceNeedingToken(token_kind),
            ) => {
                handle_string_end(
                    LexState::Ident(start_position, string),
                    &mut lexed_tokens,
                    &keyword_map,
                    position,
                )?;
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
        position += 1;
    }

    // handle end state
    handle_string_end(lex_state, &mut lexed_tokens, &keyword_map, position)?;
    return Ok(lexed_tokens);
}
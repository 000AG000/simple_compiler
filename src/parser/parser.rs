use std::{iter::Peekable, slice::Iter};

use super::parser_helper_func::*;

use crate::{
    lexer::{Span, Token, TokenKind},
    parser::{
        ParseError, ParseErrorKind, Statement,
        parse_context::{IdentKind, ParseContext},
    },
};

use super::Program;

/// Parser for parsing tokens from lexical analysis to simplified AST
///
/// Design choises
/// - parsers uses stack to see in what loop the statements needs to be added
/// - build up parse context for storing information like variables

pub fn parse(input_tokens: &Vec<Token>, input_str: &str) -> Result<Program, ParseError> {
    let mut program = Program::new();

    let mut parse_stack = vec![(None, Vec::new())];
    let mut input_iter: Peekable<Iter<'_, Token>> = input_tokens.iter().peekable();

    let mut parse_context = ParseContext::new();

    let mut expected_token_kinds;

    while let Some(token) = input_iter.next() {
        match (
            token.span,
            token.kind,
            //parse_state_vec.pop().or(Some(ParseState::Idle)).unwrap(),
        ) {
            (span, token_kind) => {
                let mut associated_tokens = vec![token];
                match token_kind {
                    TokenKind::Let => {
                        expected_token_kinds = vec![TokenKind::Ident];

                        // test for Ident token
                        let mut token = read_next_token(
                            &mut input_iter,
                            &expected_token_kinds,
                            &mut associated_tokens,
                        )?;

                        let ident = match token {
                            Token {
                                kind: TokenKind::Ident,
                                span: ident_span,
                            } => {
                                let ident_def_span = Span {
                                    start: span.start,
                                    end: ident_span.end,
                                };
                                parse_context.new_ident(
                                    token.lexeme(input_str),
                                    IdentKind::Variable,
                                    ident_def_span,
                                    token.clone(),
                                )?
                            }
                            _ => {
                                return Err(give_non_expected_token_error(
                                    &token.kind,
                                    expected_token_kinds,
                                    &mut associated_tokens,
                                ));
                            }
                        };

                        expected_token_kinds =
                            vec![TokenKind::Semicolon, TokenKind::Newline, TokenKind::Equal];

                        token = read_next_token(
                            &mut input_iter,
                            &expected_token_kinds,
                            &mut associated_tokens,
                        )?;

                        match token {
                            Token {
                                kind: TokenKind::Semicolon | TokenKind::Newline,
                                ..
                            } => {
                                let new_statement = Statement::Let {
                                    name: ident,
                                    value: None,
                                };
                                add_statement_with_token_ref(
                                    &associated_tokens,
                                    &mut parse_stack,
                                    new_statement,
                                )?;
                                continue; // early return when only declaration
                            }
                            Token {
                                kind: TokenKind::Equal,
                                ..
                            } => (),

                            token => {
                                return Err(give_non_expected_token_error(
                                    &token.kind,
                                    expected_token_kinds,
                                    &mut associated_tokens,
                                ));
                            }
                        }

                        let expr = read_in_expr(
                            &mut input_iter,
                            input_str,
                            &mut associated_tokens,
                            &parse_context,
                        )?;

                        read_in_end(&mut input_iter, &mut associated_tokens)?;

                        let new_statement = Statement::Let {
                            name: ident,
                            value: Some(expr),
                        };
                        add_statement_with_token_ref(
                            &associated_tokens,
                            &mut parse_stack,
                            new_statement,
                        )?;
                    }
                    kind @ (TokenKind::Equal
                    | TokenKind::Plus
                    | TokenKind::Minus
                    | TokenKind::Do
                    | TokenKind::Number(_)) => {
                        return Err(give_non_expected_token_error(
                            &kind,
                            vec![
                                TokenKind::Let,
                                TokenKind::Loop,
                                TokenKind::Print,
                                TokenKind::Ident,
                            ],
                            &mut associated_tokens,
                        ));
                    }
                    TokenKind::Newline | TokenKind::Semicolon => {
                        add_statement_with_token_ref(
                            &associated_tokens,
                            &mut parse_stack,
                            Statement::Empty,
                        )?;
                    }
                    TokenKind::Loop => {
                        let mut expected_token_kinds = vec![TokenKind::Ident];

                        // test for Ident token
                        let mut token = read_next_token(
                            &mut input_iter,
                            &expected_token_kinds,
                            &mut associated_tokens,
                        )?;

                        let ident = match token {
                            Token {
                                kind: TokenKind::Ident,
                                span: _,
                            } => parse_context
                                .classify(token.lexeme(&input_str), &associated_tokens)?,
                            _ => {
                                return Err(give_non_expected_token_error(
                                    &token.kind,
                                    expected_token_kinds,
                                    &mut associated_tokens,
                                ));
                            }
                        };

                        expected_token_kinds = vec![TokenKind::Do];

                        token = read_next_token(
                            &mut input_iter,
                            &expected_token_kinds,
                            &mut associated_tokens,
                        )?;

                        match token {
                            Token {
                                kind: TokenKind::Do,
                                span: _,
                            } => (),
                            _ => {
                                return Err(give_non_expected_token_error(
                                    &token_kind,
                                    expected_token_kinds,
                                    &mut associated_tokens,
                                ));
                            }
                        };

                        parse_stack.push((
                            Some((
                                ident,
                                associated_tokens.iter().map(|t| (*t).clone()).collect(),
                            )),
                            Vec::new(),
                        ));
                    }
                    TokenKind::End => match parse_stack.pop() {
                        Some((Some((ident, mut associated_loop_tokens)), statements)) => {
                            associated_loop_tokens.push(token.clone());
                            let new_statement = Statement::Loop {
                                var: ident,
                                body: statements,
                            };
                            add_statement(
                                &associated_loop_tokens,
                                &mut parse_stack,
                                new_statement,
                            )?;
                        }
                        _ => {
                            return Err(ParseError {
                                kind: ParseErrorKind::UnexpectedEnd(token.span),
                                associated_tokens: vec![token.clone()],
                            });
                        }
                    },
                    TokenKind::Print => {
                        let ident = read_in_ident(
                            &mut input_iter,
                            input_str,
                            &mut associated_tokens,
                            &parse_context,
                        )?;
                        let new_statement = Statement::Print { name: ident };
                        add_statement_with_token_ref(
                            &associated_tokens,
                            &mut parse_stack,
                            new_statement,
                        )?;
                    }
                    TokenKind::Ident => {
                        let ident =
                            parse_context.classify(token.lexeme(input_str), &associated_tokens)?;

                        expected_token_kinds = vec![TokenKind::Equal];

                        let token = read_next_token(
                            &mut input_iter,
                            &expected_token_kinds,
                            &mut associated_tokens,
                        )?;

                        match token {
                            Token {
                                kind: TokenKind::Equal,
                                ..
                            } => (),

                            token => {
                                return Err(give_non_expected_token_error(
                                    &token.kind,
                                    expected_token_kinds,
                                    &mut associated_tokens,
                                ));
                            }
                        }

                        let expr = read_in_expr(
                            &mut input_iter,
                            input_str,
                            &mut associated_tokens,
                            &parse_context,
                        )?;

                        read_in_end(&mut input_iter, &mut associated_tokens)?;

                        let new_statement = Statement::Assign {
                            name: ident,
                            value: expr,
                        };
                        add_statement_with_token_ref(
                            &associated_tokens,
                            &mut parse_stack,
                            new_statement,
                        )?;
                    }
                }
            }
        }
    }

    match parse_stack.pop() {
        Some((None, statements)) => {
            program.statements = statements;
        }
        Some((Some((_, associated_tokens)), _)) => {
            let begin = associated_tokens
                .first()
                .unwrap_or(&Token {
                    kind: TokenKind::Loop,
                    span: Span { start: 0, end: 0 },
                })
                .span
                .start;
            let end = associated_tokens
                .last()
                .unwrap_or(&Token {
                    kind: TokenKind::Loop,
                    span: Span { start: 0, end: 0 },
                })
                .span
                .end;

            return Err(ParseError {
                kind: ParseErrorKind::UnclosedLoop(Span {
                    start: begin,
                    end: end,
                }),
                associated_tokens: associated_tokens,
            });
        }
        None => {
            return Err(ParseError {
                kind: ParseErrorKind::InternalError("Parse stack corruption".to_string()),
                associated_tokens: vec![],
            });
        }
    }

    Ok(program)
}

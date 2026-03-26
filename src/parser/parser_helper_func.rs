/// parser_helper_func.rs
///
/// Helper function to
use std::{iter::Peekable, slice::Iter};

use crate::{
    lexer::{Token, TokenKind},
    parser::{
        Expr, ParseError, ParseErrorKind, Statement,
        parse_context::{Ident, ParseContext},
        parse_structure::BinOp,
    },
};

/// read next token from an iterator
/// throws an error when no next token
pub fn read_next_token<'a>(
    input_iter: &mut Peekable<Iter<'a, Token>>,
    expected_tokenkinds: &Vec<TokenKind>,
    associated_tokens: &mut Vec<&'a Token>,
) -> Result<&'a Token, ParseError> {
    match input_iter.next() {
        Some(t) => {
            associated_tokens.push(t);
            Ok(t)
        }
        None => {
            return Err(ParseError {
                kind: ParseErrorKind::UnexpectedEOF(expected_tokenkinds.clone()),
                associated_tokens: associated_tokens
                    .iter()
                    .map(|token| (*token).clone())
                    .collect(),
            });
        }
    }
}

/// read in expression from token iterator
pub fn read_in_expr<'a>(
    input_iter: &mut Peekable<Iter<'a, Token>>,
    // string the token are refering to
    input_str: &str,
    associated_tokens: &mut Vec<&'a Token>,
    parse_context: &ParseContext,
) -> Result<Expr, ParseError> {
    let mut expr =
        read_in_non_operand_expr(input_iter, input_str, associated_tokens, parse_context)?;

    while let Some(Token {
        kind: token_kind, ..
    }) = input_iter.peek()
    {
        let op = match token_kind {
            TokenKind::Plus => BinOp::Add,
            TokenKind::Minus => BinOp::Sub,
            _ => break,
        };
        input_iter.next();

        let right_expr =
            read_in_non_operand_expr(input_iter, input_str, associated_tokens, parse_context)?;
        expr = Expr::Binary {
            left: Box::new(expr),
            op,
            right: Box::new(right_expr),
        };
    }

    Ok(expr)
}

/// reading in a expression that is no operand
pub fn read_in_non_operand_expr<'a>(
    mut input_iter: &mut Peekable<Iter<'a, Token>>,
    input_str: &str,
    mut associated_tokens: &mut Vec<&'a Token>,
    parse_context: &ParseContext,
) -> Result<Expr, ParseError> {
    let expected_token_kinds = vec![TokenKind::Ident, TokenKind::Number(0)];
    let token = read_next_token(
        &mut input_iter,
        &expected_token_kinds,
        &mut associated_tokens,
    )?;
    Ok(match token {
        Token {
            kind: TokenKind::Ident,
            ..
        } => Expr::Ident(parse_context.classify(token.lexeme(&input_str), &associated_tokens)?),
        Token {
            kind: TokenKind::Number(num),
            ..
        } => Expr::Number(*num),
        _ => {
            return Err(give_non_expected_token_error(
                &token.kind,
                expected_token_kinds,
                &mut associated_tokens,
            ));
        }
    })
}


/// read in end tokens (Semicolon or Newline)
pub fn read_in_end<'a>(
    mut input_iter: &mut Peekable<Iter<'a, Token>>,
    mut associated_tokens: &mut Vec<&'a Token>,
) -> Result<(), ParseError> {
    let expected_token_kinds = vec![TokenKind::Semicolon, TokenKind::Newline];

    let token = read_next_token(
        &mut input_iter,
        &expected_token_kinds,
        &mut associated_tokens,
    )?;

    match token {
        Token {
            kind: TokenKind::Semicolon | TokenKind::Newline,
            ..
        } => Ok(()),

        token => {
            return Err(give_non_expected_token_error(
                &token.kind,
                expected_token_kinds,
                &mut associated_tokens,
            ));
        }
    }
}

/// Reading in an Identificator (Variable etc.)
pub fn read_in_ident<'a>(
    mut input_iter: &mut Peekable<Iter<'a, Token>>,
    input_str: &str,
    mut associated_tokens: &mut Vec<&'a Token>,
    parse_context: &ParseContext,
) -> Result<Ident, ParseError> {
    let expected_token_kinds = vec![TokenKind::Ident];

    let token = read_next_token(
        &mut input_iter,
        &expected_token_kinds,
        &mut associated_tokens,
    )?;

    match token {
        Token {
            kind: TokenKind::Ident,
            span: _,
        } => parse_context.classify(token.lexeme(&input_str), &associated_tokens),

        _ => {
            return Err(give_non_expected_token_error(
                &token.kind,
                expected_token_kinds,
                &mut associated_tokens,
            ));
        }
    }
}

/// helper function to add statement to current statement list
/// giving it the associated tokens as reference
pub fn add_statement_with_token_ref(
    associated_tokens: &Vec<&Token>,
    parse_stack: &mut Vec<(Option<(Ident, Vec<Token>)>, Vec<Statement>)>,
    statement: Statement,
) -> Result<(), ParseError> {
    match parse_stack.last_mut() {
        Some((_, statements)) => {
            statements.push(statement);
        }
        None => {
            let error_msg = format!("zero parse stack can't add statement");
            return Err(ParseError {
                kind: ParseErrorKind::InternalError(error_msg),
                associated_tokens: associated_tokens
                    .iter()
                    .map(|token| (*token).clone())
                    .collect(),
            });
        }
    }
    Ok(())
}

/// helper function to add statement to current statement list
pub fn add_statement(
    associated_tokens: &Vec<Token>,
    parse_stack: &mut Vec<(Option<(Ident, Vec<Token>)>, Vec<Statement>)>,
    statement: Statement,
) -> Result<(), ParseError> {
    match parse_stack.last_mut() {
        Some((_, statements)) => {
            statements.push(statement);
        }
        None => {
            let error_msg = format!("zero parse stack can't add statement");
            return Err(ParseError {
                kind: ParseErrorKind::InternalError(error_msg),
                associated_tokens: associated_tokens.clone(),
            });
        }
    }
    Ok(())
}



/// give non expected token error
pub fn give_non_expected_token_error(
    got_token_kind: &TokenKind,
    expected_token_kinds: Vec<TokenKind>,
    associated_tokens: &mut Vec<&Token>,
) -> ParseError {
    ParseError {
        kind: ParseErrorKind::NonExpectedToken(expected_token_kinds, got_token_kind.clone()),
        associated_tokens: associated_tokens
            .iter()
            .map(|token| (*token).clone())
            .collect(),
    }
}
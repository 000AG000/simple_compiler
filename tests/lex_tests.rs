//! Tests for Lexer

mod common;
use common::*;

use simple_interpreter::lexer::{Span, Token, TokenKind, lex_ascii};

#[test]
fn lex_empty_input() {
    init();

    let tokens = lex_ascii("").unwrap();
    assert_eq!(tokens.len(), 1);
    assert!(matches!(tokens[0].kind, TokenKind::EOF));
}

#[test]
fn lex_simple_statement() {
    init();

    let tokens = lex_ascii("let x = 5;").unwrap();

    assert!(matches!(tokens[0].kind, TokenKind::Let));
    assert!(matches!(tokens[1].kind, TokenKind::Ident));
    assert!(matches!(tokens[2].kind, TokenKind::Equal));
    assert!(matches!(tokens[3].kind, TokenKind::Number(5)));
}

#[test]
fn lex_identifier_with_numbers() {
    init();

    let tokens = lex_ascii("abc123").unwrap();
    assert!(matches!(tokens[0].kind, TokenKind::Ident));
}

#[test]
fn lex_rejects_invalid_character() {
    init();

    let result = lex_ascii("let x = 5$;");
    assert!(result.is_err());
}

#[test]
fn lex_handles_whitespace_and_newlines() {
    init();

    let tokens = lex_ascii("let x = 1;\nprint x;").unwrap();

    assert!(tokens.iter().any(|t| matches!(t.kind, TokenKind::Newline)));
}

#[test]
fn lex_distinguishes_keywords_and_identifiers() {
    init();

    let tokens = lex_ascii("let letx = 1;").unwrap(); // cspell:disable-line 

    assert!(matches!(tokens[0].kind, TokenKind::Let));
    assert!(matches!(tokens[1].kind, TokenKind::Ident));
}

#[test]
fn lex_large_number() {
    init();

    let tokens = lex_ascii("let x = 123456;").unwrap();

    assert!(matches!(tokens[3].kind, TokenKind::Number(123456)));
}

#[test]
fn test_tokenization_simple_test_file() {
    init();
    let filepath = "tests/example_files/simple_test.ms";
    let input_str = std::fs::read_to_string(filepath).unwrap();
    let lex_vec = lex_ascii(&input_str).unwrap();

    let simple_test_file_vec = vec![
        Token {
            kind: TokenKind::Let,
            span: Span { start: 0, end: 3 },
        },
        Token {
            kind: TokenKind::Ident,
            span: Span { start: 4, end: 5 },
        },
        Token {
            kind: TokenKind::Equal,
            span: Span { start: 6, end: 7 },
        },
        Token {
            kind: TokenKind::Number(0),
            span: Span { start: 8, end: 9 },
        },
        Token {
            kind: TokenKind::Semicolon,
            span: Span { start: 9, end: 10 },
        },
        Token {
            kind: TokenKind::Newline,
            span: Span { start: 10, end: 11 },
        },
        Token {
            kind: TokenKind::Ident,
            span: Span { start: 11, end: 12 },
        },
        Token {
            kind: TokenKind::Equal,
            span: Span { start: 13, end: 14 },
        },
        Token {
            kind: TokenKind::Ident,
            span: Span { start: 15, end: 16 },
        },
        Token {
            kind: TokenKind::Plus,
            span: Span { start: 17, end: 18 },
        },
        Token {
            kind: TokenKind::Number(1),
            span: Span { start: 19, end: 20 },
        },
        Token {
            kind: TokenKind::Semicolon,
            span: Span { start: 20, end: 21 },
        },
        Token {
            kind: TokenKind::Newline,
            span: Span { start: 21, end: 22 },
        },
        Token {
            kind: TokenKind::Print,
            span: Span { start: 22, end: 27 },
        },
        Token {
            kind: TokenKind::Ident,
            span: Span { start: 28, end: 29 },
        },
        Token {
            kind: TokenKind::Semicolon,
            span: Span { start: 29, end: 30 },
        },
        Token {
            kind: TokenKind::EOF,
            span: Span { start: 30, end: 30 },
        },
    ];

    assert_eq!(lex_vec, simple_test_file_vec);
}

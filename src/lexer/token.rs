/// token.rs
/// Token used by lexical analysis
/// - consists of Tokenkind and Span 
/// - Span defines start and endposition of token
/// 
/// Defines also Token(kind)classes like:
/// - CONSTANT_TOKENS: tokens that have a constant length
/// - KEYWORD_TOKEN: token assotiated with a string keyword
/// 
/// Defines a mapping between TokenKind and associated String or
/// Char via SpecialToken
/// 
/// get_keyword_map gives a HashMap for all keyword tokens

use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq)]
// Token given from the lexical analysis
pub struct Token {
    pub kind: TokenKind,
    pub span: Span,
}


#[derive(Debug, Clone, Copy, PartialEq)]
/// Span for token start and end
pub struct Span {
    pub start: usize,
    pub end: usize,
}



#[derive(Debug, Clone, Copy, PartialEq)]
/// Token used for the lexing process
pub enum TokenKind {
    // Keyword like Tokens with meaning
    Let,
    Equal,
    Plus,
    Minus,
    Newline,
    Semicolon,
    Loop,
    End,
    Do,
    Print,
    // Tokens
    Ident,
    Number(isize),
}

/// Tokens that are fixed (keywords and single character tokens)
pub(crate) const CONSTANT_TOKENS: [TokenKind; 10] = [
    TokenKind::Let,
    TokenKind::Equal,
    TokenKind::Plus,
    TokenKind::Minus,
    TokenKind::Newline,
    TokenKind::Semicolon,
    TokenKind::Loop,
    TokenKind::End,
    TokenKind::Do,
    TokenKind::Print,
];

/// Non Ident and Number Tokens for iterating over it
pub const KEYWORD_TOKEN: [TokenKind; 5] = [
    TokenKind::Let,
    TokenKind::Loop,
    TokenKind::End,
    TokenKind::Do,
    TokenKind::Print,
];

#[derive(Debug, Clone, Copy)]
/// To a TokenKind assotiated keyword or char
pub(crate) enum BoundTokenKeyword {
    None,
    Char(char),
    CharWithNeededSpace(char),
    String(&'static str),
}

/// mapping of special to their strings
pub(crate) const fn get_token_keyword(token: &TokenKind) -> BoundTokenKeyword {
    match token {
        TokenKind::Let => BoundTokenKeyword::String("let"),
        TokenKind::Equal => BoundTokenKeyword::Char('='),
        TokenKind::Plus => BoundTokenKeyword::Char('+'),
        TokenKind::Minus => BoundTokenKeyword::CharWithNeededSpace('-'),
        TokenKind::Newline => BoundTokenKeyword::Char('\n'),
        TokenKind::Semicolon => BoundTokenKeyword::Char(';'),
        TokenKind::Loop => BoundTokenKeyword::String("LOOP"),
        TokenKind::End => BoundTokenKeyword::String("END"),
        TokenKind::Do => BoundTokenKeyword::String("DO"),
        TokenKind::Print => BoundTokenKeyword::String("print"),
        TokenKind::Ident => BoundTokenKeyword::None,
        TokenKind::Number(_) => BoundTokenKeyword::None,
    }
}

/// get keyword token map
pub fn get_keyword_map() -> HashMap<&'static str, TokenKind> {
    let mut keyword_map = HashMap::new();

    KEYWORD_TOKEN.iter().for_each(|x| {
        if let BoundTokenKeyword::String(token_str) = get_token_keyword(x) {
            keyword_map.insert(token_str, x.clone());
        }
    });

    keyword_map
}

use std::{fs::File, io::BufReader};

#[derive(Debug,Clone)]
/// Token used for the lexing process
enum Token{
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
    Ident(String),
    Number(isize),
}

#[derive(Debug,Clone)]
enum SpecialToken{
    None,
    Char(char),
    String(&'static str),
}

#[inline]
fn get_token_keyword(token:Token) -> SpecialToken{
    match token{
        Token::Let => SpecialToken::String("let"),
        Token::Equal => SpecialToken::Char('='),
        Token::Plus => SpecialToken::Char('+'),
        Token::Minus => SpecialToken::Char('-'),
        Token::Newline => SpecialToken::Char('\n'),
        Token::Semicolon => SpecialToken::Char(';'),
        Token::Loop => SpecialToken::String("LOOP"),
        Token::End => SpecialToken::String("END"),
        Token::Do => SpecialToken::String("DO"),
        Token::Print => SpecialToken::String("print"),
        Token::Ident(_) => SpecialToken::None,
        Token::Number(_) => SpecialToken::None,
    }
}


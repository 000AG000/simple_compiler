use std::{collections::HashMap, fs::File, io::Read};

#[derive(Debug, Clone, Copy, PartialEq)]
/// Span for token start and end
pub struct Span {
    pub start: usize,
    pub end: usize,
}

#[derive(Debug, Clone, PartialEq)]
// Token given from the lexical analysis
pub struct Token {
    kind: TokenKind,
    span: Span,
}

#[derive(Debug, Clone, PartialEq)]
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
const CONSTANT_TOKENS: [TokenKind; 10] = [
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
const KEYWORD_TOKEN: [TokenKind; 5] = [
    TokenKind::Let,
    TokenKind::Loop,
    TokenKind::End,
    TokenKind::Do,
    TokenKind::Print,
];

#[derive(Debug, Clone, Copy)]
enum SpecialToken {
    None,
    Char(char),
    CharWithNeededSpace(char),
    String(&'static str),
}

/// get keyword token map
fn get_keyword_map() -> HashMap<&'static str, TokenKind> {
    let mut keyword_map = HashMap::new();

    KEYWORD_TOKEN.iter().for_each(|x| {
        if let SpecialToken::String(token_str) = get_token_keyword(x) {
            keyword_map.insert(token_str, x.clone());
        }
    });

    keyword_map
}

/// mapping of special to their strings
const fn get_token_keyword(token: &TokenKind) -> SpecialToken {
    match token {
        TokenKind::Let => SpecialToken::String("let"),
        TokenKind::Equal => SpecialToken::Char('='),
        TokenKind::Plus => SpecialToken::Char('+'),
        TokenKind::Minus => SpecialToken::CharWithNeededSpace('-'),
        TokenKind::Newline => SpecialToken::Char('\n'),
        TokenKind::Semicolon => SpecialToken::Char(';'),
        TokenKind::Loop => SpecialToken::String("LOOP"),
        TokenKind::End => SpecialToken::String("END"),
        TokenKind::Do => SpecialToken::String("DO"),
        TokenKind::Print => SpecialToken::String("print"),
        TokenKind::Ident => SpecialToken::None,
        TokenKind::Number(_) => SpecialToken::None,
    }
}

/// get in an array special chars
const fn get_char_mask() -> [Option<TokenKind>; u8::MAX as usize] {
    let mut lex_table = [const { None }; u8::MAX as usize];
    let mut i = 0;

    // no for loop allowed because of const

    while i < CONSTANT_TOKENS.len() {
        match get_token_keyword(&CONSTANT_TOKENS[i]) {
            SpecialToken::Char(c) => {
                lex_table[c as usize] = Some(match &CONSTANT_TOKENS[i] {
                    TokenKind::Let => TokenKind::Let,
                    TokenKind::Equal => TokenKind::Equal,
                    TokenKind::Plus => TokenKind::Plus,
                    TokenKind::Minus => TokenKind::Minus,
                    TokenKind::Newline => TokenKind::Newline,
                    TokenKind::Semicolon => TokenKind::Semicolon,
                    TokenKind::Loop => TokenKind::Loop,
                    TokenKind::End => TokenKind::End,
                    TokenKind::Do => TokenKind::Do,
                    TokenKind::Print => TokenKind::Print,
                    TokenKind::Ident => TokenKind::Ident,
                    TokenKind::Number(x) => TokenKind::Number(*x),
                });
            }
            _ => {}
        }
        i += 1;
    }

    lex_table
}

/// get in an array special chars needing a space after it to be valid
const fn get_char_needing_space_mask() -> [Option<TokenKind>; u8::MAX as usize] {
    let mut lex_table = [const { None }; u8::MAX as usize];
    let mut i = 0;

    // no for loop allowed because of const

    while i < CONSTANT_TOKENS.len() {
        match get_token_keyword(&CONSTANT_TOKENS[i]) {
            SpecialToken::CharWithNeededSpace(c) => {
                lex_table[c as usize] = Some(match &CONSTANT_TOKENS[i] {
                    TokenKind::Let => TokenKind::Let,
                    TokenKind::Equal => TokenKind::Equal,
                    TokenKind::Plus => TokenKind::Plus,
                    TokenKind::Minus => TokenKind::Minus,
                    TokenKind::Newline => TokenKind::Newline,
                    TokenKind::Semicolon => TokenKind::Semicolon,
                    TokenKind::Loop => TokenKind::Loop,
                    TokenKind::End => TokenKind::End,
                    TokenKind::Do => TokenKind::Do,
                    TokenKind::Print => TokenKind::Print,
                    TokenKind::Ident => TokenKind::Ident,
                    TokenKind::Number(x) => TokenKind::Number(*x),
                });
            }
            _ => {}
        }
        i += 1;
    }

    lex_table
}

#[derive(Debug, Clone)]
enum LexTableEntries {
    Split,
    Token(TokenKind),
    SpaceNeedingToken(TokenKind),
    Alphabetic,
    Numeric,
    Undefined,
}

/// get table for checking possible token type from start in constant time
fn get_lex_table() -> [LexTableEntries; u8::MAX as usize] {
    let mut lex_table = [const { LexTableEntries::Undefined }; u8::MAX as usize];

    for i in 0..u8::MAX {
        match char::from_u32(i as u32) {
            None => (),
            Some(char) => match (
                char,
                get_char_mask()[i as usize].clone(),
                get_char_needing_space_mask()[i as usize].clone(),
            ) {
                ('a'..'z', ..) => lex_table[i as usize] = LexTableEntries::Alphabetic,
                ('A'..'Z', ..) => lex_table[i as usize] = LexTableEntries::Alphabetic,
                ('0'..'9', ..) => lex_table[i as usize] = LexTableEntries::Numeric,
                (' ', ..) => lex_table[i as usize] = LexTableEntries::Split,
                (_, Some(token), _) => lex_table[i as usize] = LexTableEntries::Token(token),
                (_, _, Some(token)) => {
                    lex_table[i as usize] = LexTableEntries::SpaceNeedingToken(token)
                }
                _ => (),
            },
        }
    }

    lex_table
}

#[derive(Debug, Clone)]
/// Errors that can occur during the lexanizer process
pub enum LexanizerError {
    UnkownCharacter(char, usize),
    UnexpectedCharacter(char, usize),
    /// Conversion error expected first type (first string) and god last string
    ConversionError(String, String),
}

/// lexanizer function transfor file into token vector
///
/// when file does not follow its function you get an LexanizerError
///
/// example usage:
/// ```
/// let file = File::open("filename").unwrap();
/// let token_vec = lexanize(file)?;
/// ```
///
pub fn lexanize(file: &mut File) -> Result<Vec<TokenKind>, LexanizerError> {
    let mut lexed_tokens = Vec::with_capacity(1000);
    let lex_table = get_lex_table();
    let mut buffer = [0; 3000];
    let keyword_map = get_keyword_map();

    #[derive(Debug)]
    enum LexanizerState {
        Normal,
        SpaceNeeding,
        Number(String),
        Indent(String),
    }

    fn handle_string_end(
        state: LexanizerState,
        token_vec: &mut Vec<TokenKind>,
        keyword_map: &HashMap<&str, TokenKind>,
    ) -> Result<(), LexanizerError> {
        match state {
            LexanizerState::Number(number_str) => {
                token_vec.push(TokenKind::Number(match number_str.parse::<isize>() {
                    Ok(number) => number,
                    Err(_) => {
                        return Err(LexanizerError::ConversionError(
                            "number".to_string(),
                            number_str.to_string(),
                        ));
                    }
                }))
            }
            LexanizerState::Indent(indent) => match keyword_map.get(&indent as &str) {
                Some(token) => token_vec.push(token.clone()),
                None => token_vec.push(TokenKind::Ident),
            },
            _ => {
                return Err(LexanizerError::ConversionError(
                    "Alphabetic|Number".to_string(),
                    format!("{:?}", state),
                ));
            }
        }
        Ok(())
    }
    let mut lexanizer_state = LexanizerState::Normal;
    let mut string_now = String::new();
    let mut position = 0;
    // iterate over file buffer
    while let Ok(n_read) = file.read(&mut buffer) {
        if n_read == 0 {
            // empty file
            break;
        }

        for place in 0..n_read {
            let next_elem = buffer[place];

            let lex_entry = &lex_table[next_elem as usize];

            lexanizer_state = match (lexanizer_state, lex_entry) {
                (LexanizerState::Normal, LexTableEntries::Split) => LexanizerState::Normal,
                (LexanizerState::Normal, LexTableEntries::Token(token)) => {
                    lexed_tokens.push(token.clone());
                    LexanizerState::Normal
                }
                (LexanizerState::Normal, LexTableEntries::SpaceNeedingToken(token)) => {
                    lexed_tokens.push(token.clone());
                    LexanizerState::SpaceNeeding
                }
                (LexanizerState::Normal, LexTableEntries::Alphabetic) => {
                    let mut string = String::new();
                    string.push(next_elem as char);
                    LexanizerState::Indent(string)
                }
                (LexanizerState::Indent(mut string), LexTableEntries::Alphabetic) => {
                    string.push(next_elem as char);
                    LexanizerState::Indent(string)
                }
                (_, LexTableEntries::Alphabetic) => {
                    return Err(LexanizerError::UnexpectedCharacter(
                        next_elem as char,
                        position,
                    ));
                }
                (LexanizerState::Normal, LexTableEntries::Numeric) => {
                    let mut string = String::new();
                    string.push(next_elem as char);
                    LexanizerState::Number(string)
                }
                (LexanizerState::Indent(mut string), LexTableEntries::Numeric) => {
                    string.push(next_elem as char);
                    LexanizerState::Number(string)
                }
                (_, LexTableEntries::Numeric) => {
                    return Err(LexanizerError::UnexpectedCharacter(
                        next_elem as char,
                        position,
                    ));
                }
                (_, LexTableEntries::Undefined) => {
                    return Err(LexanizerError::UnkownCharacter(next_elem as char, position));
                }
                (LexanizerState::SpaceNeeding, LexTableEntries::Split) => LexanizerState::Normal,
                (LexanizerState::SpaceNeeding, _) => {
                    return Err(LexanizerError::UnexpectedCharacter(
                        next_elem as char,
                        position,
                    ));
                }
                (LexanizerState::Number(in_string), LexTableEntries::Split) => {
                    handle_string_end(
                        LexanizerState::Number(in_string),
                        &mut lexed_tokens,
                        &keyword_map,
                    )?;
                    LexanizerState::Normal
                }
                (LexanizerState::Indent(in_string), LexTableEntries::Split) => {
                    handle_string_end(
                        LexanizerState::Indent(in_string),
                        &mut lexed_tokens,
                        &keyword_map,
                    )?;
                    LexanizerState::Normal
                }
                (LexanizerState::Number(string), LexTableEntries::Token(token_kind)) => {
                    handle_string_end(
                        LexanizerState::Number(string),
                        &mut lexed_tokens,
                        &keyword_map,
                    )?;
                    lexed_tokens.push(token_kind.clone());
                    LexanizerState::Normal
                }
                (
                    LexanizerState::Number(string),
                    LexTableEntries::SpaceNeedingToken(token_kind),
                ) => {
                    handle_string_end(
                        LexanizerState::Number(string),
                        &mut lexed_tokens,
                        &keyword_map,
                    )?;
                    lexed_tokens.push(token_kind.clone());
                    LexanizerState::SpaceNeeding
                }
                (LexanizerState::Indent(string), LexTableEntries::Token(token_kind)) => {
                    handle_string_end(
                        LexanizerState::Indent(string),
                        &mut lexed_tokens,
                        &keyword_map,
                    )?;
                    lexed_tokens.push(token_kind.clone());
                    LexanizerState::Normal
                }
                (
                    LexanizerState::Indent(string),
                    LexTableEntries::SpaceNeedingToken(token_kind),
                ) => {
                    handle_string_end(
                        LexanizerState::Number(string),
                        &mut lexed_tokens,
                        &keyword_map,
                    )?;
                    lexed_tokens.push(token_kind.clone());
                    LexanizerState::SpaceNeeding
                }
            };
            position += 1;
        }
    }

    // handle end state
    handle_string_end(lexanizer_state, &mut lexed_tokens, &keyword_map);
    return Ok(lexed_tokens);
}

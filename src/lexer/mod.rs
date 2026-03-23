use std::{
    collections::HashMap,
    fs::File,
    io::{Read},
};


#[derive(Debug, Clone, Copy)]
/// Tokens used for compile time lexing table
pub enum CompileTimeToken {
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
    Ident(&'static str),
    Number(isize),
}

#[inline]
/// mapping function for compile time token to tokens
fn map_compiletime_token2token(comp_token: CompileTimeToken) -> Token {
    match comp_token {
        CompileTimeToken::Let => Token::Let,
        CompileTimeToken::Equal => Token::Equal,
        CompileTimeToken::Plus => Token::Plus,
        CompileTimeToken::Minus => Token::Minus,
        CompileTimeToken::Newline => Token::Newline,
        CompileTimeToken::Semicolon => Token::Semicolon,
        CompileTimeToken::Loop => Token::Loop,
        CompileTimeToken::End => Token::End,
        CompileTimeToken::Do => Token::Do,
        CompileTimeToken::Print => Token::Print,
        CompileTimeToken::Ident(ident_str) => Token::Ident(ident_str.to_string()),
        CompileTimeToken::Number(num) => Token::Number(num),
    }
}

#[derive(Debug, Clone,PartialEq)]
/// Token used for the lexing process
pub enum Token {
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

/// Tokens that are fixed (keywords and single character tokens) 
const CONSTANT_TOKENS: [CompileTimeToken; 10] = [
    CompileTimeToken::Let,
    CompileTimeToken::Equal,
    CompileTimeToken::Plus,
    CompileTimeToken::Minus,
    CompileTimeToken::Newline,
    CompileTimeToken::Semicolon,
    CompileTimeToken::Loop,
    CompileTimeToken::End,
    CompileTimeToken::Do,
    CompileTimeToken::Print,
];

/// Non Ident and Number Tokens for iterating over it
const KEYWORD_TOKEN: [CompileTimeToken; 5] = [
    CompileTimeToken::Let,
    CompileTimeToken::Loop,
    CompileTimeToken::End,
    CompileTimeToken::Do,
    CompileTimeToken::Print,
];

#[derive(Debug, Clone, Copy)]
enum SpecialToken {
    None,
    Char(char),
    CharWithNeededSpace(char),
    String(&'static str),
}

/// get keyword token map
fn get_keyword_map() -> HashMap<&'static str, Token> {
    let mut keyword_map = HashMap::new();

    KEYWORD_TOKEN.iter().for_each(|x| {
        if let SpecialToken::String(token_str) = get_token_keyword(x) {
            keyword_map.insert(
                token_str,
                map_compiletime_token2token(x.clone()),
            );
        }
    });

    keyword_map
}

/// mapping of special to their strings
const fn get_token_keyword(token: &CompileTimeToken) -> SpecialToken {
    match token {
        CompileTimeToken::Let => SpecialToken::String("let"),
        CompileTimeToken::Equal => SpecialToken::Char('='),
        CompileTimeToken::Plus => SpecialToken::Char('+'),
        CompileTimeToken::Minus => SpecialToken::CharWithNeededSpace('-'),
        CompileTimeToken::Newline => SpecialToken::Char('\n'),
        CompileTimeToken::Semicolon => SpecialToken::Char(';'),
        CompileTimeToken::Loop => SpecialToken::String("LOOP"),
        CompileTimeToken::End => SpecialToken::String("END"),
        CompileTimeToken::Do => SpecialToken::String("DO"),
        CompileTimeToken::Print => SpecialToken::String("print"),
        CompileTimeToken::Ident(_) => SpecialToken::None,
        CompileTimeToken::Number(_) => SpecialToken::None,
    }
}

/// get in an array special chars
const fn get_char_mask() -> [Option<CompileTimeToken>; u8::MAX as usize] {
    let mut lex_table = [None; u8::MAX as usize];
    let mut i = 0;

    // no for loop allowed because of const

    while i < CONSTANT_TOKENS.len() {
        match get_token_keyword(&CONSTANT_TOKENS[i]) {
            SpecialToken::Char(c) => {
                lex_table[c as usize] = Some(CONSTANT_TOKENS[i]);
            }
            _ => {}
        }
        i += 1;
    }

    lex_table
}

/// get in an array special chars needing a space after it to be valid
const fn get_char_needing_space_mask() -> [Option<CompileTimeToken>; u8::MAX as usize] {
    let mut lex_table = [None; u8::MAX as usize];
    let mut i = 0;

    // no for loop allowed because of const

    while i < CONSTANT_TOKENS.len() {
        match get_token_keyword(&CONSTANT_TOKENS[i]) {
            SpecialToken::CharWithNeededSpace(c) => {
                lex_table[c as usize] = Some(CONSTANT_TOKENS[i]);
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
    Token(Token),
    SpaceNeedingToken(Token),
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
                get_char_mask()[i as usize],
                get_char_needing_space_mask()[i as usize],
            ) {
                ('a'..'z', ..) => lex_table[i as usize] = LexTableEntries::Alphabetic,
                ('A'..'Z', ..) => lex_table[i as usize] = LexTableEntries::Alphabetic,
                ('0'..'9', ..) => lex_table[i as usize] = LexTableEntries::Numeric,
                (' ', ..) => lex_table[i as usize] = LexTableEntries::Split,
                (_, Some(token), _) => lex_table[i as usize] = LexTableEntries::Token(map_compiletime_token2token(token)),
                (_, _, Some(token)) => {
                    lex_table[i as usize] = LexTableEntries::SpaceNeedingToken(map_compiletime_token2token(token))
                }
                _ => (),
            },
        }

    }

    lex_table
}

#[derive(Debug,Clone)]
/// Errors that can occur during the lexanizer process
pub enum LexanizerError {
    UnkownCharacter(char),
    UnexpectedCharacter(String, char),
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
pub fn lexanize(file: &mut File) -> Result<Vec<Token>, LexanizerError> {
    let mut lexed_tokens = Vec::with_capacity(1000);
    let lex_table = get_lex_table();
    let mut buffer = [0; 3000];
    let keyword_map = get_keyword_map();

    enum LexanizerState {
        Empty,
        Number,
        String,
        NextSpace,
    }

    fn handle_string_end(
        state: &LexanizerState,
        string: &str,
        token_vec: &mut Vec<Token>,
        keyword_map: &HashMap<&str, Token>,
    ) -> Result<(), LexanizerError> {
        match state {
            LexanizerState::Number => {
                token_vec.push(Token::Number(match string.parse::<isize>() {
                    Ok(number) => number,
                    Err(_) => {
                        return Err(LexanizerError::ConversionError(
                            "number".to_string(),
                            string.to_string(),
                        ));
                    }
                }))
            }
            LexanizerState::String => {
                match keyword_map.get(string){
                    Some(token) => token_vec.push(token.clone()),
                    None => 
                token_vec.push(Token::Ident(string.to_string())),
                }
            }
            _ => (),
        
            }
        Ok(())
    }
    let mut lexanizer_state = LexanizerState::Empty;
    let mut string_now = String::new();

    // iterate over file buffer
    while let Ok(n_read) = file.read(&mut buffer) {
        if n_read == 0 {
            // empty file
            break;
        }

        for place in 0..n_read {
            let next_elem = buffer[place];

            let lex_entry = &lex_table[next_elem as usize];
            match lex_entry {
                LexTableEntries::Split => {
                    handle_string_end(&lexanizer_state, &string_now, &mut lexed_tokens,&keyword_map)?;
                    lexanizer_state = LexanizerState::Empty
                }
                LexTableEntries::Token(token) => {
                    handle_string_end(&lexanizer_state, &string_now, &mut lexed_tokens,&keyword_map)?;
                    lexanizer_state = LexanizerState::Empty;
                    lexed_tokens.push(token.clone());
                }
                LexTableEntries::SpaceNeedingToken(token) => {
                    handle_string_end(&lexanizer_state, &string_now, &mut lexed_tokens,&keyword_map)?;
                    lexanizer_state = LexanizerState::NextSpace;
                    lexed_tokens.push(token.clone());
                }
                LexTableEntries::Alphabetic => match lexanizer_state {
                    LexanizerState::Empty => {
                        lexanizer_state = LexanizerState::String;
                        string_now = String::new();
                        string_now.push(next_elem as char);
                    }
                    LexanizerState::String => {
                        string_now.push(next_elem as char);
                    }
                    _ => {
                        return Err(LexanizerError::UnexpectedCharacter(
                            string_now,
                            next_elem as char,
                        ));
                    }
                },
                LexTableEntries::Numeric => match lexanizer_state {
                    LexanizerState::Empty => {
                        lexanizer_state = LexanizerState::Number;
                        string_now = String::new();
                        string_now.push(next_elem as char);
                    }
                    LexanizerState::Number => {
                        string_now.push(next_elem as char)
                    }
                    _ => {
                        return Err(LexanizerError::UnexpectedCharacter(
                            string_now,
                            next_elem as char,
                        ));
                    }
                },

                LexTableEntries::Undefined => {
                    return Err(LexanizerError::UnkownCharacter(
                        next_elem as char,
                    ));
                }
            }
        }
    }

    // handle end state
    if let LexanizerState::Number|LexanizerState::String = lexanizer_state{
        handle_string_end(&lexanizer_state, &string_now, &mut lexed_tokens,&keyword_map)?;
    }
    return Ok(lexed_tokens);
}

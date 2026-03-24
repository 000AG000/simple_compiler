use std::fmt;
use std::{collections::HashMap, error::Error, fmt::Display, fs::File, io::Read};

#[derive(Debug, Clone, Copy, PartialEq)]
/// Span for token start and end
pub struct Span {
    pub start: usize,
    pub end: usize,
}

#[derive(Debug, Clone, PartialEq)]
// Token given from the lexical analysis
pub struct Token {
    pub kind: TokenKind,
    pub span: Span,
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

#[derive(Debug, Clone, PartialEq)]
pub enum LexTableEntry {
    Split,
    Token(TokenKind),
    SpaceNeedingToken(TokenKind),
    Alphabetic,
    Numeric,
    Undefined,
}

#[derive(Debug, Clone, PartialEq)]
/// LexTable is a O(1) lookup table for what kind of byte it is
///
/// The LexTable is build when crating the new instance and accessed via classify
pub struct LexTable {
    entries: [LexTableEntry; 256],
}

impl LexTable {
    /// Creating a new LexTable
    pub fn new() -> Self {
        let mut entries = [const { LexTableEntry::Undefined }; 256];

        for i in 0..u8::MAX {
            match char::from_u32(i as u32) {
                None => (),
                Some(char) => match (
                    char,
                    LexTable::get_char_mask()[i as usize].clone(),
                    LexTable::get_char_needing_space_mask()[i as usize].clone(),
                ) {
                    ('a'..'z', ..) => entries[i as usize] = LexTableEntry::Alphabetic,
                    ('A'..'Z', ..) => entries[i as usize] = LexTableEntry::Alphabetic,
                    ('0'..'9', ..) => entries[i as usize] = LexTableEntry::Numeric,
                    (' ', ..) => entries[i as usize] = LexTableEntry::Split,
                    (_, Some(token), _) => entries[i as usize] = LexTableEntry::Token(token),
                    (_, _, Some(token)) => {
                        entries[i as usize] = LexTableEntry::SpaceNeedingToken(token)
                    }
                    _ => (),
                },
            }
        }

        LexTable { entries }
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

    /// lookup byte in LexTable
    pub fn classify(&self, byte: u8) -> &LexTableEntry {
        &self.entries[byte as usize]
    }
}

#[derive(Debug, Clone)]
/// Errors that can occur during the lexanizer process
pub enum LexErrorKind {
    UnkownCharacter(char),
    UnexpectedCharacter(char),
    /// Conversion error expected first type (first string) and god last string
    ConversionError(String, String),
}

#[derive(Debug, Clone)]
/// LexError struct with kind of error and span that it refers to
pub struct LexError {
    pub kind: LexErrorKind,
    pub span: Span,
}

impl Display for LexError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Lexical Analysis error on position {} to {}: {:?}",self.span.start,self.span.end,self.kind)
    }
}

impl Error for LexError {}

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
pub fn lexanize(file: &mut File) -> Result<Vec<Token>, LexError> {
    let mut lexed_tokens: Vec<Token> = Vec::with_capacity(1000);
    let lex_table = LexTable::new();
    let mut buffer = [0; 3000];
    let keyword_map = get_keyword_map();

    #[derive(Debug)]
    enum LexanizerState {
        Normal,
        SpaceNeeding,
        /// Number(start position, string)
        Number(usize, String),
        /// Indent(start position, string)
        Indent(usize, String),
    }

    fn handle_string_end(
        state: LexanizerState,
        token_vec: &mut Vec<Token>,
        keyword_map: &HashMap<&str, TokenKind>,
        end_position: usize,
    ) -> Result<(), LexError> {
        match state {
            LexanizerState::Number(start_position, number_str) => token_vec.push(Token {
                kind: TokenKind::Number(match number_str.parse::<isize>() {
                    Ok(number) => number,
                    Err(_) => {
                        return Err(LexError{kind:LexErrorKind::ConversionError(
                            "number".to_string(),
                            number_str.to_string(),
                        ),span:Span { start: start_position, end: end_position }});
                    }
                }),
                span: Span {
                    start: start_position,
                    end: end_position,
                },
            }),
            LexanizerState::Indent(start_position, indent) => {
                match keyword_map.get(&indent as &str) {
                    Some(token) => token_vec.push(Token {
                        kind: token.clone(),
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
                }
            }
            _ => (),
        }
        Ok(())
    }
    let mut lexanizer_state = LexanizerState::Normal;
    let mut position = 0;
    // iterate over file buffer
    while let Ok(n_read) = file.read(&mut buffer) {
        if n_read == 0 {
            // empty file
            break;
        }

        for place in 0..n_read {
            let next_elem = buffer[place];

            let lex_entry = lex_table.classify(next_elem);

            lexanizer_state = match (lexanizer_state, lex_entry) {
                (LexanizerState::Normal, LexTableEntry::Split) => LexanizerState::Normal,
                (LexanizerState::Normal, LexTableEntry::Token(token)) => {
                    lexed_tokens.push(Token {
                        kind: token.clone(),
                        span: Span {
                            start: position,
                            end: position + 1,
                        },
                    });
                    LexanizerState::Normal
                }
                (LexanizerState::Normal, LexTableEntry::SpaceNeedingToken(token)) => {
                    lexed_tokens.push(Token {
                        kind: token.clone(),
                        span: Span {
                            start: position,
                            end: position + 1,
                        },
                    });
                    LexanizerState::SpaceNeeding
                }
                (LexanizerState::Normal, LexTableEntry::Alphabetic) => {
                    let mut string = String::new();
                    string.push(next_elem as char);
                    LexanizerState::Indent(position, string)
                }
                (LexanizerState::Indent(start_postion, mut string), LexTableEntry::Alphabetic) => {
                    string.push(next_elem as char);
                    LexanizerState::Indent(start_postion, string)
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
                (LexanizerState::Normal, LexTableEntry::Numeric) => {
                    let mut string = String::new();
                    string.push(next_elem as char);
                    LexanizerState::Number(position, string)
                }
                (LexanizerState::Indent(start_position, mut string), LexTableEntry::Numeric) => {
                    string.push(next_elem as char);
                    LexanizerState::Number(start_position, string)
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
                        kind: LexErrorKind::UnkownCharacter(next_elem as char),
                        span: Span {
                            start: position,
                            end: position + 1,
                        },
                    });
                }
                (LexanizerState::SpaceNeeding, LexTableEntry::Split) => LexanizerState::Normal,
                (LexanizerState::SpaceNeeding, _) => {
                    return Err(LexError {
                        kind: LexErrorKind::UnexpectedCharacter(next_elem as char),
                        span: Span {
                            start: position,
                            end: position + 1,
                        },
                    });
                }
                (LexanizerState::Number(start_position, in_string), LexTableEntry::Split) => {
                    handle_string_end(
                        LexanizerState::Number(start_position, in_string),
                        &mut lexed_tokens,
                        &keyword_map,
                        position,
                    )?;
                    LexanizerState::Normal
                }
                (LexanizerState::Indent(start_position, in_string), LexTableEntry::Split) => {
                    handle_string_end(
                        LexanizerState::Indent(start_position, in_string),
                        &mut lexed_tokens,
                        &keyword_map,
                        position,
                    )?;
                    LexanizerState::Normal
                }
                (
                    LexanizerState::Number(start_position, string),
                    LexTableEntry::Token(token_kind),
                ) => {
                    handle_string_end(
                        LexanizerState::Number(start_position, string),
                        &mut lexed_tokens,
                        &keyword_map,
                        position,
                    )?;
                    lexed_tokens.push(Token {
                        kind: token_kind.clone(),
                        span: Span {
                            start: position,
                            end: position + 1,
                        },
                    });
                    LexanizerState::Normal
                }
                (
                    LexanizerState::Number(start_position, string),
                    LexTableEntry::SpaceNeedingToken(token_kind),
                ) => {
                    handle_string_end(
                        LexanizerState::Number(start_position, string),
                        &mut lexed_tokens,
                        &keyword_map,
                        position,
                    )?;
                    lexed_tokens.push(Token {
                        kind: token_kind.clone(),
                        span: Span {
                            start: position,
                            end: position + 1,
                        },
                    });
                    LexanizerState::SpaceNeeding
                }
                (
                    LexanizerState::Indent(start_position, string),
                    LexTableEntry::Token(token_kind),
                ) => {
                    handle_string_end(
                        LexanizerState::Indent(start_position, string),
                        &mut lexed_tokens,
                        &keyword_map,
                        position,
                    )?;
                    lexed_tokens.push(Token {
                        kind: token_kind.clone(),
                        span: Span {
                            start: position,
                            end: position + 1,
                        },
                    });
                    LexanizerState::Normal
                }
                (
                    LexanizerState::Indent(start_position, string),
                    LexTableEntry::SpaceNeedingToken(token_kind),
                ) => {
                    handle_string_end(
                        LexanizerState::Number(start_position, string),
                        &mut lexed_tokens,
                        &keyword_map,
                        start_position,
                    )?;
                    lexed_tokens.push(Token {
                        kind: token_kind.clone(),
                        span: Span {
                            start: position,
                            end: position + 1,
                        },
                    });
                    LexanizerState::SpaceNeeding
                }
            };
            position += 1;
        }
    }

    // handle end state
    handle_string_end(lexanizer_state, &mut lexed_tokens, &keyword_map, position)?;
    return Ok(lexed_tokens);
}

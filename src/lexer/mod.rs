use std::{fs::File, io::BufReader};

#[derive(Debug,Clone,Copy)]
/// Token used for the lexing process
pub enum Token{
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

// Non Ident and Number Tokens for iterating over it
const NON_FALLBACK_TOKENS: [Token; 10] = [
    Token::Let,
    Token::Equal,
    Token::Plus,
    Token::Minus,
    Token::Newline,
    Token::Semicolon,
    Token::Loop,
    Token::End,
    Token::Do,
    Token::Print,
];

#[derive(Debug,Clone,Copy)]
enum SpecialToken{
    None,
    Char(char),
    CharWithNeededSpace(char),
    String(&'static str),
}

const fn get_token_keyword(token:Token) -> SpecialToken{
    match token{
        Token::Let => SpecialToken::String("let"),
        Token::Equal => SpecialToken::Char('='),
        Token::Plus => SpecialToken::Char('+'),
        Token::Minus => SpecialToken::CharWithNeededSpace('-'),
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

const fn get_char_mask() -> [Option<Token>;u8::MAX as usize]{
    let mut lex_table = [None;u8::MAX as usize];
    let mut i = 0;

    // no for loop allowed because of const

    while i < NON_FALLBACK_TOKENS.len() {
        match get_token_keyword(NON_FALLBACK_TOKENS[i]) {
            SpecialToken::Char(c) => {
                lex_table[c as usize] = Some(NON_FALLBACK_TOKENS[i]);
            }
            _ => {}
        }
        i += 1;
    }
    

    lex_table
}

const fn get_char_needing_space_mask() -> [Option<Token>;u8::MAX as usize]{
    let mut lex_table = [None;u8::MAX as usize];
    let mut i = 0;

    // no for loop allowed because of const

    while i < NON_FALLBACK_TOKENS.len() {
        match get_token_keyword(NON_FALLBACK_TOKENS[i]) {
            SpecialToken::CharWithNeededSpace(c) => {
                lex_table[c as usize] = Some(NON_FALLBACK_TOKENS[i]);
            }
            _ => {}
        }
        i += 1;
    }
    

    lex_table
}

#[derive(Debug,Clone,Copy)]
enum LexTableEntries {
    Split,
    Token(Token),
    SpaceNeedingToken(Token),
    KeywordOrNumIdent,
    Undefined,
}

const fn get_lex_table() -> [LexTableEntries;u8::MAX as usize]{
    let mut lex_table = [LexTableEntries::Undefined;u8::MAX as usize];

    let mut i = 0;

    while i < u8::MAX{

        match char::from_u32(i as u32){
            None => (),
            Some(char) => match (char,get_char_mask()[i as usize],get_char_needing_space_mask()[i as usize]){
                ('a'..'z',..) => lex_table[i as usize] = LexTableEntries::KeywordOrNumIdent,
                ('A'..'Z',..) => lex_table[i as usize] = LexTableEntries::KeywordOrNumIdent,
                (_,Some(token),_) => lex_table[i as usize] = LexTableEntries::Token(token),
                (_,_,Some(token)) => lex_table[i as usize] = LexTableEntries::SpaceNeedingToken(token),
                _ => ()

            }
        }

        i += 1;
    }

    lex_table
}


pub fn lexanize(file:File) -> Vec<Token>{
    let mut lexed_tokens = Vec::with_capacity(1000);

    let mut buff_reader = BufReader::new(file);
    let mut buff_string = String::new();

    println!("float: {:?}",get_lex_table());



    return lexed_tokens;
}
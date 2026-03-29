//! lex_table.rs
//! Defines LexTable struct for O(1) checking to possible TokenKind mapping
//! - classify method for getting assotiated type
//! 


use crate::lexer::token::{TokenKind,CONSTANT_TOKENS,BoundTokenKeyword,get_token_keyword,get_token_kind_clone};

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
/// LexTable is a O(1) lookup table for what kind token is associated with the byte
///
/// The LexTable is build when crating the new instance and accessed via classify
pub struct LexTable {
    entries: [LexTableEntry; 256],
}

impl LexTable {
    /// Creating a new LexTable
    pub fn new() -> Self {
        let mut entries = [const { LexTableEntry::Undefined }; 256];

        let char_mask = Self::get_char_mask();
        let space_mask = Self::get_char_needing_space_mask();

        for i in 0..u8::MAX {
            match char::from_u32(i as u32) {
                None => (),
                Some(char) => match (
                    char,
                    char_mask[i as usize],
                    space_mask[i as usize],
                ) {
                    ('a'..='z', ..) => entries[i as usize] = LexTableEntry::Alphabetic,
                    ('A'..='Z', ..) => entries[i as usize] = LexTableEntry::Alphabetic,
                    ('0'..='9', ..) => entries[i as usize] = LexTableEntry::Numeric,
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

    /// get LexTableEmtries for char type TokenKinds
    const fn get_char_mask() -> [Option<TokenKind>; u8::MAX as usize] {
        let mut lex_table = [const { None }; u8::MAX as usize];
        let mut i = 0;

        // no for loop allowed because of const

        while i < CONSTANT_TOKENS.len() {
            if let BoundTokenKeyword::Char(c) = get_token_keyword(&CONSTANT_TOKENS[i]) {
                lex_table[c as usize] = Some(get_token_kind_clone(&CONSTANT_TOKENS[i]));
            }
            i += 1;
        }

        lex_table
    }

    /// get LexTableEmtries for char type TokenKinds that need spaceing afterwards
    const fn get_char_needing_space_mask() -> [Option<TokenKind>; u8::MAX as usize] {
        let mut lex_table = [const { None }; u8::MAX as usize];
        let mut i = 0;

        // no for loop allowed because of const

        while i < CONSTANT_TOKENS.len() {
            if let BoundTokenKeyword::CharWithNeededSpace(c) = get_token_keyword(&CONSTANT_TOKENS[i]) {
                lex_table[c as usize] = Some(get_token_kind_clone(&CONSTANT_TOKENS[i]));
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
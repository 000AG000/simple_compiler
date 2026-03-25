use std::collections::HashMap;

use super::{ParseError, ParseErrorKind};
/// Parse context used and build up by parser
/// Context at the moment only contains variable bound to names and no shadowing
use crate::lexer::{Span,Token};

/// Kind of Identificator
///
/// For now only Variable but extensible for function etc
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum IdentKind {
    Variable,
}

/// Identificator bound to name
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Ident {
    pub ident_number: usize,
    pub kind: IdentKind,
    pub span: Span,
}

#[derive(Debug, Clone, PartialEq)]
pub(crate) struct ParseContext{
    next_ident_number: usize,
    ident_mapping: HashMap<String, Ident>,
}

impl ParseContext {
    pub fn new() -> Self {
        ParseContext {
            next_ident_number: 0,
            ident_mapping: HashMap::new(),
        }
    }

    /// classify identification string gives None when no identificator bound to it
    pub fn classify(&self, ident_str: &str) -> Option<Ident> {
        return self.ident_mapping.get(ident_str).copied();
    }

    /// get the next identification number
    /// - yust adding +1 to the last identification number
    fn get_next_ident_number(&mut self) -> usize{
        let num = self.next_ident_number;
        self.next_ident_number += 1;
        num
    }

    /// inserts new identification in parse context 
    pub fn new_ident(
        &mut self,
        ident_name: &str,
        ident_kind: IdentKind,
        ident_span: Span,
        token: Token,
    ) -> Result<(), ParseError> {
        let ident_string = ident_name.to_string();

        // early return if identificator already defined
        if let Some(ident) = self.classify(ident_name) {
            return Err(ParseError { kind: ParseErrorKind::IdentificatorAlreadyUsed(ident_string, ident.span), assotiated_tokens: vec![token] });
        }

        let next_ident_number = self.get_next_ident_number();

        self.ident_mapping.insert(ident_name.to_string(), Ident { ident_number: next_ident_number, kind: ident_kind, span: ident_span });


        Ok(())
    }
}

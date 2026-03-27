use std::collections::HashMap;

use super::{ParseError, ParseErrorKind};
/// Parse context used and build up by parser
/// Context at the moment only contains variable bound to names and no shadowing
use crate::lexer::{Span};

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
pub(crate) struct ParseContext {
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
    pub fn classify(
        &self,
        ident_str: &str,
        associated_span: Span,
    ) -> Result<Ident, ParseError> {
        match self.ident_mapping.get(ident_str).copied() {
            Some(ident) => Ok(ident),
            None => {
                return Err(ParseError {
                    kind: ParseErrorKind::IdentificatorNotKnown(ident_str.to_string()),
                    span: associated_span,
                });
            }
        }
    }

    /// get the next identification number
    /// - yust adding +1 to the last identification number
    fn get_next_ident_number(&mut self) -> usize {
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
    ) -> Result<Ident, ParseError> {
        let ident_string = ident_name.to_string();

        // early return if identificator already defined
        if self.ident_mapping.contains_key(ident_name) {

            let ident = self.classify(ident_name, ident_span)?;
            return Err(ParseError {
                kind: ParseErrorKind::IdentificatorAlreadyUsed(ident_string, ident.span),
                span: ident.span,
            });
        }

        let next_ident_number = self.get_next_ident_number();
        let ident = Ident {
            ident_number: next_ident_number,
            kind: ident_kind,
            span: ident_span,
        };
        self.ident_mapping
            .insert(ident_name.to_string(), ident.clone());

        Ok(ident)
    }
}

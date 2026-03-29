/// runtime_error.rs
/// Defines the RuntimeError struct that saves the error type and the Span associated with the error
use std::{error::Error, fmt, fmt::Display};

use crate::lexer::Span;

/// number of characters to visualize ahead when showing an error
const LOOK_AHEAD:usize = 20;
/// number of characters to visualize afterwards when showing an error
const LOOK_AFTER:usize = 20;


#[derive(Debug, Clone)]
/// Errors that can occur during the tokenization process
pub enum RuntimeErrorKind {
    InternalError(String),
    VariableAlreadyDefined,
}

#[derive(Debug, Clone)]
/// Runtime struct with kind of error and span that it refers to
pub struct RuntimeError {
    pub kind: RuntimeErrorKind,
    pub span: Span,
}

impl RuntimeError{
    /// Generate error message enriched with input information
    /// Used to better locate message and use ParseError span information
    pub fn generate_error_msg(&self, input: &str) -> String {
        let str_before = &input[self.span.start.saturating_sub(LOOK_AHEAD)..self.span.start];
        let str_content = &input[self.span.start..self.span.end];
        let str_after = &input[self.span.end..if self.span.end + LOOK_AFTER < input.len() {
            self.span.end + LOOK_AFTER
        } else {
            input.len()
        }];
        format!(
            "error occurred: ...{str_before} here >>> {str_content} <<<{str_after}...\n {}",
            self
        )
    }
}

impl Display for RuntimeError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self.kind {
            RuntimeErrorKind::InternalError(error_str) => {
                write!(f, "Runtime error: {}", error_str)
            }
            RuntimeErrorKind::VariableAlreadyDefined => write!(f, "Runtime error: Variable already defined"),
        }
    }
}

impl Error for RuntimeError {}

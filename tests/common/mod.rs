#![allow(dead_code)]
use std::sync::Once;

use simple_interpreter::{interpreter::exec_with_output, lex_ascii, parse};

static INIT: Once = Once::new();

pub fn init() {
    INIT.call_once(|| {
        let _ = env_logger::builder().is_test(true).try_init();
    });
}

pub fn run_program(input: &str) -> String {
    let tokens = lex_ascii(input).unwrap();
    let program = parse(&tokens, input).unwrap();

    let mut output = String::new();
    exec_with_output(program, input, &mut output).unwrap();

    output
}

pub fn run_program_expect_error(input: &str) {
    let tokens = match lex_ascii(input) {
        Ok(t) => t,
        Err(_) => return,
    };
    let program = match parse(&tokens, input) {
        Ok(p) => p,
        Err(_) => return,
    };

    let mut output = String::new();
    let result = exec_with_output(program, input, &mut output);

    assert!(result.is_err());
}

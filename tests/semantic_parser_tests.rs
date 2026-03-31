mod common;
use common::*;

use simple_interpreter::{lex_ascii, parse};

#[test]
fn parse_simple_program() {
    init();

    let input = "let x = 1; print x;";
    let tokens = lex_ascii(input).unwrap();
    let program = parse(&tokens, input).unwrap();

    assert_eq!(program.statements.len(), 2);
}

#[test]
fn parse_assignment_expression() {
    init();

    let input = "let x = 1 + 2 + 3;";
    let tokens = lex_ascii(input).unwrap();
    let program = parse(&tokens, input).unwrap();

    assert_eq!(program.statements.len(), 1);
}

#[test]
fn parse_nested_loop_structure() {
    init();

    let input = r#"let x = 2;
        LOOP x DO
            LOOP x DO
                print x;
            END
        END"#;

    let tokens = lex_ascii(input).unwrap();
    let program = parse(&tokens, input).unwrap();

    assert_eq!(program.statements.len(), 3);
}

#[test]
fn parse_fails_on_missing_end() {
    init();

    let input = "let x = 1; LOOP x DO print x;";
    let tokens = lex_ascii(input).unwrap();

    let result = parse(&tokens, input);

    assert!(result.is_err());
}

#[test]
fn parse_fails_on_unexpected_token() {
    init();

    let input = "= x 5;";
    let tokens = lex_ascii(input).unwrap();

    let result = parse(&tokens, input);

    assert!(result.is_err());
}

#[test]
fn parse_fails_on_undefined_identifier() {
    init();

    let input = "x = 1;";
    let tokens = lex_ascii(input).unwrap();

    let result = parse(&tokens, input);

    assert!(result.is_err());
}

#[test]
fn parse_allows_empty_statements() {
    init();

    let input = ";;;";
    let tokens = lex_ascii(input).unwrap();
    let program = parse(&tokens, input).unwrap();

    assert!(!program.statements.is_empty());
}

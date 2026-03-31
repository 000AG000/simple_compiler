//! Tests for Lexer

#[cfg(test)]
mod tests {

    use simple_interpreter::{
        error::ParseErrorKind,
        lexer::{ErrorKind, Span, lex_ascii},
        sem_parser::{
            BinOp, BinOpKind, Expr, ExprKind, Ident, IdentKind, Program, Statement, StatementKind,
            parse,
        },
    };

    /// setting up test environment
    fn init() {
        // env_logger::init();
    }

    #[test]
    fn test_parsing_simple_test_file() {
        init();
        let filepath = "tests/example_files/simple_test.ms";
        let input_str = std::fs::read_to_string(filepath).unwrap();
        let lex_vec = lex_ascii(&input_str).unwrap();

        let program = match parse(&lex_vec, &input_str) {
            Ok(program) => program,
            Err(error) => {
                eprintln!("{}", error.generate_error_msg(&input_str));
                panic!("program not read in correctly");
            }
        };

        let ident_x = Ident {
            ident_number: 0,
            kind: IdentKind::Variable,
            span: Span { start: 0, end: 5 },
        };
        let right_parse = Program {
            statements: vec![
                Statement::new(
                    StatementKind::Let {
                        name: ident_x,
                        value: Some(Expr::new(ExprKind::Number(0), Span { start: 8, end: 9 })),
                    },
                    Span { start: 0, end: 10 },
                ),
                Statement::new(StatementKind::Empty, Span { start: 10, end: 11 }),
                Statement::new(
                    StatementKind::Assign {
                        name: ident_x,
                        value: Expr::new(
                            ExprKind::Binary {
                                left: Box::new(Expr::new(
                                    ExprKind::Ident(ident_x),
                                    Span { start: 15, end: 16 },
                                )),
                                op: BinOp::new(BinOpKind::Add, Span { start: 17, end: 18 }),
                                right: Box::new(Expr::new(
                                    ExprKind::Number(1),
                                    Span { start: 19, end: 20 },
                                )),
                            },
                            Span { start: 15, end: 20 },
                        ),
                    },
                    Span { start: 11, end: 21 },
                ),
                Statement::new(StatementKind::Empty, Span { start: 21, end: 22 }),
                Statement::new(
                    StatementKind::Print { name: ident_x },
                    Span { start: 22, end: 30 },
                ),
            ],
        };

        assert_eq!(program, right_parse);
    }

    #[test]
    fn test_parsing_loop_test_file() {
        init();
        let filepath = "tests/example_files/loop_test.ms";
        let input_str = std::fs::read_to_string(filepath).unwrap();
        let lex_vec = lex_ascii(&input_str).unwrap();

        let program = match parse(&lex_vec, &input_str) {
            Ok(program) => program,
            Err(error) => {
                println!("{}", error.generate_error_msg(&input_str));
                panic!("program not read in correctly");
            }
        };

        println!("{:#?}", program);
    }

    #[test]
    fn test_parsing_only_assign() {
        init();
        let input_str = "x = 1;print x;";
        let lex_vec = lex_ascii(input_str).unwrap();

        match parse(&lex_vec, input_str) {
            Ok(_) => panic!(),
            Err(error) => {
                assert_eq!(
                    error.kind,
                    ErrorKind::Parse(ParseErrorKind::IdentifierNotKnown("x".to_string()))
                );
            }
        };
    }
}

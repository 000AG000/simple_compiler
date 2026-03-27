/// Tests for Lexer

#[cfg(test)]
mod tests {

    use simple_compiler::{lexer::{Span, lex}, parser::{BinOp, Expr, Ident, IdentKind, Program, Statement, parse}};

    #[test]
    fn test_parsing_simple_test_file() {
        let filepath = "tests/example_files/simple_test.ms";
        let input_str = std::fs::read_to_string(filepath).unwrap();
        let lex_vec = lex(&input_str).unwrap();

        let program = match parse(&lex_vec, &input_str){
            Ok(program) => program,
            Err(error) => {
                println!("{}",error.generate_error_msg(&input_str));
                panic!("program not read in correctly");
            },
        };

        let ident_x = Ident { ident_number: 0, kind: IdentKind::Variable, span: Span{start:0,end:5}};
        let right_parse = Program{ statements:vec![
            Statement::Let { name:  ident_x.clone(), value: Some(Expr::Number(0)) },
            Statement::Empty,
            Statement::Assign { name: ident_x.clone(), value: 
                Expr::Binary { left: Box::new(Expr::Ident(ident_x.clone())), op: BinOp::Add, right: Box::new(Expr::Number(1)) } },
            Statement::Empty,
            Statement::Print { name: ident_x.clone() },
        ]};

        assert_eq!(program,right_parse);
    }

   #[test]
    fn test_parsing_loop_test_file() {
        let filepath = "tests/example_files/loop_test.ms";
        let input_str = std::fs::read_to_string(filepath).unwrap();
        let lex_vec = lex(&input_str).unwrap();

        let program = match parse(&lex_vec, &input_str){
            Ok(program) => program,
            Err(error) => {
                println!("{}",error.generate_error_msg(&input_str));
                panic!("program not read in correctly");
            },
        };

        println!("{:#?}",program);

    }
}

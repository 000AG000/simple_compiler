
#[cfg(test)]
mod tests {
    use std::fs::File;

    use simple_compiler::lexer::{Token, lexanize};

    #[test]
    fn test_lexanization_simple_test_file() {
        let mut file = File::open("tests/example_files/simple_test.ms").unwrap();
        let lex_vec = lexanize(&mut file).unwrap();

        let simple_test_file_vec = vec![
            Token::Let, Token::Ident(String::from("x")),Token::Equal, Token::Number(0),Token::Semicolon,Token::Newline,
            Token::Ident(String::from("x")),Token::Equal,Token::Ident(String::from("x")),Token::Plus,Token::Number(1),Token::Semicolon,Token::Newline,
            Token::Print,Token::Ident(String::from("x")),Token::Semicolon
        ] ;

        assert_eq!(lex_vec,simple_test_file_vec);
    }
}
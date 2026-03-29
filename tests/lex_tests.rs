/// Tests for Lexer

#[cfg(test)]
mod tests {
    use simple_interpreter::lexer::{Span, Token, TokenKind, lex};

    #[test]
    fn test_invalid_char() {
        let input = "let x = 5$;";
        assert!(lex(input).is_err());
    }

    #[test]
    fn test_number_ident_mix() {
        let input = "abc123";
        let tokens = lex(input).unwrap();
        assert_eq!(tokens[0],Token{kind:TokenKind::Ident,span:Span { start: 0, end: 6 }})
    }

    #[test]
    fn test_empty() {
        let input = "";
        let tokens = lex(input).unwrap();
        let eof_tokens = vec![Token{kind:TokenKind::EOF,span:Span{start:0,end:0}}];
        assert_eq!(tokens, eof_tokens);
    }

    #[test]
    fn test_simple() {
        let input = "let x = 5;";
        let tokens = lex(input).unwrap();

        assert_eq!(tokens.len(), 6);
    }

    #[test]
    fn test_lexanization_simple_test_file() {
        let filepath = "tests/example_files/simple_test.ms";
        let input_str = std::fs::read_to_string(filepath).unwrap();
        let lex_vec = lex(&input_str).unwrap();

        let simple_test_file_vec = vec![
            Token {
                kind: TokenKind::Let,
                span: Span { start: 0, end: 3 },
            },
            Token {
                kind: TokenKind::Ident,
                span: Span { start: 4, end: 5 },
            },
            Token {
                kind: TokenKind::Equal,
                span: Span { start: 6, end: 7 },
            },
            Token {
                kind: TokenKind::Number(0),
                span: Span { start: 8, end: 9 },
            },
            Token {
                kind: TokenKind::Semicolon,
                span: Span { start: 9, end: 10 },
            },
            Token {
                kind: TokenKind::Newline,
                span: Span { start: 10, end: 11 },
            },
            Token {
                kind: TokenKind::Ident,
                span: Span { start: 11, end: 12 },
            },
            Token {
                kind: TokenKind::Equal,
                span: Span { start: 13, end: 14 },
            },
            Token {
                kind: TokenKind::Ident,
                span: Span { start: 15, end: 16 },
            },
            Token {
                kind: TokenKind::Plus,
                span: Span { start: 17, end: 18 },
            },
            Token {
                kind: TokenKind::Number(1),
                span: Span { start: 19, end: 20 },
            },
            Token {
                kind: TokenKind::Semicolon,
                span: Span { start: 20, end: 21 },
            },
            Token {
                kind: TokenKind::Newline,
                span: Span { start: 21, end: 22 },
            },
            Token {
                kind: TokenKind::Print,
                span: Span { start: 22, end: 27 },
            },
            Token {
                kind: TokenKind::Ident,
                span: Span { start: 28, end: 29 },
            },
            Token {
                kind: TokenKind::Semicolon,
                span: Span { start: 29, end: 30 },
            },
             Token {
                kind: TokenKind::EOF,
                span: Span { start: 30, end: 30 },
            },           
        ];

        assert_eq!(lex_vec, simple_test_file_vec);
    }
}

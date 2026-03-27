use super::parser_helper_func::*;

use crate::{
    lexer::{Span, Token, TokenKind},
    parser::{
        BinOp, Expr, Ident, ParseError, ParseErrorKind, Statement,
        parse_context::{IdentKind, ParseContext},
    },
};

use super::Program;

/// Parser for saving information about the current parse state
struct Parser<'a> {
    tokens: &'a [Token],
    pos: usize,
    input: &'a str,
    context: ParseContext,
}

impl<'a> Parser<'a> {
    pub fn new(tokens: &'a [Token], input: &'a str, context: ParseContext) -> Self {
        Parser {
            tokens,
            pos: 0,
            input,
            context,
        }
    }

    pub fn current(&self) -> &Token {
        &self.tokens[self.pos]
    }

    /// Get Current token and advance to the next token
    /// Own function because it is offen used
    pub fn next(&mut self) -> &Token {
        let pos = self.pos;
        self.advance();
        &self.tokens[pos]
    }

    pub fn peek(&self) -> Option<&Token> {
        self.tokens.get(self.pos)
    }

    pub fn advance(&mut self) {
        self.pos += 1;
    }

    /// Consumes next token and compare it to TokenKind
    /// Returns NonExpectedToken error when TokenKinds differ
    /// Used for parsing the next expected token
    pub fn expect(&mut self, kind: TokenKind) -> Result<Token, ParseError> {
        let token = self.current().clone();
        if token.kind == kind {
            self.advance();
            Ok(token)
        }
        // handle TokenKind with data separatly
        else if let (TokenKind::Number(_), TokenKind::Number(_)) = (token.kind, kind) {
            self.advance();
            Ok(token)
        } else {
            Err(ParseError {
                kind: ParseErrorKind::NonExpectedToken(vec![kind], token.kind),
                span: token.span,
            })
        }
    }

    /// Consumes any non Operand TokenKind associated with Expressions
    /// Returns a NonExpectedToken Error when no expected Token is read
    /// Used for Expression parsing
    pub fn parse_non_operand_expr(&mut self) -> Result<Expr, ParseError> {
        let token = self.current();

        let ret = Ok(match token {
            Token {
                kind: TokenKind::Ident,
                ..
            } => Expr::Ident(
                self.context
                    .classify(token.lexeme(&self.input), token.span)?,
            ),
            Token {
                kind: TokenKind::Number(num),
                ..
            } => Expr::Number(*num),
            _ => {
                return Err(give_non_expected_token_error(
                    &token.kind,
                    vec![TokenKind::Ident, TokenKind::Number(0)],
                    token.span,
                ));
            }
        });
        self.advance();
        ret
    }

    // Consumes tokens gready till final expression is gotten
    // Used for fast Expression parsing
    pub fn parse_expr(&mut self) -> Result<Expr, ParseError> {
        let mut expr = self.parse_non_operand_expr()?;

        while let Some(Token {
            kind: token_kind, ..
        }) = self.peek()
        {
            let op = match token_kind {
                TokenKind::Plus => BinOp::Add,
                TokenKind::Minus => BinOp::Sub,
                _ => break,
            };
            self.advance();

            let right_expr = self.parse_non_operand_expr()?;
            expr = Expr::Binary {
                left: Box::new(expr),
                op,
                right: Box::new(right_expr),
            };
        }

        Ok(expr)
    }

    /// Consumes next Seimicolon or Newline
    /// Returns NonExpectedTokenError when reading any other TokenKind
    pub fn parse_statement_end(&mut self) -> Result<(), ParseError> {
        let token = self.next();

        match token {
            Token {
                kind: TokenKind::Semicolon | TokenKind::Newline,
                ..
            } => Ok(()),

            token => {
                return Err(give_non_expected_token_error(
                    &token.kind,
                    vec![TokenKind::Semicolon, TokenKind::Newline],
                    token.span,
                ));
            }
        }
    }

    /// Consuming following token as Identificator and looking it up in the Parse Context
    /// Retruns Error when not a Ident token
    /// Used for parsing expected Identificator
    pub fn parse_ident(&mut self) -> Result<Ident, ParseError> {
        let token = self.current();

        let ret = match token {
            Token {
                kind: TokenKind::Ident,
                span,
            } => self.context.classify(token.lexeme(&self.input), *span),

            _ => {
                return Err(give_non_expected_token_error(
                    &token.kind,
                    vec![TokenKind::Ident],
                    token.span,
                ));
            }
        };

        self.advance();

        ret
    }

    /// Desides from the first token what statement it is and parses it
    /// Returns ParseError when invalid statement is read
    /// Used for Recursive Descend Parsing Algorithm
    pub fn parse_statement(&mut self) -> Result<Statement, ParseError> {
        let start_token = self.current();
        Ok(match (start_token.span, start_token.kind) {
            (span, token_kind) => {
                match token_kind {
                    TokenKind::Let => 'let_stm: {
                        self.advance();
                        // test for Ident token
                        let token = self.expect(TokenKind::Ident)?;

                        let ident = self.context.new_ident(
                            token.lexeme(self.input),
                            IdentKind::Variable,
                            Span {
                                start: span.start,
                                end: token.span.end,
                            },
                        )?;

                        let token = self.current();

                        match token {
                            Token {
                                kind: TokenKind::Semicolon | TokenKind::Newline,
                                ..
                            } => {
                                self.advance();
                                let new_statement = Statement::Let {
                                    name: ident,
                                    value: None,
                                };

                                break 'let_stm new_statement;
                            }
                            Token {
                                kind: TokenKind::Equal,
                                ..
                            } => self.advance(),

                            token => {
                                return Err(give_non_expected_token_error(
                                    &token.kind,
                                    vec![
                                        TokenKind::Semicolon,
                                        TokenKind::Newline,
                                        TokenKind::Equal,
                                    ],
                                    token.span,
                                ));
                            }
                        }

                        let expr = self.parse_expr()?;

                        self.parse_statement_end()?;

                        Statement::Let {
                            name: ident,
                            value: Some(expr),
                        }
                    }
                    kind @ (TokenKind::Equal
                    | TokenKind::Plus
                    | TokenKind::Minus
                    | TokenKind::Do
                    | TokenKind::End
                    | TokenKind::Number(_)) => {
                        return Err(give_non_expected_token_error(
                            &kind,
                            vec![
                                TokenKind::Let,
                                TokenKind::Loop,
                                TokenKind::Print,
                                TokenKind::Ident,
                            ],
                            span,
                        ));
                    }
                    TokenKind::Newline | TokenKind::Semicolon => {
                        self.advance();
                        Statement::Empty
                    },

                    TokenKind::Loop => {
                        self.advance();
                        let mut expected_token_kinds = vec![TokenKind::Ident];

                        let ident = self.parse_ident()?;

                        expected_token_kinds = vec![TokenKind::Do];

                        let token = self.next();

                        match token {
                            Token {
                                kind: TokenKind::Do,
                                span: _,
                            } => (),
                            _ => {
                                return Err(give_non_expected_token_error(
                                    &token_kind,
                                    expected_token_kinds,
                                    span,
                                ));
                            }
                        }

                        let mut loop_statements = Vec::new();

                        loop {
                            match self.peek() {
                                Some(Token {
                                    kind: TokenKind::End,
                                    ..
                                }) => {
                                    self.advance();
                                    break;
                                }
                                Some(_) => loop_statements.push(self.parse_statement()?),
                                None => {
                                    return Err(ParseError {
                                        kind: ParseErrorKind::UnexpectedEOF(vec![
                                            TokenKind::Let,
                                            TokenKind::Loop,
                                            TokenKind::End,
                                            TokenKind::Ident,
                                            TokenKind::Print,
                                        ]),
                                        span,
                                    });
                                }
                            }
                        }

                        Statement::Loop {
                            var: ident,
                            body: loop_statements,
                        }
                    }
                    TokenKind::Print => {
                        self.advance();
                        let ident = self.parse_ident()?;
                        self.parse_statement_end()?;
                        Statement::Print { name: ident }
                    }
                    TokenKind::Ident => {
                        let ident = self.parse_ident()?;
                        self.expect(TokenKind::Equal)?;

                        let expr = self.parse_expr()?;

                        self.parse_statement_end()?;

                        Statement::Assign {
                            name: ident,
                            value: expr,
                        }
                    }
                }
            }
        })
    }

    /// Consumes all tokens of Parser and parse a program out of it
    /// Returns ParseError when not be able to parse Tokens
    /// Used for Parsing whole file
    pub fn parse_program(&mut self) -> Result<Program, ParseError> {
        let mut statements = Vec::with_capacity(10);

        while self.peek().is_some() {
            statements.push(self.parse_statement()?);
        }

        Ok(Program {
            statements: statements,
        })
    }
}

/// Parser for parsing tokens from lexical analysis to simplified AST
///
/// Design choises
/// - parsers uses stack to see in what loop the statements needs to be added
/// - build up parse context for storing information like variables

pub fn parse(input_tokens: &Vec<Token>, input_str: &str) -> Result<Program, ParseError> {
    let parse_context = ParseContext::new();

    let mut parser: Parser = Parser::new(input_tokens, input_str, parse_context);

    Ok(parser.parse_program()?)
}

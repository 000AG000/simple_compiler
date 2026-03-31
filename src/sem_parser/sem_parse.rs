use log::trace;

use super::sem_parser_helper_func::*;

use crate::{
    lexer::{GlobalError, Span, Token, TokenKind},
    sem_parser::{
        BinOp, BinOpKind, ErrorKind, Expr, ExprKind, Ident, ParseErrorKind, Statement,
        StatementKind,
        sem_parse_context::{IdentKind, ParseContext},
    },
};

use super::Program;

/// Parser for saving information about the current parse state
struct Parser<'a> {
    tokens: &'a [Token],
    pos: usize,
    input_str: &'a str,
    context: ParseContext,
}

impl<'a> Parser<'a> {
    pub fn new(tokens: &'a [Token], input: &'a str, context: ParseContext) -> Self {
        Parser {
            tokens,
            pos: 0,
            input_str: input,
            context,
        }
    }

    pub fn current(&self) -> &Token {
        debug_assert!(
            self.pos <= self.tokens.len(),
            "Parser position out of bounds"
        );
        &self.tokens[self.pos]
    }

    /// Retrieve the span of the token before the current
    /// Used to get the end of the last read in token for
    /// creating context information for errors
    pub fn get_previous_token_end(&self) -> usize {
        if self.pos == 0 {
            return 0;
        }; // early return if first token

        self.tokens[self.pos - 1].span.end
    }

    /// Get Current token and advance to the next token
    /// Own function because it is often used
    pub fn bump(&mut self) -> &Token {
        let pos = self.pos;
        self.advance_position();
        &self.tokens[pos]
    }

    pub fn peek(&self) -> Option<&Token> {
        self.tokens.get(self.pos)
    }

    pub fn advance_position(&mut self) {
        self.pos += 1;
    }

    /// Consumes next token and compare it to TokenKind
    /// Returns NonExpectedToken error when TokenKinds differ
    /// Used for parsing the next expected token
    pub fn expect(&mut self, kind: TokenKind) -> Result<Token, GlobalError> {
        let token = self.current().clone();
        if token.kind == kind {
            self.advance_position();
            Ok(token)
        }
        // handle TokenKind with data separately
        else if let (TokenKind::Number(_), TokenKind::Number(_)) = (token.kind, kind) {
            self.advance_position();
            Ok(token)
        } else {
            Err(GlobalError {
                kind: ErrorKind::Parse(ParseErrorKind::NonExpectedToken(vec![kind], token.kind)),
                span: token.span,
            })
        }
    }

    /// Consumes any non Operand TokenKind associated with Expressions
    /// Returns a NonExpectedToken Error when no expected Token is read
    /// Used for Expression parsing
    pub fn parse_non_operand_expr(&mut self) -> Result<Expr, GlobalError> {
        let token = self.current();

        let ret = Ok(match token {
            Token {
                kind: TokenKind::Ident,
                ..
            } => Expr::new(
                ExprKind::Ident(
                    self.context
                        .classify(token.lexeme(self.input_str), token.span)?,
                ),
                token.span,
            ),
            Token {
                kind: TokenKind::Number(num),
                ..
            } => Expr::new(ExprKind::Number(*num), token.span),
            _ => {
                return Err(give_non_expected_token_error(
                    &token.kind,
                    vec![TokenKind::Ident, TokenKind::Number(0)],
                    token.span,
                ));
            }
        });
        self.advance_position();
        ret
    }

    // Consumes tokens greedy till final expression is gotten
    // Used for fast Expression parsing
    pub fn parse_expr(&mut self) -> Result<Expr, GlobalError> {
        let mut expr = self.parse_non_operand_expr()?;
        let expr_span_start = expr.span.start;

        while let Some(Token {
            kind: token_kind,
            span,
        }) = self.peek()
        {
            let op = match token_kind {
                TokenKind::Plus => BinOp::new(BinOpKind::Add, *span),
                TokenKind::Minus => BinOp::new(BinOpKind::Sub, *span),
                _ => break,
            };
            self.advance_position();

            let right_expr = self.parse_non_operand_expr()?;
            let expr_span_end = right_expr.span.end;
            expr = Expr::new(
                ExprKind::Binary {
                    left: Box::new(expr),
                    op,
                    right: Box::new(right_expr),
                },
                Span {
                    start: expr_span_start,
                    end: expr_span_end,
                },
            );
        }

        Ok(expr)
    }

    /// Consumes next Semicolon or Newline
    /// Returns NonExpectedTokenError when reading any other TokenKind
    pub fn parse_statement_end(&mut self) -> Result<(), GlobalError> {
        let token = self.bump();

        match token {
            Token {
                kind: TokenKind::Semicolon | TokenKind::Newline,
                ..
            } => Ok(()),

            token => Err(give_non_expected_token_error(
                &token.kind,
                vec![TokenKind::Semicolon, TokenKind::Newline],
                token.span,
            )),
        }
    }

    /// Consuming following token as Identifier and looking it up in the Parse Context
    /// Returns Error when not a Ident token
    /// Used for parsing expected Identifier
    pub fn parse_ident(&mut self) -> Result<Ident, GlobalError> {
        let token = self.current();

        let ret = match token {
            Token {
                kind: TokenKind::Ident,
                span,
            } => self.context.classify(token.lexeme(self.input_str), *span),

            _ => {
                return Err(give_non_expected_token_error(
                    &token.kind,
                    vec![TokenKind::Ident],
                    token.span,
                ));
            }
        };

        self.advance_position();

        ret
    }

    /// Decides from the first token what statement it is and parses it
    /// Returns GlobalError when invalid statement is read
    /// Used for Recursive Descend Parsing Algorithm
    pub fn parse_statement(&mut self) -> Result<Statement, GlobalError> {
        let start_token = self.current();
        Ok({
            let (span, token_kind) = (start_token.span, start_token.kind);
            match token_kind {
                TokenKind::Let => self.parse_let(start_token.span)?,
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
                TokenKind::EOF => {
                    return Err(GlobalError::parse(
                        ParseErrorKind::UnexpectedEnd,
                        Span {
                            start: self.input_str.len(),
                            end: self.input_str.len(),
                        },
                    ));
                }
                TokenKind::Newline | TokenKind::Semicolon => {
                    self.advance_position();
                    Statement::new(StatementKind::Empty, span)
                }

                TokenKind::Loop => self.parse_loop()?,
                TokenKind::Print => {
                    self.advance_position();
                    let ident = self.parse_ident()?;
                    self.parse_statement_end()?;
                    let statement_span = Span {
                        start: span.start,
                        end: self.get_previous_token_end(),
                    };
                    Statement::new(StatementKind::Print { name: ident }, statement_span)
                }
                TokenKind::Ident => {
                    let ident = self.parse_ident()?;
                    self.expect(TokenKind::Equal)?;

                    let expr = self.parse_expr()?;

                    self.parse_statement_end()?;
                    let statement_span = Span {
                        start: span.start,
                        end: self.get_previous_token_end(),
                    };

                    Statement::new(
                        StatementKind::Assign {
                            name: ident,
                            value: expr,
                        },
                        statement_span,
                    )
                }
            }
        })
    }

    /// Consumes all tokens of Parser and parse a program out of it
    /// Returns GlobalError when not be able to parse Tokens
    /// Used for Parsing whole file
    pub fn parse_program(&mut self) -> Result<Program, GlobalError> {
        let mut statements = Vec::with_capacity(10);

        while self.current().kind != TokenKind::EOF {
            let statement = self.parse_statement()?;
            trace!("Added Statement {}", statement.pretty_print(self.input_str));
            statements.push(statement);
        }

        Ok(Program { statements })
    }

    /// Consumes the next tokens bound to a let statement
    /// Return GlobalError when not able to parse let statement
    /// Used as shorthand for parsing statement function
    pub fn parse_let(&mut self, let_token_span: Span) -> Result<Statement, GlobalError> {
        self.advance_position();
        // test for Ident token
        let token = self.expect(TokenKind::Ident)?;

        let ident = self.context.new_ident(
            token.lexeme(self.input_str),
            IdentKind::Variable,
            Span {
                start: let_token_span.start,
                end: token.span.end,
            },
        )?;

        let token = self.current();

        match token {
            Token {
                kind: TokenKind::Semicolon | TokenKind::Newline,
                ..
            } => {
                let new_statement = Statement::new(
                    StatementKind::Let {
                        name: ident,
                        value: None,
                    },
                    Span {
                        start: let_token_span.start,
                        end: token.span.end,
                    },
                );

                self.advance_position();
                return Ok(new_statement);
            }
            Token {
                kind: TokenKind::Equal,
                ..
            } => self.advance_position(),

            token => {
                return Err(give_non_expected_token_error(
                    &token.kind,
                    vec![TokenKind::Semicolon, TokenKind::Newline, TokenKind::Equal],
                    token.span,
                ));
            }
        }

        let expr = self.parse_expr()?;

        self.parse_statement_end()?;

        Ok(Statement::new(
            StatementKind::Let {
                name: ident,
                value: Some(expr),
            },
            Span {
                start: let_token_span.start,
                end: self.get_previous_token_end(),
            },
        ))
    }

    /// Consumes the next tokens bound to a loop statement
    /// Return GlobalError when not able to parse loop statement
    /// Used as shorthand for parsing statement function
    pub fn parse_loop(&mut self) -> Result<Statement, GlobalError> {
        let loop_span = self.current().span;
        self.advance_position();

        let ident = self.parse_ident()?;

        let token = self.bump();

        match token {
            Token {
                kind: TokenKind::Do,
                ..
            } => (),
            Token { kind: _, span } => {
                return Err(give_non_expected_token_error(
                    &token.kind,
                    vec![TokenKind::Do],
                    *span,
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
                    self.advance_position();
                    break;
                }
                Some(_) => loop_statements.push(self.parse_statement()?),
                None => {
                    return Err(GlobalError::parse(
                        ParseErrorKind::UnexpectedEOF(vec![
                            TokenKind::Let,
                            TokenKind::Loop,
                            TokenKind::End,
                            TokenKind::Ident,
                            TokenKind::Print,
                        ]),
                        Span {
                            start: self.input_str.len(),
                            end: self.input_str.len(),
                        },
                    ));
                }
            }
        }

        Ok(Statement::new(
            StatementKind::Loop {
                var: ident,
                body: loop_statements,
            },
            Span {
                start: loop_span.start,
                end: self.get_previous_token_end(),
            },
        ))
    }
}

/// Parser for parsing tokens from lexical analysis to simplified AST
///
/// Design choices
/// - parser context also handling semantic analysis of variables (no other semantic analysis needed)
/// - build up parse context for storing information like variables
///
/// example usage:
/// ```
/// use simple_interpreter::lexer::lex_ascii;
/// use simple_interpreter::sem_parser::parse;
/// let input_str = "let x = 0;";
/// let input_tokens = lex_ascii(input_str).unwrap();
/// parse(&input_tokens,input_str).unwrap();
/// ```
pub fn parse(input_tokens: &[Token], input_str: &str) -> Result<Program, GlobalError> {
    let parse_context = ParseContext::new();

    let mut parser: Parser = Parser::new(input_tokens, input_str, parse_context);

    parser.parse_program()
}

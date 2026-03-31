use crate::{
    interpreter::{
        GlobalError, RuntimeErrorKind,
        frame::{Frame, FrameKind},
    },
    sem_parser::{Expr, IdentId, Program, Statement},
};
use log::debug;
use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq)]
/// Runtime Context for storing variables
/// currently only usize type variables are supported
struct RuntimeContext {
    variables: HashMap<IdentId, usize>,
}

impl RuntimeContext {
    pub fn new() -> Self {
        Self {
            variables: HashMap::new(),
        }
    }

    /// add variable to context
    /// the variable be 0 initialized
    pub fn add_variable(&mut self, ident_id: &IdentId) {
        self.variables.insert(*ident_id, 0);
    }

    // setting a variable in the current context
    pub fn set_variable(&mut self, ident_id: &IdentId, content: usize) {
        self.variables.insert(*ident_id, content);
    }

    /// checking if variable consists in context by variable id
    pub fn contains_variable(&self, ident_id: &IdentId) -> bool {
        self.variables.contains_key(ident_id)
    }

    // getting the current context of
    pub fn get_variable(&self, ident_id: &IdentId) -> Result<usize, RuntimeErrorKind> {
        self.variables
            .get(ident_id)
            .copied()
            .ok_or(RuntimeErrorKind::VariableNotFound)
    }
}

#[derive(Debug, Clone, PartialEq)]
/// Interpreter for handling context and execute steps
/// Used for stepwise execute statements
struct Interpreter<'a> {
    context: RuntimeContext,
    stack: Vec<Frame<'a>>,
    /// input string associated with the program for providing error and debugging context
    input_str: &'a str,
}

impl<'a> Interpreter<'a> {
    /// execute next statement
    /// when reaching the end it returns false
    pub fn step(&mut self) -> Result<bool, GlobalError> {
        let statement_index = match self.fetch_next_statement_idx() {
            Some(statement) => statement,
            None => loop {
                self.stack.pop();

                // was last frame
                if self.stack.is_empty() {
                    return Ok(false);
                }

                if let Some(frame) = self.fetch_next_statement_idx() {
                    break frame;
                }
            },
        };
        let current_frame = match self.stack.last_mut() {
            Some(frame) => frame,
            None => return Ok(false),
        };
        debug_assert!(
            current_frame.statements.len() >= statement_index,
            "interpreters statement index out of"
        );
        let statement = &current_frame.statements[statement_index];

        self.interpret_statement(statement)?;

        Ok(true)
    }

    /// interpret expression in the current context
    pub fn interpret_expr(&mut self, expr: &Expr) -> Result<usize, GlobalError> {
        match &expr.node {
            crate::sem_parser::ExprKind::Number(num) => Ok(*num),
            crate::sem_parser::ExprKind::Ident(ident) => {
                match self.context.get_variable(&ident.ident_number) {
                    Ok(variable) => Ok(variable),
                    Err(kind) => Err(GlobalError::runtime(kind, ident.span)),
                }
            }
            crate::sem_parser::ExprKind::Binary { left, op, right } => {
                let left_expr_eval = self.interpret_expr(left)?;
                let right_expr_eval = self.interpret_expr(right)?;

                Ok(match op.node {
                    crate::sem_parser::BinOpKind::Add => left_expr_eval + right_expr_eval,
                    crate::sem_parser::BinOpKind::Sub => {
                        left_expr_eval.saturating_sub(right_expr_eval)
                    }
                })
            }
        }
    }

    /// get statement index of the frame
    /// statement index is used for borrowing reasons
    /// returning None, when current frame is at the end of execution
    pub fn fetch_next_statement_idx(&mut self) -> Option<usize> {
        let current_frame = self.stack.last_mut()?;
        let mut current_ip = current_frame.ip;
        if current_ip >= current_frame.statements.len() {
            match &mut current_frame.kind {
                FrameKind::Loop { remaining } => {
                    if *remaining == 0 {
                        return None;
                    }
                    *remaining -= 1;
                    current_ip = 0;
                }
                _ => return None,
            }
        }
        current_frame.ip = current_ip + 1;
        Some(current_ip)
    }

    /// interpret statement in current context
    pub fn interpret_statement(&mut self, statement: &'a Statement) -> Result<(), GlobalError> {
        debug!(
            "Executing statement: {}",
            statement.pretty_print(self.input_str)
        );
        match &statement.node {
            crate::sem_parser::StatementKind::Let { name, value } => {
                if self.context.contains_variable(&name.ident_number) {
                    return Err(GlobalError::runtime(
                        RuntimeErrorKind::VariableAlreadyDefined,
                        statement.span,
                    ));
                }

                self.context.add_variable(&name.ident_number);

                if let Some(expr) = value {
                    let eval_expr = self.interpret_expr(expr)?;
                    self.context.set_variable(&name.ident_number, eval_expr);
                }
            }
            crate::sem_parser::StatementKind::Assign { name, value } => {
                let eval_expr = self.interpret_expr(value)?;
                self.context.set_variable(&name.ident_number, eval_expr);
            }
            crate::sem_parser::StatementKind::Loop { var, body } => {
                let num = match self.context.get_variable(&var.ident_number) {
                    Ok(num) => num,
                    Err(kind) => return Err(GlobalError::runtime(kind, var.span)),
                };
                if num != 0 {
                    let loop_frame = Frame::new(FrameKind::Loop { remaining: num - 1 }, body);
                    self.stack.push(loop_frame);
                }
            }
            crate::sem_parser::StatementKind::Print { name: ident } => {
                let variable = match self.context.get_variable(&ident.ident_number) {
                    Ok(var) => var,
                    Err(kind) => return Err(GlobalError::runtime(kind, ident.span)),
                };
                println!("{}", variable);
            }
            crate::sem_parser::StatementKind::Empty => (),
        }
        Ok(())
    }
}

/// execute parsed program till end
///
/// Design choices:
/// - used frame based ExecutionContext
/// - introduced Interpreter struct to handle stack frame and execution context
///
/// example usage:
/// ```
/// use simple_interpreter::lexer::lex_ascii;
/// use simple_interpreter::sem_parser::parse;
/// use simple_interpreter::interpreter::exec;
/// let input_str = "let x = 0;print x;";
/// let input_tokens = lex_ascii(input_str).unwrap();
/// let program = parse(&input_tokens,input_str).unwrap();
/// exec(program,input_str);
/// ```
pub fn exec(program: Program, input_str: &str) -> Result<(), GlobalError> {
    let init_frame = Frame::new(FrameKind::Block, &program.statements);
    let mut interpreter = Interpreter {
        context: RuntimeContext::new(),
        stack: vec![init_frame],
        input_str,
    };
    while match interpreter.step() {
        Ok(is_done) => is_done,
        Err(runtime_err) => {
            return Err(runtime_err);
        }
    } {}

    Ok(())
}

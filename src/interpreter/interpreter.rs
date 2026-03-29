use std::collections::HashMap;

use crate::{
    interpreter::{
        frame::{Frame, FrameKind},
        runtime_err::{RuntimeError, RuntimeErrorkind},
    },
    sem_parser::{Expr, IdentId, Program, Statement},
};

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

    // setting a variabe in the current context
    pub fn set_variable(&mut self, ident_id: &IdentId, content: usize) {
        self.variables.insert(*ident_id, content);
    }

    /// checking if variable consists in context by variable id
    pub fn contains_variable(&self, ident_id: &IdentId) -> bool {
        self.variables.contains_key(ident_id)
    }

    // getting the current context of
    pub fn get_variable(&self, ident_id: &IdentId) -> usize {
        debug_assert!(self.contains_variable(ident_id),"variable not found");
        self.variables[ident_id]
    }
}

#[derive(Debug, Clone, PartialEq)]
/// Iterpeter for handling context and execute steps
/// Used for stepwise execute statements
struct Interpreter<'a> {
    context: RuntimeContext,
    stack: Vec<Frame<'a>>,
}


impl<'a> Interpreter<'a> {
    /// execute next statement
    /// when reaching the end it returns false
    pub fn step(&mut self) -> Result<bool, RuntimeError> {
        let statement_index = match self.fetch_next_statement_idx() {
            Some(statement) => statement,
            None => loop {
                self.stack.pop();

                // was last frame
                if self.stack.len() == 0{
                    return Ok(false)
                }

                match self.fetch_next_statement_idx() {
                    Some(frame) => break frame,
                    None => (),
                }
            },
        };
        let current_frame = match self.stack.last_mut() {
            Some(frame) => frame,
            None => return Ok(false),
        };
        debug_assert!(current_frame.statements.len() >= statement_index,"interpreters statement index out of bounds");
        let statement = &current_frame.statements[statement_index];

        self.interpret_statement(statement)?;

        Ok(true)
    }

    /// interpret expression in the current context
    pub fn interpret_expr(&mut self, expr: &Expr) -> usize {
        match &expr.node {
            crate::sem_parser::ExprKind::Number(num) => *num,
            crate::sem_parser::ExprKind::Ident(ident) => {
                self.context.get_variable(&ident.ident_number)
            }
            crate::sem_parser::ExprKind::Binary { left, op, right } => {
                let left_expr_eval = self.interpret_expr(&left);
                let right_expr_eval = self.interpret_expr(&right);

                match op.node {
                    crate::sem_parser::BinOpKind::Add => left_expr_eval + right_expr_eval,
                    crate::sem_parser::BinOpKind::Sub => {
                        if left_expr_eval >= right_expr_eval {
                            left_expr_eval - right_expr_eval
                        } else {
                            0
                        }
                    }
                }
            }
        }
    }

    /// get statement index of the frame
    /// statement index is used for borrowing reasons
    /// returning None, when current frame is at the end of execution
    pub fn fetch_next_statement_idx(&mut self) -> Option<usize>{
        let current_frame = match self.stack.last_mut() {
            Some(frame) => frame,
            None => return None,
        };
        let mut current_ip = current_frame.ip;
        if current_ip >= current_frame.statements.len(){
            match &mut current_frame.kind{
                FrameKind::Loop { remaining } => {
                    if *remaining == 0{
                        return None
                    }
                    *remaining -= 1;
                    current_ip = 0;
                },
                _ => return None
            }

        }
        current_frame.ip = current_ip + 1;
        Some(current_ip)
    }

    /// interpret statement in current context
    pub fn interpret_statement(&mut self, statement: &'a Statement) -> Result<(), RuntimeError> {
        match &statement.node {
            crate::sem_parser::StatementKind::Let { name, value } => {
                if self.context.contains_variable(&name.ident_number) {
                    return Err(RuntimeError {
                        kind: RuntimeErrorkind::VariableAlreadyDefined,
                        span: statement.span,
                    });
                }

                self.context.add_variable(&name.ident_number);

                if let Some(expr) = value {
                    let eval_expr = self.interpret_expr(expr);
                    self.context.set_variable(&name.ident_number, eval_expr);
                }
            }
            crate::sem_parser::StatementKind::Assign { name, value } => {
                let eval_expr = self.interpret_expr(value);
                self.context.set_variable(&name.ident_number, eval_expr);
            }
            crate::sem_parser::StatementKind::Loop { var, body } => {
                let num = self.context.get_variable(&var.ident_number);
                if num != 0 {
                    let loop_frame =
                        Frame::new(FrameKind::Loop { remaining: num - 1 }, body);
                    self.stack.push(loop_frame);
                    
                }
            }
            crate::sem_parser::StatementKind::Print { name: ident } => {
                println!("{}", self.context.get_variable(&ident.ident_number));
            }
            crate::sem_parser::StatementKind::Empty => (),
        }
        Ok(())
    }
}

/// execute parsed program till end
/// 
/// Desing choises:
/// - used frame based ExecutionContext
/// - introduced Interpreter struct to handle stack frame and execution context
/// 
/// example usage:
/// ```
/// use simple_interpreter::lexer::lex;
/// use simple_interpreter::sem_parser::parse;
/// use simple_interpreter::interpreter::exec;
/// let input_str = "let x = 0;print x;";
/// let input_tokens = lex(input_str).unwrap();
/// let program = parse(&input_tokens,input_str).unwrap();
/// exec(program,input_str);
/// ```
pub fn exec(program: Program, input_str: &str) -> Result<(), RuntimeError> {
    let init_frame = Frame::new(FrameKind::Block, &program.statements);
    let mut interpreter = Interpreter {
        context: RuntimeContext::new(),
        stack: vec![init_frame],
    };
    while let true = match interpreter.step() {
        Ok(is_done) => is_done,
        Err(runtime_err) => {
            println!("\n---ERROR OCCURED---\n{}", runtime_err.generate_error_msg(input_str));
            return Err(runtime_err);
        }
    } {}

    Ok(())
}

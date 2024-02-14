use std::io::{Read, Write};

use crate::{parser::statements::Statement, utils::errors::LanguageError};

pub mod encoder;

/// The size of the memory used by the interpreter.
pub const MEMORY_SIZE: usize = 1024 * 32;

/// The interpreter.
///
/// # Fields
///
/// * `ast` - The AST produced by the parser.
/// * `io` - The I/O to use.
/// * `memory` - The memory used by the interpreter.
/// * `pointer` - The current position in the memory.
#[derive(Debug)]
pub struct Interpreter<'a, R: Read, W: Write> {
    ast: &'a [Statement],
    io: (&'a mut R, &'a mut W),
    memory: [u8; MEMORY_SIZE],
    pointer: usize,
}

impl<'a, R: Read, W: Write> Interpreter<'a, R, W> {
    /// Create a new interpreter.
    ///
    /// # Arguments
    ///
    /// * `io` - The I/O to use.
    /// * `ast` - The AST produced by the parser.
    ///
    /// # Returns
    ///
    /// * `Interpreter` - The new interpreter.
    #[must_use]
    pub fn new(ast: &'a [Statement], io: (&'a mut R, &'a mut W)) -> Self {
        Self {
            ast,
            io,
            memory: [0; MEMORY_SIZE],
            pointer: 0,
        }
    }

    /// Run the interpreter.
    ///
    /// # Returns
    ///
    /// * `Result<(), LanguageError>` - The result of running the interpreter.
    ///
    /// # Errors
    ///
    /// * `LanguageError` - An error occurred while walking the AST.
    pub fn run(&mut self) -> Result<(), LanguageError> {
        for statement in self.ast {
            self.interpret(statement)?;
        }

        Ok(())
    }

    fn interpret(&mut self, statement: &Statement) -> Result<(), LanguageError> {
        match statement {
            Statement::Increment => self.increment(),
            Statement::Decrement => self.decrement(),
            Statement::MoveLeft => self.move_left(),
            Statement::MoveRight => self.move_right(),
            Statement::Input => self.input()?,
            Statement::Output => self.output()?,
            Statement::Loop(statements) => self.r#loop(statements)?,
        };

        Ok(())
    }

    fn increment(&mut self) {
        self.memory[self.pointer] += 1;
    }

    fn decrement(&mut self) {
        self.memory[self.pointer] -= 1;
    }

    fn move_left(&mut self) {
        if self.pointer == 0 {
            self.pointer = MEMORY_SIZE - 1;
        } else {
            self.pointer -= 1;
        }
    }

    fn move_right(&mut self) {
        self.pointer = (self.pointer + 1) % MEMORY_SIZE;
    }

    fn input(&mut self) -> Result<(), LanguageError> {
        let mut buf = [0; 1];
        self.io.0.read_exact(&mut buf)?;

        self.memory[self.pointer] = buf[0];

        Ok(())
    }

    fn output(&mut self) -> Result<(), LanguageError> {
        let byte = self.memory[self.pointer] as char;

        Ok(write!(self.io.1, "{byte}")?)
    }

    fn r#loop(&mut self, statements: &[Statement]) -> Result<(), LanguageError> {
        let start = self.pointer;

        while self.memory[start] != 0 {
            for statement in statements {
                self.interpret(statement)?;
            }
        }

        Ok(())
    }
}

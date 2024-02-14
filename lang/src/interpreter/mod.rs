use std::io::Read;

use crate::parser::statements::Statement;

/// The size of the memory used by the interpreter.
pub const MEMORY_SIZE: usize = 1024 * 32;

/// The interpreter.
///
/// # Fields
///
/// * `ast` - The AST produced by the parser.
/// * `memory` - The memory used by the interpreter.
/// * `pointer` - The current position in the memory.
#[derive(Debug)]
pub struct Interpreter<'a> {
    ast: &'a [Statement],
    memory: [u8; MEMORY_SIZE],
    pointer: usize,
}

impl<'a> Interpreter<'a> {
    /// Create a new interpreter.
    ///
    /// # Arguments
    ///
    /// * `ast` - The AST produced by the parser.
    ///
    /// # Returns
    ///
    /// * `Interpreter` - The new interpreter.
    #[must_use]
    pub const fn new(ast: &'a [Statement]) -> Self {
        Self {
            ast,
            memory: [0; MEMORY_SIZE],
            pointer: 0,
        }
    }

    pub fn run(&mut self) {
        for statement in self.ast {
            self.interpret(statement);
        }
    }

    fn interpret(&mut self, statement: &Statement) {
        match statement {
            Statement::Increment => self.increment(),
            Statement::Decrement => self.decrement(),
            Statement::MoveLeft => self.move_left(),
            Statement::MoveRight => self.move_right(),
            Statement::Input => self.input(),
            Statement::Output => self.output(),
            Statement::Loop(statements) => self.r#loop(statements),
        }
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

    fn input(&mut self) {
        let byte = std::io::stdin()
            .bytes()
            .next()
            .and_then(Result::ok)
            .unwrap_or(0);

        self.memory[self.pointer] = byte;
    }

    fn output(&self) {
        print!("{}", self.memory[self.pointer] as char);
    }

    fn r#loop(&mut self, statements: &[Statement]) {
        let start = self.pointer;

        while self.memory[start] != 0 {
            for statement in statements {
                self.interpret(statement);
            }
        }
    }
}

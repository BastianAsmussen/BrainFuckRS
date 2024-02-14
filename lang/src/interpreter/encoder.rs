use super::MEMORY_SIZE;
use crate::parser::statements::Statement;

#[derive(Debug)]
pub struct Encoder<'a> {
    text: &'a str,
    memory: [u8; MEMORY_SIZE],
    pointer: usize,
}

impl<'a> Encoder<'a> {
    pub fn new(text: &'a str) -> Self {
        Self {
            text,
            memory: [0; MEMORY_SIZE],
            pointer: 0,
        }
    }

    pub fn encode(&mut self) -> Vec<Statement> {
        let mut statements = vec![];

        for c in self.text.chars() {
            let mut steps = self.backtrace(c as u8);
            self.apply(&steps);

            statements.append(&mut steps);
        }

        statements
    }

    fn backtrace(&mut self, c: u8) -> Vec<Statement> {
        let mut steps = vec![];

        let mut diff = c as i8 - self.memory[self.pointer] as i8;
        for _ in 0..diff.abs() {
            let step = if diff > 0 {
                Statement::Increment
            } else {
                Statement::Decrement
            };

            steps.push(step);
        }

        self.apply(&steps);

        steps
    }

    fn apply(&mut self, steps: &[Statement]) {
        for step in steps {
            match step {
                Statement::Increment => self.memory[self.pointer] += 1,
                Statement::Decrement => self.memory[self.pointer] -= 1,
                Statement::MoveRight => self.pointer += 1,
                Statement::MoveLeft => self.pointer -= 1,
                Statement::Loop(statements) => {
                    let start = self.pointer;

                    while start != 0 {
                        self.apply(statements);
                    }
                }
                _ => (),
            }
        }
    }
}

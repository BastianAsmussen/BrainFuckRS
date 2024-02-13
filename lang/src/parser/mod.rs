pub mod errors;
pub mod statements;

use crate::lexer::tokens::kinds::TokenKind;
use crate::lexer::tokens::Token;
use crate::parser::errors::ParserError;
use crate::parser::statements::Statement;

/// The parser.
///
/// # Fields
///
/// * `tokens` - The tokens to parse.
/// * `position` - The current position in the tokens.
#[derive(Debug)]
pub struct Parser<'a> {
    tokens: &'a [Token],
    position: usize,
}

impl<'a> Parser<'a> {
    /// Create a new parser.
    ///
    /// # Arguments
    ///
    /// * `tokens` - The tokens to parse.
    ///
    /// # Returns
    ///
    /// * `Parser` - The new parser.
    #[must_use]
    pub const fn new(tokens: &'a [Token]) -> Self {
        Self {
            tokens,
            position: 0,
        }
    }

    /// Parse the tokens into statements.
    ///
    /// # Returns
    ///
    /// * `Result<Vec<Statement>, ParserError>` - The statements produced by the parser.
    ///
    /// # Errors
    ///
    /// * `ParserError` - An error occurred while parsing.
    pub fn parse(&mut self) -> Result<Vec<Statement>, ParserError> {
        let mut statements = Vec::new();

        while self.peek().is_some() {
            let statement = self.statement()?;
            statements.push(statement);

            self.advance();
        }

        Ok(statements)
    }

    /// Get the next token.
    ///
    /// # Returns
    ///
    /// * `Option<&Token>` - The next token, if there is one.
    fn peek(&self) -> Option<&Token> {
        self.tokens.get(self.position)
    }

    /// Advance to the next token.
    fn advance(&mut self) {
        self.position += 1;
    }

    /// Parse a statement.
    ///
    /// # Returns
    ///
    /// * `Result<Statement, ParserError>` - The statement produced by the parser.
    ///
    /// # Errors
    ///
    /// * `ParserError` - An error occurred while parsing.
    fn statement(&mut self) -> Result<Statement, ParserError> {
        let token = self
            .peek()
            .ok_or(ParserError::UnexpectedToken(TokenKind::EndOfFile))?;

        let statement = match token.kind {
            TokenKind::Plus => Statement::Increment,
            TokenKind::Minus => Statement::Decrement,
            TokenKind::GreaterThan => Statement::MoveRight,
            TokenKind::LessThan => Statement::MoveLeft,
            TokenKind::OpeningBracket => {
                let mut statements = Vec::new();

                self.advance();

                while let Some(token) = self.peek() {
                    if token.kind == TokenKind::ClosingBracket {
                        break;
                    }

                    let statement = self.statement()?;
                    statements.push(statement);

                    self.advance();
                }

                Statement::Loop(statements)
            }
            TokenKind::Comma => Statement::Input,
            TokenKind::Dot => Statement::Output,
            _ => return Err(ParserError::UnexpectedToken(token.kind)),
        };

        Ok(statement)
    }
}

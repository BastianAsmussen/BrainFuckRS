use crate::utils::errors::LanguageError;

use self::tokens::{kinds::TokenKind, Token};

pub mod tokens;

/// The lexer.
///
/// # Fields
///
/// * `source` - The source code to lex.
/// * `position` - The current position in the source code.
#[derive(Debug)]
pub struct Lexer<'a> {
    source: &'a str,
    position: usize,
}

impl<'a> Lexer<'a> {
    /// Create a new lexer.
    ///
    /// # Arguments
    ///
    /// * `source` - The source code to lex.
    ///
    /// # Returns
    ///
    /// * `Lexer` - The new lexer.
    #[must_use]
    pub const fn new(source: &'a str) -> Self {
        Self {
            source,
            position: 0,
        }
    }

    /// Lex the entire source code.
    ///
    /// # Returns
    ///
    /// * `Result<Vec<Token>, LexerError>` - The tokens produced by the lexer.
    ///
    /// # Errors
    ///
    /// * `LexerError` - An error occurred while lexing.
    pub fn lex(&mut self) -> Result<Vec<Token>, LanguageError> {
        let mut tokens = Vec::new();

        while let Some(token) = self.next_token()? {
            tokens.push(token);
        }

        Ok(tokens)
    }

    /// Get the next token.
    ///
    /// # Returns
    ///
    /// * `Result<Option<Token>, LexerError>` - The next token, if there is one.
    ///
    /// # Errors
    ///
    /// * `LexerError` - An error occurred while lexing.
    fn next_token(&mut self) -> Result<Option<Token>, LanguageError> {
        let Some(c) = self.peek() else {
            return Ok(None);
        };

        if c.is_whitespace() {
            self.advance();

            return self.next_token();
        }

        let start = self.position;
        let kind = TokenKind::try_from(c)?;
        self.advance();

        let token = Token::new(kind, start..self.position);

        Ok(Some(token))
    }

    /// Advance the lexer to the next character.
    fn advance(&mut self) {
        self.position += 1;
    }

    /// Peek at the next character.
    ///
    /// # Returns
    ///
    /// * `Option<char` - The next character, if there is one.
    fn peek(&self) -> Option<char> {
        self.source.chars().nth(self.position)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[allow(clippy::unwrap_used)]
    fn test_lex() {
        let source = "+-<>,.[]";

        let actual = Lexer::new(source).lex().unwrap();
        let expected = vec![
            Token::new(TokenKind::Plus, 0..1),
            Token::new(TokenKind::Minus, 1..2),
            Token::new(TokenKind::LessThan, 2..3),
            Token::new(TokenKind::GreaterThan, 3..4),
            Token::new(TokenKind::Comma, 4..5),
            Token::new(TokenKind::Dot, 5..6),
            Token::new(TokenKind::OpeningBracket, 6..7),
            Token::new(TokenKind::ClosingBracket, 7..8),
        ];

        assert_eq!(actual, expected);
    }

    #[test]
    fn test_lex_error() {
        let source = "a";

        let actual = Lexer::new(source).lex();
        let expected = Err(LanguageError::UnexpectedCharacter('a'));

        assert_eq!(actual, expected);
    }
}

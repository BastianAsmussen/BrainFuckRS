use thiserror::Error;

use crate::lexer::tokens::kinds::TokenKind;

/// Errors that can occur while interpreting a file.
///
/// # Variants
///
/// * `UnexpectedCharacter` - An unexpected character was encountered.
/// * `UnexpectedToken` - An unexpected token was encountered.
#[derive(Debug, Error, Eq, PartialEq)]
pub enum LanguageError {
    #[error("Unexpected character: {0}")]
    UnexpectedCharacter(char),
    #[error("Unexpected token: {0}")]
    UnexpectedToken(TokenKind),
}

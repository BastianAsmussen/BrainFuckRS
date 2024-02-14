use thiserror::Error;

use crate::lexer::tokens::kinds::TokenKind;

/// Errors that can occur while interpreting a file.
///
/// # Variants
///
/// * `UnexpectedCharacter` - An unexpected character was encountered.
/// * `UnexpectedToken` - An unexpected token was encountered.
/// * `IoError` - An I/O error occurred.
#[derive(Debug, Error)]
pub enum LanguageError {
    #[error("Unexpected character: {0}")]
    UnexpectedCharacter(char),
    #[error("Unexpected token: {0}")]
    UnexpectedToken(TokenKind),
    #[error("I/O error: {0}")]
    IoError(#[from] std::io::Error),
}

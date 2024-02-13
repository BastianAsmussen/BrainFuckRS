use thiserror::Error;

use crate::lexer::tokens::kinds::TokenKind;

/// Errors that can occur while parsing.
///
/// # Variants
///
/// * `UnexpectedToken` - An unexpected token was encountered.
#[derive(Debug, Error, Eq, PartialEq)]
pub enum ParserError {
    #[error("Unexpected token: {0}")]
    UnexpectedToken(TokenKind),
}

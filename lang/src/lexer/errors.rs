use thiserror::Error;

/// Errors that can occur while lexing.
///
/// # Variants
///
/// * `UnexpectedCharacter` - An unexpected character was encountered.
#[derive(Debug, Error, Eq, PartialEq)]
pub enum LexerError {
    #[error("Unexpected character: {0}")]
    UnexpectedCharacter(char),
}

use self::kinds::TokenKind;
use std::ops::Range;

pub mod kinds;

/// A token produced by the lexer.
///
/// # Fields
///
/// * `kind` - The kind of token.
/// * `span` - The range of characters in the source code that this token covers.
#[derive(Debug, Eq, PartialEq)]
pub struct Token {
    pub kind: TokenKind,
    pub span: Range<usize>,
}

impl Token {
    /// Create a new token.
    ///
    /// # Arguments
    ///
    /// * `kind` - The kind of token.
    /// * `span` - The range of characters in the source code that this token covers.
    ///
    /// # Returns
    ///
    /// * `Token` - The new token.
    #[must_use]
    pub const fn new(kind: TokenKind, span: Range<usize>) -> Self {
        Self { kind, span }
    }
}

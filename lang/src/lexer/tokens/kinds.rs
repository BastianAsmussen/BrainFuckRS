use std::fmt::Display;

use crate::utils::errors::LanguageError;

/// All the kinds of tokens the lexer can produce.
///
/// # Variants
///
/// * `Plus` - The `+` symbol.
/// * `Minus` - The `-` symbol.
/// * `GreaterThan` - The `>` symbol.
/// * `LessThan` - The `<` symbol.
/// * `OpeningBracket` - The `[` symbol.
/// * `ClosingBracket` - The `]` symbol.
/// * `Comma` - The `,` symbol.
/// * `Dot` - The `.` symbol.
/// * `EndOfFile` - Marks the end of the file.
#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum TokenKind {
    Plus,
    Minus,
    GreaterThan,
    LessThan,
    OpeningBracket,
    ClosingBracket,
    Comma,
    Dot,
    EndOfFile,
}

impl TryFrom<char> for TokenKind {
    type Error = LanguageError;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        let kind = match value {
            '+' => Self::Plus,
            '-' => Self::Minus,
            '>' => Self::GreaterThan,
            '<' => Self::LessThan,
            '[' => Self::OpeningBracket,
            ']' => Self::ClosingBracket,
            ',' => Self::Comma,
            '.' => Self::Dot,
            _ => return Err(LanguageError::UnexpectedCharacter(value)),
        };

        Ok(kind)
    }
}

impl Display for TokenKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let kind = match self {
            Self::Plus => "+",
            Self::Minus => "-",
            Self::GreaterThan => ">",
            Self::LessThan => "<",
            Self::OpeningBracket => "[",
            Self::ClosingBracket => "]",
            Self::Comma => ",",
            Self::Dot => ".",
            Self::EndOfFile => "EoF",
        };

        write!(f, "{kind}")
    }
}

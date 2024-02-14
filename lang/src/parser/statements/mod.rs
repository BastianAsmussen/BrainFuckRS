use std::fmt::Display;

// These statements are just brainf*ck statements like +, -, <, >, etc.
/// The possible statements that can be parsed.
///
/// # Variants
///
/// * `Increment` - Increment the current cell.
/// * `Decrement` - Decrement the current cell.
/// * `MoveLeft` - Move the pointer to the left.
/// * `MoveRight` - Move the pointer to the right.
/// * `Loop` - A loop is a list of statements that are executed until the current cell is 0.
/// * `Input` - Read a byte from the input and store it in the current cell.
/// * `Output` - Output the current cell as a byte.
#[derive(Debug)]
pub enum Statement {
    Increment,
    Decrement,
    MoveLeft,
    MoveRight,
    Loop(Vec<Statement>),
    Input,
    Output,
}

impl Display for Statement {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Increment => write!(f, "+"),
            Self::Decrement => write!(f, "-"),
            Self::MoveRight => write!(f, ">"),
            Self::MoveLeft => write!(f, "<"),
            Self::Loop(statements) => {
                write!(f, "[")?;
                for statement in statements {
                    write!(f, "{statement}")?;
                }
                write!(f, "]")?;

                Ok(())
            }
            Self::Input => write!(f, ","),
            Self::Output => write!(f, "."),
        }
    }
}

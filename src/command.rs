use std::{fmt, str::FromStr};

pub enum Command {
    Quit,
    ForceQuit,
}

impl FromStr for Command {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "q" => Ok(Self::Quit),
            "Q" => Ok(Self::ForceQuit),
            _ => Err(ParseError::UnspecificSyntaxErrorTodoRemoveThis),
        }
    }
}

pub enum ParseError {
    UnspecificSyntaxErrorTodoRemoveThis,
}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::UnspecificSyntaxErrorTodoRemoveThis => {
                f.write_str("syntax error")
            }
        }
    }
}

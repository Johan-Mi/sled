use std::{fmt, str::FromStr};

pub enum Command {
    Quit { force: bool },
    Write { quit: bool },
    Print { numbered: bool },
    Info,
    Append,
}

impl FromStr for Command {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "" => Err(ParseError::UnexpectedEndOfCommand),
            "q" => Ok(Self::Quit { force: false }),
            "Q" => Ok(Self::Quit { force: true }),
            "w" => Ok(Self::Write { quit: false }),
            "wq" => Ok(Self::Write { quit: true }),
            "p" => Ok(Self::Print { numbered: false }),
            "n" => Ok(Self::Print { numbered: true }),
            "?" => Ok(Self::Info),
            "a" => Ok(Self::Append),
            _ => Err(ParseError::UnexpectedCharacter),
        }
    }
}

pub enum ParseError {
    UnexpectedEndOfCommand,
    UnexpectedCharacter,
}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::UnexpectedEndOfCommand => {
                f.write_str("unexpected end of command")
            }
            Self::UnexpectedCharacter => {
                f.write_str("unexpected character")
            }
        }
    }
}

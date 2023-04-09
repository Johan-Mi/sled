use std::{fmt, num::IntErrorKind, ops::RangeInclusive, str::FromStr};

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
        let (s, location) = location(s)?;
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

#[derive(Clone, Copy)]
pub enum Address {
    Absolute(usize),
    Relative(isize),
    Last,
}

fn address(s: &str) -> Result<(&str, Option<Address>), ParseError> {
    Ok(if s.starts_with(|c: char| c.is_ascii_digit()) {
        let (digits, s) = split_leading_digits(s);
        let line_number =
            digits.parse().map_err(|_| ParseError::AddressOutOfBounds)?;
        (s, Some(Address::Absolute(line_number)))
    } else if let Some(s) = s.strip_prefix('+') {
        // TODO: repeated `+`
        let (digits, s) = split_leading_digits(s);
        let offset =
            digits.parse::<isize>().or_else(|err| match err.kind() {
                IntErrorKind::Empty => Ok(1),
                _ => Err(ParseError::AddressOutOfBounds),
            })?;
        (s, Some(Address::Relative(offset)))
    } else if let Some(s) = s.strip_prefix('-') {
        // TODO: repeated `-`
        let (digits, s) = split_leading_digits(s);
        let offset =
            digits.parse::<isize>().or_else(|err| match err.kind() {
                IntErrorKind::Empty => Ok(1),
                _ => Err(ParseError::AddressOutOfBounds),
            })?;
        (s, Some(Address::Relative(-offset)))
    } else if let Some(s) = s.strip_prefix('.') {
        (s, Some(Address::Relative(0)))
    } else if let Some(s) = s.strip_prefix('$') {
        (s, Some(Address::Last))
    } else if s.starts_with(['/', '?']) {
        // TODO: regex
        return Err(ParseError::RegexNotSupportedYet);
    } else {
        (s, None)
    })
}

fn split_leading_digits(s: &str) -> (&str, &str) {
    let first_non_digit =
        s.find(|c: char| !c.is_ascii_digit()).unwrap_or(s.len());
    s.split_at(first_non_digit)
}

pub enum Location {
    None,
    Single(Address),
    Range(RangeSeparator, RangeInclusive<Address>),
}

fn location(s: &str) -> Result<(&str, Location), ParseError> {
    let (s, start) = address(s)?;
    let ((s, end), separator) = if let Some(s) = s.strip_prefix(',') {
        (address(s)?, RangeSeparator::Comma)
    } else if let Some(s) = s.strip_prefix(';') {
        (address(s)?, RangeSeparator::Semicolon)
    } else {
        return Ok((s, start.map_or(Location::None, Location::Single)));
    };
    Ok((
        s,
        match (start, end, separator) {
            (None, Some(end), RangeSeparator::Semicolon) => Location::Range(
                RangeSeparator::Semicolon,
                Address::Relative(0)..=end,
            ),
            (None, end, _) => Location::Range(
                RangeSeparator::Comma,
                Address::Absolute(1)..=end.unwrap_or(Address::Last),
            ),
            (Some(start), Some(end), _) => {
                Location::Range(separator, start..=end)
            }
            (Some(start), None, _) => Location::Range(
                RangeSeparator::Semicolon,
                start..=Address::Relative(0),
            ),
        },
    ))
}

#[derive(Clone, Copy)]
pub enum RangeSeparator {
    Comma,
    Semicolon,
}

pub enum ParseError {
    UnexpectedEndOfCommand,
    UnexpectedCharacter,
    AddressOutOfBounds,
    RegexNotSupportedYet,
}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::UnexpectedEndOfCommand => {
                f.write_str("unexpected end of command")
            }
            Self::UnexpectedCharacter => f.write_str("unexpected character"),
            Self::AddressOutOfBounds => f.write_str("address out of bounds"),
            Self::RegexNotSupportedYet => {
                f.write_str("regular expressions are not supported yet")
            }
        }
    }
}

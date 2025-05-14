use std::fmt;

#[derive(Debug, Clone)]
pub enum ParseError {
    UnexpectedToken(String),
    ExpectedToken(String),
    ExpectedIdentifier(String),
    ExpectedType(String),
    ExpectedKeyword(String),
    ExpectedNumber(String),
    UnexpectedEndOfInput(String),
    InvalidInput(String),
}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ParseError::UnexpectedToken(msg) => write!(f, "Unexpected token: {}", msg),
            ParseError::ExpectedToken(msg) => write!(f, "Expected token: {}", msg),
            ParseError::ExpectedIdentifier(msg) => write!(f, "Expected identifier: {}", msg),
            ParseError::ExpectedType(msg) => write!(f, "Expected type: {}", msg),
            ParseError::ExpectedKeyword(msg) => write!(f, "Expected keyword: {}", msg),
            ParseError::ExpectedNumber(msg) => write!(f, "Expected number: {}", msg),
            ParseError::UnexpectedEndOfInput(msg) => write!(f, "Unexpected end of input: {}", msg),
            ParseError::InvalidInput(msg) => write!(f, "Invalid input: {}", msg),
        }
    }
}

use crate::ast::{Keyword, Token};
use crate::error::ParseError;
use std::str::FromStr;

/// Tokenizer struct
pub struct Tokenizer {
    input: Vec<char>,
    position: usize,
    tokens: Vec<Token>, // Store tokens separately
}

impl Tokenizer {
    /// Creates a new tokenizer and tokenizes the entire input
    pub fn new(input: &str) -> Self {
        let mut tokenizer = Tokenizer {
            input: input.chars().collect(),
            position: 0,
            tokens: vec![],
        };
        tokenizer.tokenize_input(); // Tokenize once on initialization
        tokenizer
    }

    /// Tokenizes the entire input and returns the tokens
    pub fn tokenize_string(&mut self) -> Result<Vec<Token>, ParseError> {
        self.tokenize_input();  // Ensure input is fully tokenized
        Ok(self.tokens.clone())
    }

    /// Tokenizes the entire input into the internal tokens vector
    fn tokenize_input(&mut self) {
        while let Some(token) = self.tokenize_next_token() {
            match token {
                Ok(token) => self.tokens.push(token),
                Err(e) => {
                    eprintln!("Tokenizer error: {:?}", e);
                    self.tokens.push(Token::Eof);
                    break;
                }
            }
        }

        // Add the Eof token at the end if not already present
        if self.tokens.is_empty() || self.tokens.last() != Some(&Token::Eof) {
            self.tokens.push(Token::Eof);
        }
    }

    /// Returns the next character without advancing the position
    fn peek(&self) -> Option<char> {
        self.input.get(self.position).copied()
    }

    /// Returns the next character and advances the position
    fn advance(&mut self) -> Option<char> {
        if self.position < self.input.len() {
            let ch = self.input[self.position];
            self.position += 1;
            Some(ch)
        } else {
            None
        }
    }

    /// Tokenizes the next available token
    fn tokenize_next_token(&mut self) -> Option<Result<Token, ParseError>> {
        while let Some(ch) = self.peek() {
            match ch {
                // Skip whitespace
                ' ' | '\t' | '\n' | '\r' => {
                    self.advance();
                }

                // String literals
                '"' => return Some(self.tokenize_string_literal()),

                // Numbers
                '0'..='9' => return Some(self.tokenize_number()),

                // Identifiers or keywords
                'a'..='z' | 'A'..='Z' | '_' => return Some(self.tokenize_identifier_or_keyword()),

                // Single-character tokens
                '(' => {
                    self.advance();
                    return Some(Ok(Token::LeftParentheses));
                }
                ')' => {
                    self.advance();
                    return Some(Ok(Token::RightParentheses));
                }
                ',' => {
                    self.advance();
                    return Some(Ok(Token::Comma));
                }
                ';' => {
                    self.advance();
                    return Some(Ok(Token::Semicolon));
                }

                // Multi-character operators
                '=' => {
                    self.advance();
                    if self.peek() == Some('=') {
                        self.advance();
                        return Some(Ok(Token::Equal));
                    }
                    return Some(Ok(Token::Equal));
                }
                '!' => {
                    self.advance();
                    if self.peek() == Some('=') {
                        self.advance();
                        return Some(Ok(Token::NotEqual));
                    }
                    return Some(Err(ParseError::UnexpectedToken("Unexpected '!' without '='".to_string())));
                }
                '>' => {
                    self.advance();
                    if self.peek() == Some('=') {
                        self.advance();
                        return Some(Ok(Token::GreaterThanOrEqual));
                    }
                    return Some(Ok(Token::GreaterThan));
                }
                '<' => {
                    self.advance();
                    if self.peek() == Some('=') {
                        self.advance();
                        return Some(Ok(Token::LessThanOrEqual));
                    }
                    return Some(Ok(Token::LessThan));
                }

                // Single-character operators
                '+' => {
                    self.advance();
                    return Some(Ok(Token::Plus));
                }
                '-' => {
                    self.advance();
                    return Some(Ok(Token::Minus));
                }
                '*' => {
                    self.advance();
                    return Some(Ok(Token::Multiply));
                }
                '/' => {
                    self.advance();
                    return Some(Ok(Token::Divide));
                }

                // Unknown character
                _ => {
                    let invalid_char = self.advance().unwrap();
                    return Some(Err(ParseError::UnexpectedToken(format!("Unexpected character '{}'", invalid_char))));
                }
            }
        }

        // Return Eof if no more characters
        Some(Ok(Token::Eof))
    }

    /// Tokenizes string literals
    fn tokenize_string_literal(&mut self) -> Result<Token, ParseError> {
        let mut value = String::new();
        self.advance(); // Skip the opening quote

        while let Some(ch) = self.peek() {
            match ch {
                '"' => {
                    self.advance(); // Consume the closing quote
                    return Ok(Token::String(value));
                }
                _ => value.push(self.advance().unwrap()),
            }
        }

        Err(ParseError::UnexpectedEndOfInput("Unterminated string literal".to_string()))
    }

    /// Tokenizes numbers (u64 only)
    fn tokenize_number(&mut self) -> Result<Token, ParseError> {
        let mut value = String::new();

        while let Some(ch) = self.peek() {
            if ch.is_ascii_digit() {
                value.push(self.advance().unwrap());
            } else {
                break;
            }
        }

        match value.parse::<u64>() {
            Ok(num) => Ok(Token::Number(num)),
            Err(_) => Err(ParseError::ExpectedNumber(format!("Invalid number: {}", value))),
        }
    }

    /// Tokenizes identifiers or keywords
    fn tokenize_identifier_or_keyword(&mut self) -> Result<Token, ParseError> {
        let mut value = String::new();

        while let Some(ch) = self.peek() {
            if ch.is_ascii_alphanumeric() || ch == '_' {
                value.push(self.advance().unwrap());
            } else {
                break;
            }
        }

        // Check if the value is a known keyword
        match Keyword::from_str(&value) {
            Ok(keyword) => Ok(Token::Keyword(keyword)),
            Err(_) => Ok(Token::Identifier(value)),
        }
    }

    /// Returns the next token without advancing the position
    pub fn peek_token(&self) -> Option<Token> {
        self.tokens.get(self.position).cloned()
    }

    /// Returns the next token and advances the position
    pub fn next(&mut self) -> Option<Token> {
        if self.position < self.tokens.len() {
            let token = self.tokens[self.position].clone();
            self.position += 1;
            Some(token)
        } else {
            Some(Token::Eof)
        }
    }
}

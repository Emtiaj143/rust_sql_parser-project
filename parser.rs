use crate::ast::{Expression, Token, BinaryOperator};
use crate::error::ParseError;
use crate::tokenizer::Tokenizer;

pub struct PrattParser {
    tokenizer: Tokenizer,
    current_token: Option<Token>,
}

impl PrattParser {
    pub fn new(input: &str) -> Self {
        let mut tokenizer = Tokenizer::new(input);
        let first_token = tokenizer.next();
        PrattParser {
            tokenizer,
            current_token: first_token,
        }
    }

    pub fn parse(&mut self) -> Result<Expression, ParseError> {
        self.parse_expression(0)
    }

    fn advance(&mut self) -> Result<(), ParseError> {
        self.current_token = self.tokenizer.next();
        Ok(())
    }

    fn parse_expression(&mut self, precedence: u8) -> Result<Expression, ParseError> {
        let mut left = self.parse_primary()?;

        while let Some(token) = &self.current_token {
            let token_precedence = self.get_precedence(token);

            if token_precedence <= precedence {
                break;
            }

            let op = self.current_token.clone();
            self.advance()?;
            let right = self.parse_expression(token_precedence)?;

            match op {
                Some(Token::Plus) => {
                    left = Expression::BinaryOperation {
                        left_operand: Box::new(left),
                        operator: BinaryOperator::Plus,
                        right_operand: Box::new(right),
                    };
                }
                Some(Token::Minus) => {
                    left = Expression::BinaryOperation {
                        left_operand: Box::new(left),
                        operator: BinaryOperator::Minus,
                        right_operand: Box::new(right),
                    };
                }
                Some(Token::Multiply) => {
                    left = Expression::BinaryOperation {
                        left_operand: Box::new(left),
                        operator: BinaryOperator::Multiply,
                        right_operand: Box::new(right),
                    };
                }
                Some(Token::Divide) => {
                    left = Expression::BinaryOperation {
                        left_operand: Box::new(left),
                        operator: BinaryOperator::Divide,
                        right_operand: Box::new(right),
                    };
                }
                _ => return Err(ParseError::InvalidInput("Unexpected operator".into())),
            }
        }

        Ok(left)
    }

    fn parse_primary(&mut self) -> Result<Expression, ParseError> {
        match self.current_token.clone() {
            Some(Token::Number(n)) => {
                self.advance()?;
                Ok(Expression::Number(n))
            }
            Some(Token::Identifier(s)) => {
                self.advance()?;
                Ok(Expression::Identifier(s))
            }
            Some(Token::LeftParentheses) => {
                self.advance()?;
                let expr = self.parse_expression(0)?;
                if let Some(Token::RightParentheses) = self.current_token {
                    self.advance()?;
                    Ok(expr)
                } else {
                    Err(ParseError::InvalidInput("Expected closing parenthesis".into()))
                }
            }
            Some(t) => Err(ParseError::InvalidInput(format!("Unexpected token: {:?}", t))),
            None => Err(ParseError::InvalidInput("Unexpected end of input".into())),
        }
    }

    fn get_precedence(&self, token: &Token) -> u8 {
        match token {
            Token::Plus | Token::Minus => 1,
            Token::Multiply | Token::Divide => 2,
            _ => 0,
        }
    }
}

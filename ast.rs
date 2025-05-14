use std::str::FromStr;

/// Represents SQL keywords like SELECT, CREATE, WHERE, etc.
#[derive(Debug, PartialEq, Clone)]
pub enum Keyword {
    Select,
    Create,
    Table,
    Where,
    From,
    Order,
    By,
    And,
    Or,
    Not,
}

impl FromStr for Keyword {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_uppercase().as_str() {
            "SELECT" => Ok(Keyword::Select),
            "CREATE" => Ok(Keyword::Create),
            "TABLE" => Ok(Keyword::Table),
            "WHERE" => Ok(Keyword::Where),
            "FROM" => Ok(Keyword::From),
            "ORDER" => Ok(Keyword::Order),
            "BY" => Ok(Keyword::By),
            "AND" => Ok(Keyword::And),
            "OR" => Ok(Keyword::Or),
            "NOT" => Ok(Keyword::Not),
            _ => Err(()),
        }
    }
}

/// Represents all possible token types in the SQL language
#[derive(Debug, PartialEq, Clone)]
pub enum Token {
    Keyword(Keyword),
    Identifier(String),
    String(String),
    Number(u64),
    Invalid(char),

    // Punctuation
    RightParentheses,
    LeftParentheses,
    Comma,
    Semicolon,

    // Comparison Operators
    GreaterThan,
    GreaterThanOrEqual,
    LessThan,
    LessThanOrEqual,
    Equal,
    NotEqual,

    // Arithmetic Operators
    Multiply,
    Divide,
    Minus,
    Plus,

    // Special Tokens
    Eof,
}

/// Represents binary operators for mathematical and logical operations
#[derive(Debug, PartialEq, Clone)]
pub enum BinaryOperator {
    Plus,
    Minus,
    Multiply,
    Divide,
}

/// Represents SQL expressions
#[derive(Debug, PartialEq, Clone)]
pub enum Expression {
    BinaryOperation {
        left_operand: Box<Expression>,
        operator: BinaryOperator,
        right_operand: Box<Expression>,
    },
    Number(u64),
    Identifier(String),
}

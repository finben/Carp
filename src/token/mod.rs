#[derive(Debug, PartialEq)]
pub enum TokenType {
    NumericLiteral,
    Identifier,
    Slash,
    Star,
    Bang,
    Equal,
    EqualEqual,
    BangEqual,
    Plus,
    PlusEqual,
    Greater,
    GreaterEqual,
    Less,
    LessEqual,
    LeftParen,
    RightParen,
    LeftBracket,
    LeftCurly,
    RightCurly,
    RightBracket,
    Unknown,
    EOF,
}

#[derive(Debug, PartialEq)]
pub struct Token {
    pub token_type: TokenType,
    pub lexeme: Option<String>,
}

impl Default for Token {
    fn default() -> Self {
        Self {
            token_type: TokenType::Unknown,
            lexeme: None,
        }
    }
}

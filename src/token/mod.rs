#[derive(Debug)]
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
    EOF,
}

#[derive(Debug)]
pub struct Token {
    pub token_type: TokenType,
    pub lexeme: String,
}

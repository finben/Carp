extern crate carp;
use carp::token::Token;
use carp::token::TokenType;
use carp::token::TokenType::*;

pub fn token_gen(token_type: TokenType, lexeme: Option<&str>) -> Token {
    Token {
        token_type,
        lexeme: match lexeme {
            Some(lex) => Some(lex.to_string()),
            None => None,
        },
    }
}

pub fn tokenize(source: &str) -> Vec<Token> {
    let mut position = 0;
    let mut tokens = Vec::new();
    while position < source.len() {
        let current = source.chars().nth(position).unwrap();
        match current {
            //Single Lexemes
            '(' => tokens.push(token_gen(LeftParen, None)),
            ')' => tokens.push(token_gen(RightParen, None)),
            '[' => tokens.push(token_gen(LeftBracket, None)),
            ']' => tokens.push(token_gen(RightBracket, None)),
            '{' => tokens.push(token_gen(LeftCurly, None)),
            '}' => tokens.push(token_gen(RightCurly, None)),
            '*' => tokens.push(token_gen(Star, None)),
            '+' => tokens.push(token_gen(Plus, None)),
            //Possible Doubles
            '>' => {
                position += 1;
                if position < source.len() {
                    let next_char = source.chars().nth(position).unwrap();
                    if next_char == '=' {
                        tokens.push(token_gen(GreaterEqual, None));
                        position += 1;
                        continue;
                    } else {
                        tokens.push(token_gen(Greater, None));
                    }
                }
                continue;
            }
            '<' => {
                position += 1;
                let next_char = source.chars().nth(position).unwrap();
                if next_char == '=' {
                    tokens.push(token_gen(LessEqual, None));
                    position += 1;
                } else {
                    tokens.push(token_gen(Less, None));
                }
                continue;
            }
            '=' => {
                position += 1;
                let next_char = source.chars().nth(position).unwrap();
                if next_char == '=' {
                    tokens.push(token_gen(EqualEqual, None));
                    position += 1;
                } else {
                    tokens.push(token_gen(Equal, None));
                }
                continue;
            }
            '!' => {
                position += 1;
                let next_char = source.chars().nth(position).unwrap();
                if next_char == '=' {
                    tokens.push(token_gen(BangEqual, None));
                    position += 1;
                } else {
                    tokens.push(token_gen(Bang, None));
                }
                continue;
            }
            // NumericalLiterals
            x if x.is_digit(10) => {
                let mut numerical_string = x.to_string();
                position += 1;
                while position < source.len() {
                    let next_char = source.chars().nth(position).unwrap();
                    if next_char.is_digit(10) || next_char == '.' {
                        numerical_string.push(next_char);
                        position += 1;
                    } else {
                        break;
                    }
                }
                tokens.push(token_gen(NumericLiteral, Some(&numerical_string)));
            }
            // New Lines and Spaces
            ' ' | '\n' => {}
            // String Literals
            c => {
                let mut string_literal = c.to_string();
                position += 1;
                while position < source.len() {
                    let next_char = source.chars().nth(position).unwrap();
                    if next_char.is_alphanumeric() || next_char == '_' {
                        string_literal.push(next_char);
                        position += 1;
                        continue;
                    } else {
                        break;
                    }
                }
                tokens.push(token_gen(Identifier, Some(&string_literal)));
                continue;
            }
        };
        position += 1;
        //Dummy Token
    }
    // DUMMY EOF Token
    tokens.push(token_gen(EOF, None));
    return tokens;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn handle_one_char_tokens() {
        let source = "((  )){}";
        let tokens = tokenize(source);
        assert_eq!(tokens[0].token_type, LeftParen);
        assert_eq!(tokens[1].token_type, LeftParen);
        assert_eq!(tokens[2].token_type, RightParen);
        assert_eq!(tokens[3].token_type, RightParen);
        assert_eq!(tokens[4].token_type, LeftCurly);
        assert_eq!(tokens[5].token_type, RightCurly);
        assert_eq!(tokens[6].token_type, EOF);
    }

    #[test]
    fn handle_two_char_tokens() {
        let source = "! != > >= < <= = ==";
        let tokens = tokenize(source);
        assert_eq!(tokens[0].token_type, Bang);
        assert_eq!(tokens[1].token_type, BangEqual);
        assert_eq!(tokens[2].token_type, Greater);
        assert_eq!(tokens[3].token_type, GreaterEqual);
    }

    #[test]
    fn handle_number_lexeme() {
        let source = "123 456 2.0";
        let tokens = tokenize(source);
        assert_eq!(tokens[0].lexeme, Some("123".to_string()));
        assert_eq!(tokens[0].token_type, NumericLiteral);
        assert_eq!(tokens[1].lexeme, Some("456".to_string()));
        assert_eq!(tokens[2].lexeme, Some("2.0".to_string()));
    }

    #[test]
    fn handle_identifier_lexeme() {
        let source = "been there_magi magicarpy";
        let tokens = tokenize(source);
        assert_eq!(tokens[0].lexeme, Some("been".to_string()));
        assert_eq!(tokens[0].token_type, Identifier);
        assert_eq!(tokens[1].lexeme, Some("there_magi".to_string()));
        assert_eq!(tokens[2].lexeme, Some("magicarpy".to_string()));
    }
}

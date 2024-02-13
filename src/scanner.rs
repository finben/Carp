extern crate carp;
use carp::token::Token;
use carp::token::TokenType;
use carp::token::TokenType::*;

pub fn token_gen(token_type: TokenType, lexeme: &str) -> Token {
    Token {
        token_type,
        lexeme: lexeme.to_string(),
    }
}

pub fn tokenize(source: &str) -> Vec<Token> {
    let mut position = 0;
    let mut tokens = Vec::new();
    while position < source.len() {
        let current = source.chars().nth(position).unwrap();
        match current {
            //Single Lexemes
            '(' => tokens.push(token_gen(LeftParen, "(")),
            ')' => tokens.push(token_gen(RightParen, ")")),
            '*' => tokens.push(token_gen(Star, "*")),
            '+' => tokens.push(token_gen(Plus, "+")),

            //Possible Doubles
            '>' => {
                position += 1;
                if position < source.len() {
                    let next_char = source.chars().nth(position).unwrap();
                    if next_char == '=' {
                        tokens.push(token_gen(GreaterEqual, "=="));
                        position += 1;
                        continue;
                    } else {
                        tokens.push(token_gen(Greater, ">"));
                    }
                }
                continue;
            }

            '<' => {
                position += 1;
                let next_char = source.chars().nth(position).unwrap();
                if next_char == '=' {
                    tokens.push(token_gen(LessEqual, "<="));
                    position += 1;
                } else {
                    tokens.push(token_gen(Less, "<"));
                }
                continue;
            }
            '=' => {
                position += 1;
                let next_char = source.chars().nth(position).unwrap();
                if next_char == '=' {
                    tokens.push(token_gen(EqualEqual, "<="));
                    position += 1;
                } else {
                    tokens.push(token_gen(Equal, "="));
                }
                continue;
            }

            // NumericalLiterals
            x if x.is_digit(10) => {
                let mut numberical_string = x.to_string();

                tokens.push(token_gen(NumericLiteral, &numberical_string));
            } // String Literals
            ' ' | '\n' => {}
            c => {
                let mut string_literal = c.to_string();
                position += 1;
                while position < source.len() {
                    let next_char = source.chars().nth(position).unwrap();
                    if next_char.is_alphanumeric() {
                        string_literal.push(next_char);
                        position += 1;
                        continue;
                    } else {
                        break;
                    }
                }

                tokens.push(token_gen(Identifier, &string_literal));
                continue;
            }
        };
        position += 1;

        //Dummy Token
    }
    // DUMMY EOF Token
    tokens.push(Token {
        token_type: Identifier,
        lexeme: "EOF".to_string(),
    });
    return tokens;
}

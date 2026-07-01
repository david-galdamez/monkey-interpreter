use std::{char, iter::Peekable, str::Chars};

use crate::token;

struct Lexer<'a> {
    input: &'a str,
    input_iter: Peekable<Chars<'a>>,
    position: usize,
    ch: char,
}

fn new_token(token_type: token::TokenType, ch: char) -> token::Token {
    token::Token {
        token_type,
        literal: ch.to_string(),
    }
}

impl<'a> Lexer<'a> {
    pub fn new(input: &'a str) -> Lexer<'a> {
        let mut lexer = Lexer {
            input,
            input_iter: input.chars().peekable(),
            position: 0,
            ch: char::default(),
        };
        lexer.read_char();
        lexer
    }

    pub fn read_char(&mut self) {
        if self.input_iter.peek().is_none() {
            self.ch = char::default();
        } else {
            self.ch = self.input_iter.next().unwrap_or_default();
            self.position += 1;
        }
    }

    pub fn read_identifier(&mut self) -> String {
        let position = self.position;
        while self.ch.is_alphabetic() {
            self.read_char();
        }
        self.input[position..self.position].to_string()
    }

    pub fn next_token(&mut self) -> token::Token {
        let mut token = token::Token::default();

        match self.ch {
            '=' => token = new_token(token::ASSIGN, self.ch),
            ';' => token = new_token(token::SEMICOLON, self.ch),
            '(' => token = new_token(token::LPAREN, self.ch),
            ')' => token = new_token(token::RPAREN, self.ch),
            ',' => token = new_token(token::COMMA, self.ch),
            '+' => token = new_token(token::PLUS, self.ch),
            '{' => token = new_token(token::LBRACE, self.ch),
            '}' => token = new_token(token::RBRACE, self.ch),
            '\x00' => {
                token.literal = char::default().to_string();
                token.token_type = token::EOF;
            }
            other => {
                if other.is_alphabetic() {
                    token.literal = self.read_identifier();
                    return token;
                } else {
                    token = new_token(token::ILLEGAL, other)
                }
            }
        };
        self.read_char();
        token
    }
}

#[cfg(test)]
mod tests {

    use crate::{lexer::Lexer, token};

    struct Expected {
        expected_type: token::TokenType,
        expected_literal: String,
    }

    #[test]
    fn test_next_token() {
        let input = " let five = 5;
        let ten = 10;

        let add = fn(x,y) {
            x + y;
        };

        let result = add(five, ten);
            ";

        let tests = vec![
            Expected {
                expected_type: token::LET,
                expected_literal: "let".to_string(),
            },
            Expected {
                expected_type: token::IDENT,
                expected_literal: "five".to_string(),
            },
            Expected {
                expected_type: token::ASSIGN,
                expected_literal: "=".to_string(),
            },
            Expected {
                expected_type: token::INT,
                expected_literal: "5".to_string(),
            },
            Expected {
                expected_type: token::SEMICOLON,
                expected_literal: ";".to_string(),
            },
            Expected {
                expected_type: token::LET,
                expected_literal: "let".to_string(),
            },
            Expected {
                expected_type: token::IDENT,
                expected_literal: "ten".to_string(),
            },
            Expected {
                expected_type: token::ASSIGN,
                expected_literal: "=".to_string(),
            },
            Expected {
                expected_type: token::INT,
                expected_literal: "10".to_string(),
            },
            Expected {
                expected_type: token::SEMICOLON,
                expected_literal: ";".to_string(),
            },
            Expected {
                expected_type: token::LET,
                expected_literal: "let".to_string(),
            },
            Expected {
                expected_type: token::IDENT,
                expected_literal: "add".to_string(),
            },
            Expected {
                expected_type: token::ASSIGN,
                expected_literal: "=".to_string(),
            },
            Expected {
                expected_type: token::FUNCTION,
                expected_literal: "fn".to_string(),
            },
            Expected {
                expected_type: token::LPAREN,
                expected_literal: "(".to_string(),
            },
            Expected {
                expected_type: token::IDENT,
                expected_literal: "x".to_string(),
            },
            Expected {
                expected_type: token::COMMA,
                expected_literal: ",".to_string(),
            },
            Expected {
                expected_type: token::IDENT,
                expected_literal: "y".to_string(),
            },
            Expected {
                expected_type: token::RPAREN,
                expected_literal: ")".to_string(),
            },
            Expected {
                expected_type: token::RBRACE,
                expected_literal: "{".to_string(),
            },
            Expected {
                expected_type: token::IDENT,
                expected_literal: "x".to_string(),
            },
            Expected {
                expected_type: token::PLUS,
                expected_literal: "+".to_string(),
            },
            Expected {
                expected_type: token::IDENT,
                expected_literal: "y".to_string(),
            },
            Expected {
                expected_type: token::SEMICOLON,
                expected_literal: ";".to_string(),
            },
            Expected {
                expected_type: token::LBRACE,
                expected_literal: "}".to_string(),
            },
            Expected {
                expected_type: token::SEMICOLON,
                expected_literal: ";".to_string(),
            },
            Expected {
                expected_type: token::LET,
                expected_literal: "let".to_string(),
            },
            Expected {
                expected_type: token::IDENT,
                expected_literal: "result".to_string(),
            },
            Expected {
                expected_type: token::ASSIGN,
                expected_literal: "=".to_string(),
            },
            Expected {
                expected_type: token::IDENT,
                expected_literal: "add".to_string(),
            },
            Expected {
                expected_type: token::LPAREN,
                expected_literal: "(".to_string(),
            },
            Expected {
                expected_type: token::IDENT,
                expected_literal: "five".to_string(),
            },
            Expected {
                expected_type: token::COMMA,
                expected_literal: ",".to_string(),
            },
            Expected {
                expected_type: token::IDENT,
                expected_literal: "ten".to_string(),
            },
            Expected {
                expected_type: token::RPAREN,
                expected_literal: ")".to_string(),
            },
            Expected {
                expected_type: token::SEMICOLON,
                expected_literal: ";".to_string(),
            },
            Expected {
                expected_type: token::EOF,
                expected_literal: '\x00'.to_string(),
            },
        ];

        let mut lexer = Lexer::new(input);
        for test_token in tests.iter() {
            let token = lexer.next_token();

            assert_eq!(token.token_type, test_token.expected_type);
            assert_eq!(token.literal, test_token.expected_literal);
        }
    }
}

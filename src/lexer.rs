use std::{char, iter::Peekable, str::Chars};

use crate::token;

#[derive(Debug)]
pub struct Lexer<'a> {
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

    // we inspect if there is a valid character so we can consume it
    // and go forward
    pub fn read_char(&mut self) {
        if self.input_iter.peek().is_none() {
            self.ch = char::default();
        } else {
            self.ch = self.input_iter.next().unwrap_or_default();
        }
        self.position += 1;
    }

    pub fn peek_char(&mut self) -> String {
        match self.input_iter.peek() {
            Some(ch) => ch.to_string(),
            None => char::default().to_string(),
        }
    }

    // we use the - 1 because the position starts at 0 but the first read_char()
    // advances forward
    fn read_identifier(&mut self) -> String {
        let position = self.position - 1;
        while self.ch.is_alphabetic() || self.ch == '_' {
            self.read_char();
        }
        self.input[position..self.position - 1].trim().to_string()
    }

    //same function as the one above
    fn read_number(&mut self) -> String {
        let position = self.position - 1;
        while self.ch.is_ascii_digit() {
            self.read_char();
        }
        self.input[position..self.position - 1].trim().to_string()
    }

    fn read_string(&mut self) -> String {
        let position = self.position;
        loop {
            self.read_char();
            if self.ch == '"' || self.ch == '\x00' {
                break;
            }
        }
        self.input[position..self.position - 1].to_string()
    }

    fn skip_whitespace(&mut self) {
        // important to do a while so we can skip all the
        // whitespace
        while self.ch.is_whitespace() {
            self.read_char();
        }
    }

    pub fn next_token(&mut self) -> token::Token {
        let mut token = token::Token::default();

        self.skip_whitespace();

        match self.ch {
            '=' => {
                if self.peek_char() == "=" {
                    let ch = self.ch;
                    self.read_char();
                    token = token::Token {
                        token_type: token::EQ,
                        literal: format!("{}{}", ch, self.ch),
                    }
                } else {
                    token = new_token(token::ASSIGN, self.ch);
                }
            }
            '+' => token = new_token(token::PLUS, self.ch),
            '-' => token = new_token(token::MINUS, self.ch),
            '!' => {
                if self.peek_char() == "=" {
                    let ch = self.ch;
                    self.read_char();
                    token = token::Token {
                        token_type: token::NOT_EQ,
                        literal: format!("{}{}", ch, self.ch),
                    }
                } else {
                    token = new_token(token::BANG, self.ch);
                }
            }
            '/' => token = new_token(token::SLASH, self.ch),
            '*' => token = new_token(token::ASTERISK, self.ch),
            '<' => token = new_token(token::LT, self.ch),
            '>' => token = new_token(token::GT, self.ch),
            ';' => token = new_token(token::SEMICOLON, self.ch),
            '(' => token = new_token(token::LPAREN, self.ch),
            ')' => token = new_token(token::RPAREN, self.ch),
            ',' => token = new_token(token::COMMA, self.ch),
            ':' => token = new_token(token::COLON, self.ch),
            '{' => token = new_token(token::LBRACE, self.ch),
            '}' => token = new_token(token::RBRACE, self.ch),
            '[' => token = new_token(token::LBRACKET, self.ch),
            ']' => token = new_token(token::RBRACKET, self.ch),
            '"' => {
                token.token_type = token::STRING;
                token.literal = self.read_string();
            }
            '\x00' => {
                token.literal = char::default().to_string();
                token.token_type = token::EOF;
            }
            other => {
                if other.is_alphabetic() || other == '_' {
                    token.literal = self.read_identifier();
                    token.token_type = token::lookup_ident(&token.literal);
                    return token;
                } else if other.is_ascii_digit() {
                    token.token_type = token::INT;
                    token.literal = self.read_number();
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
        let input = "let five = 5;
        let ten = 10;

        let add = fn(x,y) {
            x + y;
        };

        let result = add(five, ten);
        !-/*5;
        5 < 10 > 5;

        if (5 < 10) {
            return true;
        } else {
            return false;
        }

        10 == 10;
        10 != 9;
        \"foobar\"
        \"foo bar\"
        [1, 2];
        {\"foo\": \"bar\"}
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
                expected_type: token::LBRACE,
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
                expected_type: token::RBRACE,
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
                expected_type: token::BANG,
                expected_literal: "!".to_string(),
            },
            Expected {
                expected_type: token::MINUS,
                expected_literal: "-".to_string(),
            },
            Expected {
                expected_type: token::SLASH,
                expected_literal: "/".to_string(),
            },
            Expected {
                expected_type: token::ASTERISK,
                expected_literal: "*".to_string(),
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
                expected_type: token::INT,
                expected_literal: "5".to_string(),
            },
            Expected {
                expected_type: token::LT,
                expected_literal: "<".to_string(),
            },
            Expected {
                expected_type: token::INT,
                expected_literal: "10".to_string(),
            },
            Expected {
                expected_type: token::GT,
                expected_literal: ">".to_string(),
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
                expected_type: token::IF,
                expected_literal: "if".to_string(),
            },
            Expected {
                expected_type: token::LPAREN,
                expected_literal: "(".to_string(),
            },
            Expected {
                expected_type: token::INT,
                expected_literal: "5".to_string(),
            },
            Expected {
                expected_type: token::LT,
                expected_literal: "<".to_string(),
            },
            Expected {
                expected_type: token::INT,
                expected_literal: "10".to_string(),
            },
            Expected {
                expected_type: token::RPAREN,
                expected_literal: ")".to_string(),
            },
            Expected {
                expected_type: token::LBRACE,
                expected_literal: "{".to_string(),
            },
            Expected {
                expected_type: token::RETURN,
                expected_literal: "return".to_string(),
            },
            Expected {
                expected_type: token::TRUE,
                expected_literal: "true".to_string(),
            },
            Expected {
                expected_type: token::SEMICOLON,
                expected_literal: ";".to_string(),
            },
            Expected {
                expected_type: token::RBRACE,
                expected_literal: "}".to_string(),
            },
            Expected {
                expected_type: token::ELSE,
                expected_literal: "else".to_string(),
            },
            Expected {
                expected_type: token::LBRACE,
                expected_literal: "{".to_string(),
            },
            Expected {
                expected_type: token::RETURN,
                expected_literal: "return".to_string(),
            },
            Expected {
                expected_type: token::FALSE,
                expected_literal: "false".to_string(),
            },
            Expected {
                expected_type: token::SEMICOLON,
                expected_literal: ";".to_string(),
            },
            Expected {
                expected_type: token::RBRACE,
                expected_literal: "}".to_string(),
            },
            Expected {
                expected_type: token::INT,
                expected_literal: "10".to_string(),
            },
            Expected {
                expected_type: token::EQ,
                expected_literal: "==".to_string(),
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
                expected_type: token::INT,
                expected_literal: "10".to_string(),
            },
            Expected {
                expected_type: token::NOT_EQ,
                expected_literal: "!=".to_string(),
            },
            Expected {
                expected_type: token::INT,
                expected_literal: "9".to_string(),
            },
            Expected {
                expected_type: token::SEMICOLON,
                expected_literal: ";".to_string(),
            },
            Expected {
                expected_type: token::STRING,
                expected_literal: "foobar".to_string(),
            },
            Expected {
                expected_type: token::STRING,
                expected_literal: "foo bar".to_string(),
            },
            Expected {
                expected_type: token::LBRACKET,
                expected_literal: "[".to_string(),
            },
            Expected {
                expected_type: token::INT,
                expected_literal: "1".to_string(),
            },
            Expected {
                expected_type: token::COMMA,
                expected_literal: ",".to_string(),
            },
            Expected {
                expected_type: token::INT,
                expected_literal: "2".to_string(),
            },
            Expected {
                expected_type: token::RBRACKET,
                expected_literal: "]".to_string(),
            },
            Expected {
                expected_type: token::SEMICOLON,
                expected_literal: ";".to_string(),
            },
            Expected {
                expected_type: token::LBRACE,
                expected_literal: "{".to_string(),
            },
            Expected {
                expected_type: token::STRING,
                expected_literal: "foo".to_string(),
            },
            Expected {
                expected_type: token::COLON,
                expected_literal: ":".to_string(),
            },
            Expected {
                expected_type: token::STRING,
                expected_literal: "bar".to_string(),
            },
            Expected {
                expected_type: token::RBRACE,
                expected_literal: "}".to_string(),
            },
            Expected {
                expected_type: token::EOF,
                expected_literal: '\x00'.to_string(),
            },
        ];

        let mut lexer = Lexer::new(input);
        for (i, test_token) in tests.iter().enumerate() {
            let token = lexer.next_token();
            assert_eq!(
                token.token_type, test_token.expected_type,
                "test[{}] token_type wrong",
                i
            );
            assert_eq!(
                token.literal, test_token.expected_literal,
                "test[{}] literal wrong",
                i
            );
        }
    }
}

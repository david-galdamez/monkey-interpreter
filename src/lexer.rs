use std::{iter::Peekable, str::Chars};

use crate::token;

struct Lexer<'a> {
    input: &'a str,
    input_iter: Peekable<Chars<'a>>,
    ch: char,
}

impl<'a> Lexer<'a> {
    pub fn new(input: &'a str) -> Lexer<'a> {
        let mut lexer = Lexer {
            input: input,
            input_iter: input.chars().peekable(),
            ch: char::default(),
        };
        lexer.read_char();
        lexer
    }

    pub fn read_char(&mut self) {
        if let None = self.input_iter.peek() {
            self.ch = char::default();
        } else {
            self.ch = self.input_iter.next().unwrap_or_default();
        }
    }

    pub fn next_token(&self) -> token::Token {
        let mut token = token::Token::default();

        match self.ch {
            '=' => token = new_token(token::ASS, ch),
        }
    }
}

fn new_token(token_type: token::TokenType, ch: char) -> token::Token {
    token::Token {
        token_type,
        literal: ch.to_string(),
    }
}

#[cfg(test)]
mod tests {

    use crate::token;

    struct Expected {
        expected_type: token::TokenType,
        expected_literal: String,
    }

    fn test_next_token() {
        let input = "=+(){},;";
    }
}

use std::mem;

use crate::{ast, lexer, token};

struct Parser<'a> {
    lex: lexer::Lexer<'a>,
    cur_token: token::Token,
    peek_token: token::Token,
}

impl<'a> Parser<'a> {
    pub fn new(lex: lexer::Lexer<'a>) -> Parser<'a> {
        let mut parser = Parser{
            lex,
            cur_token: token::Token::default(),
            peek_token: token::Token::default(),
        };

        parser.next_token();
        parser.next_token();
        parser
    }

    fn next_token(&mut self) {
        self.cur_token = mem::replace(&mut self.peek_token, self.lex.next_token());
    }

}

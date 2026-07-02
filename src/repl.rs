use std::io::{self, Write};

use crate::{lexer, token};

pub fn start() {
    loop {
        let mut input = String::new();
        print!(">> ");
        io::stdout().flush().unwrap();

        match io::stdin().read_line(&mut input) {
            Ok(_) => (),
            Err(_) => return,
        }

        let mut lex = lexer::Lexer::new(&input);

        loop {
            let tok = lex.next_token();
            if tok.token_type == token::EOF {
                break;
            }

            println!("{:#?}", tok);
        }
    }
}

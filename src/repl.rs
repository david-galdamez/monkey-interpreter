use std::io::{self, Write};

use crate::{evaluator, lexer, parser};

pub fn start() {
    loop {
        let mut input = String::new();
        print!(">> ");
        io::stdout().flush().unwrap();

        match io::stdin().read_line(&mut input) {
            Ok(_) => (),
            Err(_) => return,
        }

        let lex = lexer::Lexer::new(&input);
        let mut parser = parser::Parser::new(lex);
        let program = match parser.parse_program() {
            Some(p) => p,
            None => {
                eprintln!("error getting program");
                return;
            }
        };

        if parser.errors().len() != 0 {
            print_parser_errors(parser.errors());
            continue;
        }

        let eval = evaluator::eval(&program);
        println!("{}", eval.inspect());
    }
}

fn print_parser_errors(errors: &[String]) {
    for err in errors {
        eprintln!("\t{}", err);
    }
}

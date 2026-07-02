mod lexer;
mod token;
mod repl;
mod ast;
mod parser;

fn main() {
    println!("Hello! This is the Monkey programming language!");
    println!("Feel free to type in commands");
    repl::start();
}

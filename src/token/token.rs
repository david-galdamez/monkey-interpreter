type TokenType = String;

#[derive(Debug)]
pub struct Token {
    Type: TokenType,
    Literal: String,
}

const ILLEGAL: &'static str = "ILLEGAL";
const EOF: &'static str = "EOF";

// Identifiers + literals
const IDENT: &'static str = "IDENT";
const INT: &'static str = "INT";

// Operators
const ASSIGN: &'static str = "=";
const PLUS: &'static str = "+";

// Delimiters
const COMMA: &'static str = ",";
const SEMICOLON: &'static str = ";";

const LPAREN: &'static str = "(";
const RPAREN: &'static str = ")";
const LBRACE: &'static str = "{";
const RBRACE: &'static str = "}";

// Keywords
const FUNCTION: &'static str = "FUNCTION";
const LET: &'static str = "LET";

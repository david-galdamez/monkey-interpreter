pub type TokenType = String;

#[derive(Debug, Default)]
pub struct Token {
    pub token_type: TokenType,
    pub literal: String,
}

pub const ILLEGAL: &'static str = "ILLEGAL";
pub const EOF: &'static str = "EOF";

// Identifiers + literals
pub const IDENT: &'static str = "IDENT";
pub const INT: &'static str = "INT";

// Operators
pub const ASSIGN: &'static str = "=";
pub const PLUS: &'static str = "+";

// Delimiters
pub const COMMA: &'static str = ",";
pub const SEMICOLON: &'static str = ";";

const LPAREN: &'static str = "(";
const RPAREN: &'static str = ")";
const LBRACE: &'static str = "{";
const RBRACE: &'static str = "}";

// Keywords
const FUNCTION: &'static str = "FUNCTION";
const LET: &'static str = "LET";

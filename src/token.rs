pub type TokenType = &'static str;

#[derive(Debug, Default, Clone)]
pub struct Token {
    pub token_type: TokenType,
    pub literal: String,
}

pub const ILLEGAL: &str = "ILLEGAL";
pub const EOF: &str = "EOF";

// Identifiers + literals
pub const IDENT: &str = "IDENT";
pub const INT: &str = "INT";
pub const STRING: &str = "STRING";

// Operators
pub const ASSIGN: &str = "=";
pub const PLUS: &str = "+";
pub const MINUS: &str = "-";
pub const BANG: &str = "!";
pub const ASTERISK: &str = "*";
pub const SLASH: &str = "/";
pub const EQ: &str = "==";
pub const NOT_EQ: &str = "!=";
pub const LT: &str = "<";
pub const GT: &str = ">";

// Delimiters
pub const COMMA: &str = ",";
pub const SEMICOLON: &str = ";";

pub const LPAREN: &str = "(";
pub const RPAREN: &str = ")";
pub const LBRACE: &str = "{";
pub const RBRACE: &str = "}";
pub const LBRACKET: &str = "[";
pub const RBRACKET: &str = "]";

// Keywords
pub const FUNCTION: &str = "FUNCTION";
pub const LET: &str = "LET";
pub const IF: &str = "IF";
pub const ELSE: &str = "ELSE";
pub const RETURN: &str = "RETURN";
pub const FALSE: &str = "FALSE";
pub const TRUE: &str = "TRUE";

// helper function so we can map identifiers
pub fn lookup_ident(ident: &str) -> TokenType {
    match ident {
        "fn" => FUNCTION,
        "let" => LET,
        "if" => IF,
        "else" => ELSE,
        "return" => RETURN,
        "false" => FALSE,
        "true" => TRUE,
        _other => IDENT,
    }
}

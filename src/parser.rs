use std::{collections::HashMap, mem};

use crate::{
    ast::{
        Expression, ExpressionStatement, Identifier, IntegerLiteral, LetStatement, Program,
        ReturnStatement, Statement,
    },
    lexer, token,
};

#[derive(Debug, Ord, PartialEq, PartialOrd, Eq)]
enum ExpressionConstants {
    LOWEST,
    EQUALS,
    LESSGREATER,
    SUM,
    PRODUCT,
    PREFIX,
    CALL,
}

type PrefixParseFns = fn(&mut Parser) -> Option<Box<dyn Expression>>;
type InfixParseFns = fn(&mut Parser, Box<dyn Expression>) -> Option<Box<dyn Expression>>;

pub struct Parser<'a> {
    lex: lexer::Lexer<'a>,
    cur_token: token::Token,
    peek_token: token::Token,

    prefix_parse_fns: HashMap<token::TokenType, PrefixParseFns>,
    infix_parse_fns: HashMap<token::TokenType, InfixParseFns>,

    errors: Vec<String>,
}

impl<'a> Parser<'a> {
    pub fn new(lex: lexer::Lexer<'a>) -> Self {
        let mut parser = Parser {
            lex,
            cur_token: token::Token::default(),
            peek_token: token::Token::default(),

            prefix_parse_fns: HashMap::new(),
            infix_parse_fns: HashMap::new(),

            errors: Vec::new(),
        };

        parser.register_prefix(token::IDENT, Parser::parse_identifier);
        parser.register_prefix(token::INT, Parser::parse_integer_literal);

        parser.next_token();
        parser.next_token();
        parser
    }

    pub fn errors(&self) -> &[String] {
        &self.errors
    }

    fn peek_error(&mut self, tok: token::TokenType) {
        self.errors.push(format!(
            "expected next token to be {}, got {} instead",
            tok, self.peek_token.token_type
        ));
    }

    fn next_token(&mut self) {
        self.cur_token = mem::replace(&mut self.peek_token, self.lex.next_token());
    }

    pub fn parse_program(&mut self) -> Option<Program> {
        let mut program = Program::default();

        while !self.cur_token_is(token::EOF) {
            if let Some(stmt) = self.parse_statement() {
                program.statements.push(stmt);
            }
            self.next_token();
        }

        Some(program)
    }

    // Statements

    fn parse_statement(&mut self) -> Option<Box<dyn Statement>> {
        match self.cur_token.token_type {
            token::LET => self.parse_let_statement(),
            token::RETURN => self.parse_return_statement(),
            _ => self.parse_expression_statement(),
        }
    }

    fn parse_let_statement(&mut self) -> Option<Box<dyn Statement>> {
        let mut stmt = LetStatement {
            token: self.cur_token.clone(),
            ..Default::default() // We can use this so we don't have to mutate after initializing
        };

        if !self.expect_peek(token::IDENT) {
            return None;
        }

        stmt.name = Identifier {
            token: self.cur_token.clone(),
            value: self.cur_token.literal.clone(),
        };

        if !self.expect_peek(token::ASSIGN) {
            return None;
        }

        // skipping expressions until we encounter a semicolon
        while !self.cur_token_is(token::SEMICOLON) {
            self.next_token();
        }

        // We can use Box::new and the compiler will coerce it to dyn Statement
        Some(Box::new(stmt))
    }

    fn parse_return_statement(&mut self) -> Option<Box<dyn Statement>> {
        let stmt = ReturnStatement {
            token: self.cur_token.clone(),
            ..Default::default()
        };

        self.next_token();

        if !self.cur_token_is(token::SEMICOLON) {
            self.next_token();
        }

        Some(Box::new(stmt))
    }

    fn parse_expression_statement(&mut self) -> Option<Box<dyn Statement>> {
        let mut stmt = ExpressionStatement {
            token: self.cur_token.clone(),
            ..Default::default()
        };

        stmt.expression = self.parse_expression(ExpressionConstants::LOWEST);

        if self.peek_token_is(token::SEMICOLON) {
            self.next_token();
        }

        Some(Box::new(stmt))
    }

    // Expressions

    fn parse_identifier(parser: &mut Parser) -> Option<Box<dyn Expression>> {
        Some(Box::new(Identifier {
            token: parser.cur_token.clone(),
            value: parser.cur_token.literal.clone(),
        }))
    }

    fn parse_integer_literal(parser: &mut Parser) -> Option<Box<dyn Expression>> {
        let mut literal = IntegerLiteral {
            token: parser.cur_token.clone(),
            ..Default::default()
        };

        let value: i64 = match parser.cur_token.literal.parse() {
            Ok(val) => val,
            Err(_) => {
                parser.errors.push(format!(
                    "could not parse {} as integer",
                    parser.cur_token.literal
                ));
                return None;
            }
        };
        literal.value = value;

        Some(Box::new(literal))
    }

    fn parse_expression(&mut self, precedence: ExpressionConstants) -> Option<Box<dyn Expression>> {
        match self.prefix_parse_fns.get(self.cur_token.token_type) {
            Some(pref) => {
                let left_exp = pref(self);
                left_exp
            }
            None => None,
        }
    }

    fn cur_token_is(&self, tok: token::TokenType) -> bool {
        self.cur_token.token_type == tok
    }

    fn peek_token_is(&self, tok: token::TokenType) -> bool {
        self.peek_token.token_type == tok
    }

    fn expect_peek(&mut self, tok: token::TokenType) -> bool {
        if self.peek_token_is(tok) {
            self.next_token();
            true
        } else {
            self.peek_error(tok);
            false
        }
    }

    fn register_prefix(&mut self, tok: token::TokenType, func: PrefixParseFns) {
        self.prefix_parse_fns.insert(tok, func);
    }

    fn register_infix(&mut self, tok: token::TokenType, func: InfixParseFns) {
        self.infix_parse_fns.insert(tok, func);
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        ast::{
            ExpressionStatement, Identifier, IntegerLiteral, LetStatement, Node, ReturnStatement,
            Statement,
        },
        lexer::Lexer,
        parser::Parser,
    };

    struct Expected<'a> {
        expected_identifier: &'a str,
    }

    #[test]
    fn test_let_statement() {
        let input = "
        let x = 5;
        let y = 10;
        let foobar = 838383;
        ";

        let lexer = Lexer::new(input);
        let mut parser = Parser::new(lexer);

        let program = parser.parse_program();
        check_parse_errors(&parser);
        assert!(program.is_some(), "parse_program() returned none");

        let program = program.unwrap();
        assert!(
            program.statements.len() == 3,
            "program.statements does not contain 3 statements, got = {}",
            program.statements.len()
        );

        let tests = vec![
            Expected {
                expected_identifier: "x",
            },
            Expected {
                expected_identifier: "y",
            },
            Expected {
                expected_identifier: "foobar",
            },
        ];

        for (i, test) in tests.iter().enumerate() {
            let statement = &program.statements[i];
            assert!(let_statement(statement, test.expected_identifier));
        }
    }

    fn check_parse_errors(parser: &Parser) {
        let errors = parser.errors();

        if errors.len() == 0 {
            return;
        }

        println!("parser has {} errors", errors.len());
        for err in errors {
            println!("parser error: {err}");
        }
        panic!();
    }

    fn let_statement(stmt: &Box<dyn Statement>, expected: &str) -> bool {
        if stmt.token_literal() != "let" {
            println!("s.token_literal not 'let', got = {}", stmt.token_literal());
            return false;
        }

        let type_stmt = stmt.as_any().downcast_ref::<LetStatement>();

        if type_stmt.is_none() {
            println!("stmt not LetStatement");
            return false;
        }

        let stmt = type_stmt.unwrap();

        if stmt.name.value != expected {
            println!(
                "stmt.name.value not '{}', got = {}",
                expected, stmt.name.value
            );
            return false;
        }

        if stmt.name.token_literal() != expected {
            println!("stmt.name not {}, got = {:?}", expected, stmt.name);
            return false;
        }

        true
    }

    #[test]
    fn test_return_statements() {
        let input = "
        return 5;
        return 10;
        return 993322;
        ";

        let lexer = Lexer::new(input);
        let mut parser = Parser::new(lexer);

        let program = parser.parse_program();
        check_parse_errors(&parser);
        assert!(program.is_some(), "parse_program() returned none");

        let program = program.unwrap();

        assert!(
            program.statements.len() == 3,
            "program.statements does not contain 3 statements, got = {}",
            program.statements.len()
        );

        for stmt in program.statements {
            let type_stmt = stmt.as_any().downcast_ref::<ReturnStatement>();

            if type_stmt.is_none() {
                println!("stmt not Return Statement");
                continue;
            }

            let stmt = type_stmt.unwrap();
            if stmt.token_literal() != "return" {
                println!(
                    "stmt.token_literal no 'return', got = {}",
                    stmt.token_literal()
                );
            }
        }
    }

    #[test]
    fn test_identifier_expression() {
        let input = "foobar";

        let lexer = Lexer::new(input);
        let mut parser = Parser::new(lexer);

        let program = parser.parse_program();
        check_parse_errors(&parser);

        assert!(program.is_some(), "parse_program() returned none");

        let program = program.unwrap();
        assert!(
            program.statements.len() == 1,
            "program.statements does not contain enough statements, got = {}",
            program.statements.len()
        );

        let type_stmt = program.statements[0]
            .as_any()
            .downcast_ref::<ExpressionStatement>();

        assert!(
            type_stmt.is_some(),
            "program.statement[0] is not an expression statement"
        );

        let stmt = type_stmt.unwrap();

        let exp = match &stmt.expression {
            Some(expression) => {
                let exp = expression.as_any();
                exp.downcast_ref::<Identifier>()
            }
            None => {
                panic!("no expression found");
            }
        };

        assert!(
            exp.is_some(),
            "statement.expression is not an identifier expression"
        );
        let exp = exp.unwrap();

        assert!(
            exp.value == "foobar",
            "ident.value not foobar, got = {}",
            exp.value
        );

        assert!(
            exp.token_literal() == "foobar",
            "ident.token_literal not foobar, got = {}",
            exp.value
        );
    }

    #[test]
    fn test_integer_literal() {
        let input = "5;";

        let lexer = Lexer::new(input);
        let mut parser = Parser::new(lexer);
        let program = parser.parse_program();
        check_parse_errors(&parser);

        assert!(program.is_some(), "parse_program() returned none");

        let program = program.unwrap();
        assert!(
            program.statements.len() == 1,
            "program.statements does not contain enough statements, got = {}",
            program.statements.len()
        );

        let type_stmt = program.statements[0]
            .as_any()
            .downcast_ref::<ExpressionStatement>();

        assert!(
            type_stmt.is_some(),
            "program.statement[0] is not an expression statement"
        );

        let stmt = type_stmt.unwrap();

        let exp = match &stmt.expression {
            Some(expression) => {
                let exp = expression.as_any();
                exp.downcast_ref::<IntegerLiteral>()
            }
            None => {
                panic!("no expression found");
            }
        };

        assert!(
            exp.is_some(),
            "statement.expression is not an identifier expression"
        );
        let exp = exp.unwrap();

        assert!(
            exp.value == 5,
            "ident.value not foobar, got = {}",
            exp.value
        );

        assert!(
            exp.token_literal() == "5",
            "ident.token_literal not 5, got = {}",
            exp.value
        );
    }
}

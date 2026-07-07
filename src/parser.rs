use std::{collections::HashMap, mem};

use crate::{
    ast::{
        BlockStatement, Boolean, Expression, ExpressionStatement, FunctionLiteral, Identifier,
        IfExpression, InfixExpression, IntegerLiteral, LetStatement, PrefixExpression, Program,
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

fn precedences(tok: token::TokenType) -> ExpressionConstants {
    match tok {
        token::EQ => ExpressionConstants::EQUALS,
        token::NOT_EQ => ExpressionConstants::EQUALS,
        token::LT => ExpressionConstants::LESSGREATER,
        token::GT => ExpressionConstants::LESSGREATER,
        token::PLUS => ExpressionConstants::SUM,
        token::MINUS => ExpressionConstants::SUM,
        token::SLASH => ExpressionConstants::PRODUCT,
        token::ASTERISK => ExpressionConstants::PRODUCT,
        _ => ExpressionConstants::LOWEST,
    }
}

type PrefixParseFns = fn(&mut Parser) -> Option<Box<dyn Expression>>;
type InfixParseFns = fn(&mut Parser, Option<Box<dyn Expression>>) -> Option<Box<dyn Expression>>;

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

        // register prefixes
        parser.register_prefix(token::IDENT, Parser::parse_identifier);
        parser.register_prefix(token::INT, Parser::parse_integer_literal);
        parser.register_prefix(token::BANG, Parser::parse_prefix_expression);
        parser.register_prefix(token::MINUS, Parser::parse_prefix_expression);
        parser.register_prefix(token::TRUE, Parser::parse_boolean);
        parser.register_prefix(token::FALSE, Parser::parse_boolean);
        parser.register_prefix(token::LPAREN, Parser::parse_grouped_expression);
        parser.register_prefix(token::IF, Parser::parse_if_expression);
        parser.register_prefix(token::FUNCTION, Parser::parse_function_literal);

        // register infixes
        parser.register_infix(token::PLUS, Parser::parse_infix_expression);
        parser.register_infix(token::MINUS, Parser::parse_infix_expression);
        parser.register_infix(token::SLASH, Parser::parse_infix_expression);
        parser.register_infix(token::ASTERISK, Parser::parse_infix_expression);
        parser.register_infix(token::EQ, Parser::parse_infix_expression);
        parser.register_infix(token::NOT_EQ, Parser::parse_infix_expression);
        parser.register_infix(token::LT, Parser::parse_infix_expression);
        parser.register_infix(token::GT, Parser::parse_infix_expression);

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

    fn parse_block_statement(&mut self) -> Option<BlockStatement> {
        let mut block = BlockStatement {
            token: self.cur_token.clone(),
            ..Default::default()
        };

        self.next_token();

        while !self.cur_token_is(token::RBRACE) && !self.cur_token_is(token::EOF) {
            if let Some(stmt) = self.parse_statement() {
                block.statements.push(stmt);
            }
            self.next_token();
        }

        Some(block)
    }

    fn parse_function_parameters(&mut self) -> Vec<Identifier> {
        let mut identifiers = Vec::new();

        if self.peek_token_is(token::RPAREN) {
            self.next_token();
            return identifiers;
        }

        self.next_token();

        let ident = Identifier {
            token: self.cur_token.clone(),
            value: self.cur_token.literal.clone(),
        };
        identifiers.push(ident);

        while self.peek_token_is(token::COMMA) {
            self.next_token();
            self.next_token();
            let ident = Identifier {
                token: self.cur_token.clone(),
                value: self.cur_token.literal.clone(),
            };
            identifiers.push(ident);
        }

        if !self.expect_peek(token::RPAREN) {
            return Vec::new();
        }

        identifiers
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

    fn parse_prefix_expression(parser: &mut Parser) -> Option<Box<dyn Expression>> {
        let mut expression = PrefixExpression {
            token: parser.cur_token.clone(),
            operator: parser.cur_token.literal.clone(),
            ..Default::default()
        };

        parser.next_token();

        expression.right = parser.parse_expression(ExpressionConstants::PREFIX);

        Some(Box::new(expression))
    }

    fn parse_boolean(parser: &mut Parser) -> Option<Box<dyn Expression>> {
        Some(Box::new(Boolean {
            token: parser.cur_token.clone(),
            value: parser.cur_token_is(token::TRUE),
        }))
    }

    fn parse_grouped_expression(parser: &mut Parser) -> Option<Box<dyn Expression>> {
        parser.next_token();

        let exp = parser.parse_expression(ExpressionConstants::LOWEST);

        if !parser.expect_peek(token::RPAREN) {
            return None;
        }

        exp
    }

    fn parse_if_expression(parser: &mut Parser) -> Option<Box<dyn Expression>> {
        let mut exp = IfExpression {
            token: parser.cur_token.clone(),
            ..Default::default()
        };

        if !parser.expect_peek(token::LPAREN) {
            return None;
        }

        parser.next_token();
        exp.condition = parser.parse_expression(ExpressionConstants::LOWEST);

        if !parser.expect_peek(token::RPAREN) {
            return None;
        }

        if !parser.expect_peek(token::LBRACE) {
            return None;
        }

        exp.consequence = parser.parse_block_statement();

        if parser.peek_token_is(token::ELSE) {
            parser.next_token();

            if !parser.peek_token_is(token::LBRACE) {
                return None;
            }

            exp.alternative = parser.parse_block_statement();
        }

        Some(Box::new(exp))
    }

    fn parse_function_literal(parser: &mut Parser) -> Option<Box<dyn Expression>> {
        let mut lit = FunctionLiteral {
            token: parser.cur_token.clone(),
            ..Default::default()
        };

        if !parser.expect_peek(token::LPAREN) {
            return None;
        }

        lit.parameters = parser.parse_function_parameters();

        if !parser.expect_peek(token::LBRACE) {
            return None;
        }

        lit.body = parser.parse_block_statement();

        Some(Box::new(lit))
    }

    // infixes

    fn parse_infix_expression(
        parser: &mut Parser,
        left: Option<Box<dyn Expression>>,
    ) -> Option<Box<dyn Expression>> {
        let mut expression = InfixExpression {
            token: parser.cur_token.clone(),
            operator: parser.cur_token.literal.clone(),
            left: left,
            ..Default::default()
        };

        let precedence = parser.cur_precedence();
        parser.next_token();
        expression.right = parser.parse_expression(precedence);

        Some(Box::new(expression))
    }

    fn no_prefix_parse_fn_error(&mut self, tok: token::TokenType) {
        self.errors
            .push(format!("no prefix parse function for {} found", tok));
    }

    fn parse_expression(&mut self, precedence: ExpressionConstants) -> Option<Box<dyn Expression>> {
        match self.prefix_parse_fns.get(self.cur_token.token_type) {
            Some(pref) => {
                let mut left_exp = pref(self);

                while !self.peek_token_is(token::SEMICOLON) && precedence < self.peek_precedence() {
                    let infix = match self.infix_parse_fns.get(&self.peek_token.token_type) {
                        Some(func) => *func,
                        None => return left_exp,
                    };

                    self.next_token();

                    left_exp = infix(self, left_exp);
                }

                left_exp
            }
            None => {
                self.no_prefix_parse_fn_error(self.cur_token.token_type);
                None
            }
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

    fn peek_precedence(&self) -> ExpressionConstants {
        precedences(self.peek_token.token_type)
    }

    fn cur_precedence(&self) -> ExpressionConstants {
        precedences(self.cur_token.token_type)
    }
}

#[cfg(test)]
mod tests {
    use std::any::Any;

    use crate::{
        ast::{
            Boolean, Expression, ExpressionStatement, FunctionLiteral, Identifier, IfExpression,
            InfixExpression, IntegerLiteral, LetStatement, Node, PrefixExpression, ReturnStatement,
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

    struct PrefixTest<'a> {
        input: &'a str,
        operator: &'a str,
        value: &'a dyn Any,
    }

    #[test]
    fn test_parsing_prefix_expression() {
        let prefix_tests = vec![
            PrefixTest {
                input: "!5;",
                operator: "!",
                value: &5,
            },
            PrefixTest {
                input: "-15;",
                operator: "-",
                value: &15,
            },
            PrefixTest {
                input: "!true;",
                operator: "!",
                value: &true,
            },
            PrefixTest {
                input: "!false;",
                operator: "!",
                value: &false,
            },
        ];

        for test in prefix_tests {
            let lexer = Lexer::new(test.input);
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
                    exp.downcast_ref::<PrefixExpression>()
                }
                None => {
                    panic!("no expression found");
                }
            };

            assert!(
                exp.is_some(),
                "statement.expression is not a prefix expression"
            );
            let exp = exp.unwrap();

            assert!(
                exp.operator == test.operator,
                "exp.operator is not {}, got = {}",
                test.operator,
                exp.operator
            );

            if !test_literal_expression(&exp.right, test.value) {
                return;
            }
        }
    }

    struct InfixTests<'a> {
        input: &'a str,
        left_val: &'a dyn Any,
        operator: &'a str,
        right_val: &'a dyn Any,
    }

    #[test]
    fn test_parsing_infix_expressions() {
        let infix_test = vec![
            InfixTests {
                input: "5 + 5;",
                left_val: &5,
                operator: "+",
                right_val: &5,
            },
            InfixTests {
                input: "5 - 5;",
                left_val: &5,
                operator: "-",
                right_val: &5,
            },
            InfixTests {
                input: "5 * 5;",
                left_val: &5,
                operator: "*",
                right_val: &5,
            },
            InfixTests {
                input: "5 / 5;",
                left_val: &5,
                operator: "/",
                right_val: &5,
            },
            InfixTests {
                input: "5 > 5;",
                left_val: &5,
                operator: ">",
                right_val: &5,
            },
            InfixTests {
                input: "5 < 5;",
                left_val: &5,
                operator: "<",
                right_val: &5,
            },
            InfixTests {
                input: "5 == 5;",
                left_val: &5,
                operator: "==",
                right_val: &5,
            },
            InfixTests {
                input: "5 != 5;",
                left_val: &5,
                operator: "!=",
                right_val: &5,
            },
            InfixTests {
                input: "true == true",
                left_val: &true,
                operator: "==",
                right_val: &true,
            },
            InfixTests {
                input: "true != false",
                left_val: &true,
                operator: "!=",
                right_val: &false,
            },
            InfixTests {
                input: "false == false",
                left_val: &false,
                operator: "==",
                right_val: &false,
            },
        ];

        for test in infix_test {
            let lexer = Lexer::new(test.input);
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
                    exp.downcast_ref::<InfixExpression>()
                }
                None => {
                    panic!("no expression found");
                }
            };

            assert!(
                exp.is_some(),
                "statement.expression is not a prefix expression"
            );
            let exp = exp.unwrap();

            if !test_literal_expression(&exp.left, &test.left_val) {
                return;
            }

            assert!(
                exp.operator == test.operator,
                "exp.operator is not {}, got = {}",
                test.operator,
                exp.operator
            );

            if !test_literal_expression(&exp.right, &test.right_val) {
                return;
            }
        }
    }

    #[test]
    fn test_boolean_expression() {
        let input = "true;";

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
                exp.downcast_ref::<Boolean>()
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

        assert!(exp.value, "ident.value not true, got = {}", exp.value);

        assert!(
            exp.token_literal() == "true",
            "ident.token_literal not true, got = {}",
            exp.value
        );
    }

    struct PrecedenceTest<'a> {
        input: &'a str,
        expected: &'a str,
    }

    #[test]
    fn test_operator_precedence_parsing() {
        let tests = vec![
            PrecedenceTest {
                input: "true",
                expected: "true",
            },
            PrecedenceTest {
                input: "false",
                expected: "false",
            },
            PrecedenceTest {
                input: "3 > 5 == false",
                expected: "((3 > 5) == false)",
            },
            PrecedenceTest {
                input: "3 < 5 == true",
                expected: "((3 < 5) == true)",
            },
            PrecedenceTest {
                input: "1 + (2 + 3) + 4",
                expected: "((1 + (2 + 3)) + 4)",
            },
            PrecedenceTest {
                input: "(5 + 5) * 2",
                expected: "((5 + 5) * 2)",
            },
            PrecedenceTest {
                input: "2 / (5 + 5)",
                expected: "(2 / (5 + 5))",
            },
            PrecedenceTest {
                input: "-(5 + 5)",
                expected: "(-(5 + 5))",
            },
            PrecedenceTest {
                input: "!(true == true)",
                expected: "(!(true == true))",
            },
        ];

        for test in tests {
            let lexer = Lexer::new(test.input);
            let mut parser = Parser::new(lexer);
            let program = parser.parse_program();
            check_parse_errors(&parser);

            assert!(program.is_some(), "parse_program() returned none");

            let program = program.unwrap();

            let actual = format!("{}", program);
            assert_eq!(
                actual, test.expected,
                "expected = {}, got = {}",
                test.expected, actual
            )
        }
    }

    #[test]
    fn test_if_expressions() {
        let input = "if (x < y) { x }";
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
                exp.downcast_ref::<IfExpression>()
            }
            None => {
                panic!("no expression found");
            }
        };

        assert!(
            exp.is_some(),
            "statement.expression is not an IfExpression expression"
        );
        let exp = exp.unwrap();

        if !test_infix_expression(&exp.condition, &"x", "<", &"y") {
            return;
        }

        assert!(
            exp.consequence.as_ref().unwrap().statements.len() == 1,
            "exp.consequence.statements does not contain enough statements, got = {}",
            program.statements.len()
        );

        let consequence = exp.consequence.as_ref().unwrap().statements[0]
            .as_any()
            .downcast_ref::<ExpressionStatement>();

        assert!(
            type_stmt.is_some(),
            "exp.consequence.statement[0] is not an expression statement"
        );

        let consequence = consequence.unwrap();

        if !test_identifier(&consequence.expression, "x") {
            return;
        }

        assert!(
            exp.alternative.is_none(),
            "exp.alternative.statements was not none, got = {:#?}",
            exp.alternative
        );
    }

    #[test]
    fn test_function_literal_parsing() {
        let input = "fn(x, y) { x + y; }";

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

        let func = match &stmt.expression {
            Some(expression) => {
                let exp = expression.as_any();
                exp.downcast_ref::<FunctionLiteral>()
            }
            None => {
                panic!("no expression found");
            }
        };

        assert!(func.is_some(), "func is not an FunctionLiteral expression");
        let func = func.unwrap();

        assert!(
            func.parameters.len() == 2,
            "function literal parameters wrong, want 2, got = {}",
            func.parameters.len()
        );

        test_literal_expression(&Some(Box::new(func.parameters[0].clone())), &"x");
        test_literal_expression(&Some(Box::new(func.parameters[1].clone())), &"y");

        assert!(
            func.body.as_ref().unwrap().statements.len() == 1,
            "function.body.statements has not 1 statement, got = {}",
            func.body.as_ref().unwrap().statements.len()
        );

        let body_stmt = func
            .body
            .as_ref()
            .unwrap()
            .statements
            .first()
            .unwrap()
            .as_any()
            .downcast_ref::<ExpressionStatement>();

        assert!(
            body_stmt.is_some(),
            "function body stmt is not an ExpressionStatement"
        );

        let body_stmt = body_stmt.unwrap();

        assert!(test_infix_expression(
            &body_stmt.expression,
            &"x",
            "+",
            &"y"
        ))
    }

    // HELPERS FOR THE TESTS

    fn test_literal_expression(exp: &Option<Box<dyn Expression>>, expected: &dyn Any) -> bool {
        if expected.is::<i64>() {
            return test_integer_literal_helper(&exp, expected.downcast_ref::<i64>().unwrap());
        } else if expected.is::<&str>() {
            return test_identifier(exp, expected.downcast_ref::<&str>().unwrap());
        } else if expected.is::<bool>() {
            return test_boolean_literal(exp, expected.downcast_ref::<bool>().unwrap());
        } else {
            eprintln!("type of exp not handled, got = {:#?}", exp);
            return false;
        }
    }

    fn test_identifier(exp: &Option<Box<dyn Expression>>, value: &str) -> bool {
        let exp = match &exp {
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

        if exp.value != value {
            eprintln!("ident.value not {}, got = {}", value, exp.value);
            return false;
        }

        if exp.token_literal() != value {
            eprintln!("ident.token_literal not {}, got = {}", value, exp.value);
            return false;
        }

        true
    }

    fn test_integer_literal_helper(
        integer_literal: &Option<Box<dyn Expression>>,
        value: &i64,
    ) -> bool {
        match integer_literal {
            Some(exp) => {
                let type_exp = exp.as_any().downcast_ref::<IntegerLiteral>();

                assert!(
                    type_exp.is_some(),
                    "integer_literal is not an IntegerLiteral, got = {:#?}",
                    exp
                );

                let integer = type_exp.unwrap();

                if integer.value != *value {
                    eprintln!("integ.Value not {}, got = {}", value, integer.value);
                    return false;
                }

                if integer.token_literal() != format!("{}", value) {
                    eprintln!(
                        "integ.token_literal not {}, got = {}",
                        value,
                        integer.token_literal()
                    );
                    return false;
                }

                true
            }
            None => false,
        }
    }

    fn test_infix_expression(
        exp: &Option<Box<dyn Expression>>,
        left: &dyn Any,
        operator: &str,
        right: &dyn Any,
    ) -> bool {
        let exp = match &exp {
            Some(expression) => {
                let exp = expression.as_any();
                exp.downcast_ref::<InfixExpression>()
            }
            None => {
                panic!("no expression found");
            }
        };

        assert!(
            exp.is_some(),
            "statement.expression is not an InfixExpression expression"
        );
        let exp = exp.unwrap();

        if !test_literal_expression(&exp.left, left) {
            return false;
        }

        if exp.operator != operator {
            eprintln!("exp.operator is not {}, got = {}", operator, exp.operator);
            return false;
        }

        if !test_literal_expression(&exp.right, right) {
            return false;
        }

        true
    }

    fn test_boolean_literal(exp: &Option<Box<dyn Expression>>, value: &bool) -> bool {
        let exp = match &exp {
            Some(expression) => {
                let exp = expression.as_any();
                exp.downcast_ref::<Boolean>()
            }
            None => {
                panic!("no expression found");
            }
        };

        assert!(
            exp.is_some(),
            "statement.expression is not an InfixExpression expression"
        );
        let exp = exp.unwrap();

        if exp.value != *value {
            eprintln!("exp.operator is not {}, got = {}", value, exp.value);
            return false;
        }

        if exp.token_literal() != format!("{}", value) {
            eprintln!(
                "exp.token_literal not {}, got = {}",
                value,
                exp.token_literal()
            );
            return false;
        }

        true
    }
}

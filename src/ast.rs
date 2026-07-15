use std::{any::Any, fmt};

use crate::token;

pub trait Node {
    fn token_literal(&self) -> &str;
    fn as_any(&self) -> &dyn Any;
}

pub trait Statement: Node + fmt::Debug + fmt::Display {
    fn statement_node(&self);
}

pub trait Expression: Node + fmt::Debug + fmt::Display {
    fn expression_node(&self);
    fn as_node(self: Box<Self>) -> Box<dyn Node>;
}

#[derive(Debug, Default)]
pub struct Program {
    // We use a Box<dyn Statement> because we can have a lot of Statement implementation
    pub statements: Vec<Box<dyn Statement>>,
}

impl fmt::Display for Program {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for stmt in &self.statements {
            write!(f, "{}", stmt)?;
        }

        Ok(())
    }
}

impl Node for Program {
    fn token_literal(&self) -> &str {
        match self.statements.first() {
            Some(statement) => statement.token_literal(),
            None => "",
        }
    }

    // we use the Any trait so we can downcast to the type we're testing
    fn as_any(&self) -> &dyn Any {
        self
    }
}

#[derive(Debug, Default)]
pub struct LetStatement {
    pub token: token::Token, // token::LET token
    pub name: Identifier,
    pub value: Option<Box<dyn Expression>>,
}

impl fmt::Display for LetStatement {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} ", self.token_literal())?;
        write!(f, "{}", self.name)?;
        write!(f, " = ")?;

        if let Some(val) = &self.value {
            write!(f, "{}", val)?;
        }

        write!(f, ";")
    }
}

impl Node for LetStatement {
    fn token_literal(&self) -> &str {
        &self.token.literal
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

impl Statement for LetStatement {
    fn statement_node(&self) {}
}

#[derive(Debug, Default)]
pub struct ReturnStatement {
    pub token: token::Token,
    pub return_value: Option<Box<dyn Expression>>,
}

impl fmt::Display for ReturnStatement {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} ", self.token_literal())?;
        if let Some(val) = &self.return_value {
            write!(f, "{}", val)?;
        }
        write!(f, ";")
    }
}

impl Node for ReturnStatement {
    fn token_literal(&self) -> &str {
        &self.token.literal
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

impl Statement for ReturnStatement {
    fn statement_node(&self) {}
}

#[derive(Debug, Default)]
pub struct ExpressionStatement {
    pub token: token::Token,
    pub expression: Option<Box<dyn Expression>>,
}

impl fmt::Display for ExpressionStatement {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if let Some(val) = &self.expression {
            write!(f, "{}", val)?;
        }
        Ok(())
    }
}

impl Node for ExpressionStatement {
    fn token_literal(&self) -> &str {
        &self.token.literal
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

impl Statement for ExpressionStatement {
    fn statement_node(&self) {}
}

#[derive(Debug, Default, Clone)]
pub struct Identifier {
    pub token: token::Token, // token::IDENT token
    pub value: String,
}

impl fmt::Display for Identifier {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.value)
    }
}

impl Node for Identifier {
    fn token_literal(&self) -> &str {
        &self.token.literal
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

impl Expression for Identifier {
    fn expression_node(&self) {}
    fn as_node(self: Box<Self>) -> Box<dyn Node> {
        self
    }
}

#[derive(Debug, Default, Clone)]
pub struct IntegerLiteral {
    pub token: token::Token, // token::IDENT token
    pub value: i64,
}

impl fmt::Display for IntegerLiteral {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.value)
    }
}

impl Node for IntegerLiteral {
    fn token_literal(&self) -> &str {
        &self.token.literal
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

impl Expression for IntegerLiteral {
    fn expression_node(&self) {}
    fn as_node(self: Box<Self>) -> Box<dyn Node> {
        self
    }
}

#[derive(Debug, Default)]
pub struct PrefixExpression {
    pub token: token::Token,
    pub operator: String,
    pub right: Option<Box<dyn Expression>>,
}

impl fmt::Display for PrefixExpression {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "(")?;
        write!(f, "{}", self.operator)?;
        write!(f, "{}", self.right.as_ref().unwrap())?;
        write!(f, ")")
    }
}

impl Node for PrefixExpression {
    fn token_literal(&self) -> &str {
        &self.token.literal
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

impl Expression for PrefixExpression {
    fn expression_node(&self) {}
    fn as_node(self: Box<Self>) -> Box<dyn Node> {
        self
    }
}

#[derive(Debug, Default)]
pub struct InfixExpression {
    pub token: token::Token,
    pub left: Option<Box<dyn Expression>>,
    pub operator: String,
    pub right: Option<Box<dyn Expression>>,
}

impl fmt::Display for InfixExpression {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "(")?;
        write!(f, "{} ", self.left.as_ref().unwrap())?;
        write!(f, "{} ", self.operator)?;
        write!(f, "{}", self.right.as_ref().unwrap())?;
        write!(f, ")")
    }
}

impl Node for InfixExpression {
    fn token_literal(&self) -> &str {
        &self.token.literal
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

impl Expression for InfixExpression {
    fn expression_node(&self) {}
    fn as_node(self: Box<Self>) -> Box<dyn Node> {
        self
    }
}

#[derive(Debug, Default)]
pub struct Boolean {
    pub token: token::Token,
    pub value: bool,
}

impl fmt::Display for Boolean {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.value)
    }
}

impl Node for Boolean {
    fn token_literal(&self) -> &str {
        &self.token.literal
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

impl Expression for Boolean {
    fn expression_node(&self) {}
    fn as_node(self: Box<Self>) -> Box<dyn Node> {
        self
    }
}

#[derive(Debug, Default)]
pub struct IfExpression {
    pub token: token::Token,
    pub condition: Option<Box<dyn Expression>>,
    pub consequence: Option<BlockStatement>,
    pub alternative: Option<BlockStatement>,
}

impl fmt::Display for IfExpression {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "if")?;
        write!(f, "{}", self.condition.as_ref().unwrap())?;
        write!(f, " ")?;
        write!(f, "{}", self.consequence.as_ref().unwrap())?;

        if let Some(alt) = &self.alternative {
            write!(f, "else ")?;
            write!(f, "{}", alt)?;
        }

        Ok(())
    }
}

impl Node for IfExpression {
    fn token_literal(&self) -> &str {
        &self.token.literal
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

impl Expression for IfExpression {
    fn as_node(self: Box<Self>) -> Box<dyn Node> {
        self
    }
    fn expression_node(&self) {}
}

#[derive(Debug, Default)]
pub struct BlockStatement {
    pub token: token::Token,
    pub statements: Vec<Box<dyn Statement>>,
}

impl fmt::Display for BlockStatement {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for stmt in &self.statements {
            write!(f, "{}", stmt)?;
        }

        Ok(())
    }
}

impl Node for BlockStatement {
    fn token_literal(&self) -> &str {
        &self.token.literal
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

impl Statement for BlockStatement {
    fn statement_node(&self) {}
}

#[derive(Debug, Default)]
pub struct FunctionLiteral {
    pub token: token::Token,
    pub parameters: Vec<Identifier>,
    pub body: Option<BlockStatement>,
}

impl fmt::Display for FunctionLiteral {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let params: Vec<String> = self.parameters.iter().map(|p| format!("{}", p)).collect();

        write!(f, "{}", self.token_literal())?;
        write!(f, "(")?;
        write!(f, "{}", params.join(", "))?;
        write!(f, ")")?;
        write!(f, "{}", self.body.as_ref().unwrap())
    }
}

impl Node for FunctionLiteral {
    fn token_literal(&self) -> &str {
        &self.token.literal
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

impl Expression for FunctionLiteral {
    fn expression_node(&self) {}
    fn as_node(self: Box<Self>) -> Box<dyn Node> {
        self
    }
}

#[derive(Debug, Default)]
pub struct CallExpression {
    pub token: token::Token,
    pub function: Option<Box<dyn Expression>>,
    pub arguments: Vec<Box<dyn Expression>>,
}

impl fmt::Display for CallExpression {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let args: Vec<String> = self.arguments.iter().map(|p| format!("{}", p)).collect();

        write!(f, "{}", self.function.as_ref().unwrap())?;
        write!(f, "(")?;
        write!(f, "{}", args.join(", "))?;
        write!(f, ")")
    }
}

impl Node for CallExpression {
    fn token_literal(&self) -> &str {
        &self.token.literal
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

impl Expression for CallExpression {
    fn expression_node(&self) {}
    fn as_node(self: Box<Self>) -> Box<dyn Node> {
        self
    }
}

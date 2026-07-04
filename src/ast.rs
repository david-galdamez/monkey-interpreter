use std::{any::Any, fmt};

use crate::token;

pub trait Node {
    fn token_literal(&self) -> &str;
    fn as_any(&self) -> &dyn Any;
}

pub trait Statement: Node + fmt::Debug + fmt::Display {
    fn statement_node(&self);
}

trait Expression: Node + fmt::Debug + fmt::Display {
    fn expression_node(&self);
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
    pub value: Identifier,
}

impl fmt::Display for LetStatement {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} ", self.token_literal())?;
        write!(f, "{}", self.name)?;
        write!(f, " = ")?;

        if self.value {
            write!(f, "{}", self.value)?;
        }
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
    pub return_value: Identifier,
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
    token: token::Token,
    expression: Identifier,
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

#[derive(Debug, Default)]
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
}

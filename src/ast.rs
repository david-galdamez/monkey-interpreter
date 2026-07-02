use crate::token;

trait Node {
    fn token_literal(&self) -> &str;
}

trait Statement: Node {
    fn statement_node(&self);
}

trait Expression: Node {
    fn expression_node(&self);
}

#[derive(Debug, Default)]
pub struct Program<T: Statement> {
    statements: Vec<T>
}

impl<T: Statement> Node for Program<T> {
    fn token_literal(&self) -> &str {
        match self.statements.first() {
            Some(statement) => statement.token_literal(),
            None => "",
        }
    }
}

#[derive(Debug, Default)]
struct LetStatement<T: Expression> {
    token: token::Token, // token::LET token
    name: Identifier,
    value: T,
}

impl<T: Expression> Node for LetStatement<T> {
    fn token_literal(&self) -> &str {
        &self.token.literal
    }
}

impl<T: Expression> Statement for LetStatement<T> {
    fn statement_node(&self) {
    }
}

#[derive(Debug, Default)]
struct Identifier {
    token: token::Token, // token::IDENT token
    value: String,
}

impl Node for Identifier {
    fn token_literal(&self) -> &str {
        &self.token.literal
    }
}

impl Expression for Identifier {
    fn expression_node(&self) {
    }
}

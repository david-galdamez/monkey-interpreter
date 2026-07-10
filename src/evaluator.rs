use crate::{ast, object};

fn eval(node: Option<Box<dyn ast::Node>>) -> Option<Box<dyn object::Object>> {
    if node.is_none() {
        return None;
    }

    let node = node.unwrap();

    if let Some(node) = node.as_any().downcast_ref::<ast::Program>() {
        return eval_statement(&node.statements);
    } else if let Some(node) = node.as_any().downcast_ref::<ast::ExpressionStatement>() {
        return eval(node.expression.as_ref().map(|e| e.as_node()));
    } else if let Some(node) = node.as_any().downcast_ref::<ast::IntegerLiteral>() {
        return Some(Box::new(object::Integer { value: node.value }));
    }

    None
}

fn eval_statement(stmts: &Vec<Box<dyn ast::Statement>>) -> Option<Box<dyn object::Object>> {
    let mut result = None;

    for statements in stmts {
        result = eval(Some(statements));
    }

    result
}

#[cfg(test)]
mod tests {
    use crate::{evaluator::eval, lexer, object, parser};

    struct EvalInteger<'a> {
        input: &'a str,
        expected: i64,
    }

    #[test]
    fn test_eval_integer_expression() {
        let tests = vec![
            EvalInteger {
                input: "5",
                expected: 5,
            },
            EvalInteger {
                input: "10",
                expected: 10,
            },
        ];

        for test in tests {
            let evaluated = test_eval(test.input);
            test_integer_object(&evaluated, test.expected);
        }
    }

    fn test_eval(input: &str) -> Option<Box<dyn object::Object>> {
        let lexer = lexer::Lexer::new(input);
        let mut parser = parser::Parser::new(lexer);
        let program = parser.parse_program();

        eval(Some(Box::new(program.unwrap())))
    }

    fn test_integer_object(obj: &Option<Box<dyn object::Object>>, expected: i64) -> bool {
        assert!(obj.is_some(), "object not found");
        let obj = obj.unwrap();

        let result = obj.as_any().downcast_ref::<object::Integer>();

        assert!(result.is_some(), "object is not an integer");

        let result = result.unwrap();

        assert_eq!(
            result.value, expected,
            "object has wrong value, got = {}, want = {}",
            result.value, expected
        );

        true
    }
}

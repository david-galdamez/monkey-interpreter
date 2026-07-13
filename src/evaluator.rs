use crate::{ast, object};

pub fn eval(node: &dyn ast::Node) -> Box<dyn object::Object> {
    if let Some(node) = node.as_any().downcast_ref::<ast::Program>() {
        return eval_statement(&node.statements);
    } else if let Some(node) = node.as_any().downcast_ref::<ast::ExpressionStatement>() {
        return eval(
            node.expression
                .as_ref()
                .expect("node expression not found")
                .as_ref(),
        );
    } else if let Some(node) = node.as_any().downcast_ref::<ast::IntegerLiteral>() {
        return Box::new(object::Integer { value: node.value });
    } else if let Some(node) = node.as_any().downcast_ref::<ast::Boolean>() {
        return Box::new(object::Boolean { value: node.value });
    } else if let Some(node) = node.as_any().downcast_ref::<ast::PrefixExpression>() {
        let right = eval(
            node.right
                .as_ref()
                .expect("right expression not found")
                .as_ref(),
        );
        return eval_prefix_expression(&node.operator, right);
    } else if let Some(node) = node.as_any().downcast_ref::<ast::InfixExpression>() {
        let left = eval(
            node.left
                .as_ref()
                .expect("left expression not found")
                .as_ref(),
        );

        let right = eval(
            node.right
                .as_ref()
                .expect("right expression not found")
                .as_ref(),
        );
        return eval_infix_expression(&node.operator, left, right);
    }

    Box::new(object::Null)
}

fn eval_statement(stmts: &Vec<Box<dyn ast::Statement>>) -> Box<dyn object::Object> {
    let mut result: Box<dyn object::Object> = Box::new(object::Null);

    for statements in stmts {
        result = eval(statements.as_ref());
    }

    result
}

fn eval_prefix_expression<'a>(
    operator: &'a str,
    right: Box<dyn object::Object>,
) -> Box<dyn object::Object> {
    match operator {
        "!" => eval_bang_operator_expression(right),
        "-" => eval_minus_prefix_operator_expression(right),
        _other => Box::new(object::Null),
    }
}

fn eval_infix_expression<'a>(
    operator: &'a str,
    left: Box<dyn object::Object>,
    right: Box<dyn object::Object>,
) -> Box<dyn object::Object> {
    if left.object_type() == object::INTEGER_OBJ && right.object_type() == object::INTEGER_OBJ {
        return eval_integer_infix_expression(operator, left, right);
    }

    let left_val = left
        .as_any()
        .downcast_ref::<object::Boolean>()
        .expect("left object integer not found")
        .value;

    let right_val = right
        .as_any()
        .downcast_ref::<object::Boolean>()
        .expect("right object integer not found")
        .value;

    if operator == "==" {
        return Box::new(object::Boolean {
            value: left_val == right_val,
        });
    } else if operator == "!=" {
        return Box::new(object::Boolean {
            value: left_val != right_val,
        });
    }

    Box::new(object::Null)
}

fn eval_integer_infix_expression<'a>(
    operator: &'a str,
    left: Box<dyn object::Object>,
    right: Box<dyn object::Object>,
) -> Box<dyn object::Object> {
    let left_val = left
        .as_any()
        .downcast_ref::<object::Integer>()
        .expect("left object integer not found")
        .value;

    let right_val = right
        .as_any()
        .downcast_ref::<object::Integer>()
        .expect("right object integer not found")
        .value;

    match operator {
        "+" => Box::new(object::Integer {
            value: left_val + right_val,
        }),
        "-" => Box::new(object::Integer {
            value: left_val - right_val,
        }),
        "*" => Box::new(object::Integer {
            value: left_val * right_val,
        }),
        "/" => Box::new(object::Integer {
            value: left_val / right_val,
        }),
        "<" => Box::new(object::Boolean {
            value: left_val < right_val,
        }),
        ">" => Box::new(object::Boolean {
            value: left_val > right_val,
        }),
        "==" => Box::new(object::Boolean {
            value: left_val == right_val,
        }),
        "!=" => Box::new(object::Boolean {
            value: left_val != right_val,
        }),
        _other => Box::new(object::Null),
    }
}

fn eval_minus_prefix_operator_expression(
    right: Box<dyn object::Object>,
) -> Box<dyn object::Object> {
    if right.object_type() != object::INTEGER_OBJ {
        return Box::new(object::Null);
    }

    let value = right
        .as_any()
        .downcast_ref::<object::Integer>()
        .expect("object integer not found")
        .value;

    Box::new(object::Integer { value: -value })
}

fn eval_bang_operator_expression(right: Box<dyn object::Object>) -> Box<dyn object::Object> {
    if let Some(obj) = right.as_any().downcast_ref::<object::Boolean>() {
        if obj.value {
            Box::new(object::Boolean { value: false })
        } else {
            Box::new(object::Boolean { value: true })
        }
    } else if right.as_any().downcast_ref::<object::Null>().is_some() {
        Box::new(object::Boolean { value: true })
    } else {
        Box::new(object::Boolean { value: false })
    }
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
            EvalInteger {
                input: "-5",
                expected: -5,
            },
            EvalInteger {
                input: "-10",
                expected: -10,
            },
            EvalInteger {
                input: "5 + 5 + 5 + 5 - 10",
                expected: 10,
            },
            EvalInteger {
                input: "2 * 2 * 2 * 2 * 2",
                expected: 32,
            },
            EvalInteger {
                input: "-50 + 100 + -50",
                expected: 0,
            },
            EvalInteger {
                input: "5 * 2 + 10",
                expected: 20,
            },
            EvalInteger {
                input: "5 + 2 * 10",
                expected: 25,
            },
            EvalInteger {
                input: "20 + 2 * -10",
                expected: 0,
            },
            EvalInteger {
                input: "50 / 2 * 2 + 10",
                expected: 60,
            },
            EvalInteger {
                input: "2 * (5 + 10)",
                expected: 30,
            },
            EvalInteger {
                input: "3 * 3 * 3 + 10",
                expected: 37,
            },
            EvalInteger {
                input: "3 * (3 * 3) + 10",
                expected: 37,
            },
            EvalInteger {
                input: "(5 + 10 * 2 + 15 / 3) * 2 + -10",
                expected: 50,
            },
        ];

        for test in tests {
            let evaluated = test_eval(test.input);
            test_integer_object(evaluated, test.expected);
        }
    }

    struct EvalBoolean<'a> {
        input: &'a str,
        expected: bool,
    }

    #[test]
    fn test_eval_boolean_expression() {
        let tests = vec![
            EvalBoolean {
                input: "true",
                expected: true,
            },
            EvalBoolean {
                input: "false",
                expected: false,
            },
            EvalBoolean {
                input: "1 < 2",
                expected: true,
            },
            EvalBoolean {
                input: "1 > 2",
                expected: false,
            },
            EvalBoolean {
                input: "1 < 1",
                expected: false,
            },
            EvalBoolean {
                input: "1 > 1",
                expected: false,
            },
            EvalBoolean {
                input: "1 == 1",
                expected: true,
            },
            EvalBoolean {
                input: "1 != 1",
                expected: false,
            },
            EvalBoolean {
                input: "1 == 2",
                expected: false,
            },
            EvalBoolean {
                input: "1 != 2",
                expected: true,
            },
            EvalBoolean {
                input: "true == true",
                expected: true,
            },
            EvalBoolean {
                input: "false == false",
                expected: true,
            },
            EvalBoolean {
                input: "true == false",
                expected: false,
            },
            EvalBoolean {
                input: "true != false",
                expected: true,
            },
            EvalBoolean {
                input: "false != true",
                expected: true,
            },
            EvalBoolean {
                input: "(1 < 2) == true",
                expected: true,
            },
        ];

        for test in tests {
            let evaluated = test_eval(test.input);
            test_boolean_object(evaluated, test.expected);
        }
    }

    #[test]
    fn test_bang_operator() {
        let tests = vec![
            EvalBoolean {
                input: "!true",
                expected: false,
            },
            EvalBoolean {
                input: "!false",
                expected: true,
            },
            EvalBoolean {
                input: "!5",
                expected: false,
            },
            EvalBoolean {
                input: "!!true",
                expected: true,
            },
            EvalBoolean {
                input: "!!false",
                expected: false,
            },
            EvalBoolean {
                input: "!!5",
                expected: true,
            },
        ];

        for test in tests {
            let evaluated = test_eval(test.input);
            test_boolean_object(evaluated, test.expected);
        }
    }

    fn test_eval(input: &str) -> Box<dyn object::Object> {
        let lexer = lexer::Lexer::new(input);
        let mut parser = parser::Parser::new(lexer);
        let program = parser.parse_program().unwrap();

        eval(&program)
    }

    fn test_integer_object(obj: Box<dyn object::Object>, expected: i64) -> bool {
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

    fn test_boolean_object(obj: Box<dyn object::Object>, expected: bool) -> bool {
        let result = obj.as_any().downcast_ref::<object::Boolean>();

        assert!(result.is_some(), "object is not an boolean");

        let result = result.unwrap();

        assert_eq!(
            result.value, expected,
            "object has wrong value, got = {}, want = {}",
            result.value, expected
        );

        true
    }
}

use crate::{ast, object};

pub fn eval(node: &dyn ast::Node) -> Box<dyn object::Object> {
    if let Some(node) = node.as_any().downcast_ref::<ast::Program>() {
        return eval_program(node);
    } else if let Some(node) = node.as_any().downcast_ref::<ast::ExpressionStatement>() {
        return match &node.expression {
            Some(expr) => eval(expr.as_ref()),
            None => Box::new(object::Null {}),
        };
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
        if is_error(&right) {
            return right;
        }
        return eval_prefix_expression(&node.operator, right);
    } else if let Some(node) = node.as_any().downcast_ref::<ast::InfixExpression>() {
        let left = eval(
            node.left
                .as_ref()
                .expect("left expression not found")
                .as_ref(),
        );
        if is_error(&left) {
            return left;
        }

        let right = eval(
            node.right
                .as_ref()
                .expect("right expression not found")
                .as_ref(),
        );
        if is_error(&right) {
            return right;
        }
        return eval_infix_expression(&node.operator, left, right);
    } else if let Some(node) = node.as_any().downcast_ref::<ast::BlockStatement>() {
        return eva_block_statement(node);
    } else if let Some(node) = node.as_any().downcast_ref::<ast::IfExpression>() {
        return eval_if_expression(node);
    } else if let Some(node) = node.as_any().downcast_ref::<ast::ReturnStatement>() {
        let value = eval(node.return_value.as_ref().unwrap().as_ref());
        if is_error(&value) {
            return value;
        }
        return Box::new(object::ReturnValue { value });
    }

    Box::new(object::Null)
}

fn eval_program(program: &ast::Program) -> Box<dyn object::Object> {
    let mut result: Box<dyn object::Object> = Box::new(object::Null);

    for statements in &program.statements {
        result = eval(statements.as_ref());

        if result
            .as_any()
            .downcast_ref::<object::ReturnValue>()
            .is_some()
        {
            let return_value = result.into_any().downcast::<object::ReturnValue>().unwrap();
            return return_value.value;
        } else if result.as_any().downcast_ref::<object::Error>().is_some() {
            let return_value = result.into_any().downcast::<object::Error>().unwrap();
            return return_value;
        }
    }

    result
}

fn eva_block_statement(block: &ast::BlockStatement) -> Box<dyn object::Object> {
    let mut result: Box<dyn object::Object> = Box::new(object::Null);

    for stmt in &block.statements {
        result = eval(stmt.as_ref());

        if result
            .as_any()
            .downcast_ref::<object::ReturnValue>()
            .is_some()
        {
            let result_type = result.object_type();
            if result_type == object::RETURN_VALUE_OBJ || result_type == object::ERROR_OBJ {
                return result;
            }
        }
    }

    result
}

fn eval_if_expression(node: &ast::IfExpression) -> Box<dyn object::Object> {
    let condition = eval(node.condition.as_ref().unwrap().as_ref());
    if is_error(&condition) {
        return condition;
    }

    if is_truthy(condition) {
        eval(node.consequence.as_ref().unwrap())
    } else if node.alternative.is_some() {
        eval(node.alternative.as_ref().unwrap())
    } else {
        Box::new(object::Null {})
    }
}

fn is_truthy(obj: Box<dyn object::Object>) -> bool {
    if let Some(obj) = obj.as_any().downcast_ref::<object::Boolean>() {
        obj.value
    } else {
        obj.as_any().downcast_ref::<object::Null>().is_none()
    }
}

fn eval_prefix_expression(
    operator: &str,
    right: Box<dyn object::Object>,
) -> Box<dyn object::Object> {
    match operator {
        "!" => eval_bang_operator_expression(right),
        "-" => eval_minus_prefix_operator_expression(right),
        _other => new_error(format!(
            "unknown operator: {}{}",
            operator,
            right.object_type()
        )),
    }
}

fn eval_infix_expression(
    operator: &str,
    left: Box<dyn object::Object>,
    right: Box<dyn object::Object>,
) -> Box<dyn object::Object> {
    if left.object_type() == object::INTEGER_OBJ && right.object_type() == object::INTEGER_OBJ {
        return eval_integer_infix_expression(operator, left, right);
    }

    if left.object_type() != right.object_type() {
        return new_error(format!(
            "type mismatch: {} {} {}",
            left.object_type(),
            operator,
            right.object_type()
        ));
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
    } else {
        return new_error(format!(
            "unknown operator: {} {} {}",
            left.object_type(),
            operator,
            right.object_type()
        ));
    }
}

fn eval_integer_infix_expression(
    operator: &str,
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
        _other => new_error(format!(
            "unknown operator: {} {} {}",
            left.object_type(),
            operator,
            right.object_type()
        )),
    }
}

fn eval_minus_prefix_operator_expression(
    right: Box<dyn object::Object>,
) -> Box<dyn object::Object> {
    if right.object_type() != object::INTEGER_OBJ {
        return new_error(format!("unknown operator: -{}", right.object_type()));
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

fn new_error(message: String) -> Box<object::Error> {
    Box::new(object::Error { message })
}

fn is_error(obj: &Box<dyn object::Object>) -> bool {
    obj.object_type() == object::ERROR_OBJ
}

#[cfg(test)]
mod tests {
    use std::{any::Any, vec};

    use crate::{
        evaluator::eval,
        lexer,
        object::{self},
        parser,
    };

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

    struct ExpectIfElse<'a> {
        input: &'a str,
        expected: &'a dyn Any,
    }

    #[test]
    fn test_if_else_expression() {
        let tests = vec![
            ExpectIfElse {
                input: "if (true) { 10 }",
                expected: &10i64,
            },
            ExpectIfElse {
                input: "if (false) { 10 }",
                expected: &object::Null {},
            },
            ExpectIfElse {
                input: "if (1) { 10 }",
                expected: &10i64,
            },
            ExpectIfElse {
                input: "if (1 < 2) { 10 }",
                expected: &10i64,
            },
            ExpectIfElse {
                input: "if (1 > 2) { 10 }",
                expected: &object::Null {},
            },
            ExpectIfElse {
                input: "if (1 > 2) { 10 } else { 20 }",
                expected: &20,
            },
            ExpectIfElse {
                input: "if (1 < 2) { 10 } else { 20 }",
                expected: &10i64,
            },
        ];

        for test in tests {
            let evaluated = test_eval(test.input);
            let is_integer = test.expected.downcast_ref::<i64>();

            if let Some(int) = is_integer {
                test_integer_object(evaluated, *int);
            } else {
                test_null_object(evaluated);
            }
        }
    }

    #[test]
    fn test_return_statements() {
        let tests = vec![
            EvalInteger {
                input: "return 10;",
                expected: 10,
            },
            EvalInteger {
                input: "return 10; 9;",
                expected: 10,
            },
            EvalInteger {
                input: "return 2 * 5; 9;",
                expected: 10,
            },
            EvalInteger {
                input: "9; return 2 * 5; 9;",
                expected: 10,
            },
            EvalInteger {
                input: "if (10 > 1) {
                    if (10 > 1) {
                        return 10;
                    }
                    return 1;
                }
            ",
                expected: 10,
            },
        ];

        for test in tests {
            let evaluated = test_eval(test.input);
            test_integer_object(evaluated, test.expected);
        }
    }

    struct ExpectError<'a> {
        input: &'a str,
        expected_message: &'a str,
    }

    #[test]
    fn test_error_handling() {
        let tests = vec![
            ExpectError {
                input: "5 + true;",
                expected_message: "type mismatch: INTEGER + BOOLEAN",
            },
            ExpectError {
                input: "5 + true; 5;",
                expected_message: "type mismatch: INTEGER + BOOLEAN",
            },
            ExpectError {
                input: "-true",
                expected_message: "unknown operator: -BOOLEAN",
            },
            ExpectError {
                input: "true + false;",
                expected_message: "unknown operator: BOOLEAN + BOOLEAN",
            },
            ExpectError {
                input: "5; true + false; 5;",
                expected_message: "unknown operator: BOOLEAN + BOOLEAN",
            },
            ExpectError {
                input: "if (10 > 1) { true + false; }",
                expected_message: "unknown operator: BOOLEAN + BOOLEAN",
            },
            ExpectError {
                input: "
                if (10 > 1) {
                    if (10 > 1) {
                        return true + false;
                    }

                    return 1;
                }
                    ",
                expected_message: "unknown operator: BOOLEAN + BOOLEAN",
            },
        ];

        for test in tests {
            let evaluated = test_eval(test.input);

            let err_obj = evaluated.as_any().downcast_ref::<object::Error>();
            assert!(err_obj.is_some(), "no error object returned");
            let err_obj = err_obj.unwrap();
            assert_eq!(
                err_obj.message, test.expected_message,
                "wrong error message, expected = {}, got = {}",
                test.expected_message, err_obj.message
            )
        }
    }

    fn test_null_object(obj: Box<dyn object::Object>) -> bool {
        if obj.object_type() != object::NULL_OBJ {
            eprintln!("object is not null, got = {}", obj.object_type());
            return false;
        }

        true
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
